use super::Message;
use stdweb::Value;
use yew::prelude::*;

pub struct PubnubService {
    lib: Option<Value>,
    chat: Option<Value>,
}

impl PubnubService {
    pub fn new(publish_key: &str, subscribe_key: &str) -> Self {
        info!("Creating new instance of pubnub chatengine service");
        let chat_engine = js! { // (1) Inject JS to instantiate Pubnub's ChatEnginecore
            let ce = ChatEngineCore.create({ //Everything inside the js! block is wrapped in a private scope and converted into a JavaScript function that shows up in the generated JavaScript file.
                publishKey: @{publish_key},
                subscribeKey: @{subscribe_key}
            });
            console.log("Chat engine core created");
            return ce;
        };
        PubnubService { // (2) Return a new PubnubService with a captured reference to the chat engine JavaScript object.
            lib: Some(chat_engine),
            chat: None,
        }
    }

    pub fn send_message(&mut self, msg: &str) -> () {
        js! {
            let m = @{msg};
            myChat.emit("message", {
                text: m
            });
        }
    }





    pub fn connect(
        &mut self,
        topic: &str,
        nickname: &str,
        onmessage: Callback<Message>,
        onoffline: Callback<String>,
        ononline: Callback<String>,
    ) -> () {
        let lib = self.lib.as_ref().expect("No pubnub library!");

        let chat_callback = move |text: String, source: String| { // (3) Define a set of callback functions that will be invoked by various Chat Engine handlers.
            let msg = Message {
                text: text,
                from: source,
            };
            onmessage.emit(msg);
        };

        let useroffline_callback = move |username: String| {
            onoffline.emit(username);
        };

        let useronline_callback = move |username: String| {
            ononline.emit(username);
        };

        let chat = js! {  // (4) Inject the JavaScript necessary to attach the Rust/wasm callbacks to chat engine JavaScript handlers.
            var pn = @{lib};
            var chat_callback = @{chat_callback};
            var online_cb = @{useronline_callback};
            var offline_cb = @{useroffline_callback};

            pn.on("$.ready", function(data) {
                console.log("PubNub Chat Engine Ready");
                // set global variable
                me = data.me;
                // create a new ChatEngine Chat (global var)
                myChat = new pn.Chat(@{topic});

                myChat.on("$.connected", () => {
                    console.log("The chat is connected!");

                    myChat.on("message", (message) => {
                        chat_callback(message.data.text,
                                      message.sender.state.nickName);
                        console.log("message: " + message.data.text +
                            " from " + message.sender.state.nickName);
                    });
                    myChat.on("$.online.*", (data) => {
                        console.log("User is Online: ", data.user);
                        online_cb(data.user.state.nickName);
                    });
                    myChat.on("$.offline.*", (data) => {
                        console.log("User is Offline: ", data.user);
                        offline_cb(data.user.state.nickName);
                    });
                });

            });
            pn.connect(String(new Date().getTime()), {
                nickName: @{nickname}
            });

            console.log("pubnub connecting");
            return myChat;
        };
        self.chat = Some(chat);
    }
}