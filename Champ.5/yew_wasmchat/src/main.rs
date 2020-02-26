extern crate wasmchat;
extern crate web_logger;
extern crate yew;

use wasmchat::{services::PubnubService, Model};
use yew::prelude::*;

pub struct Context {
    pubnub: PubnubService,
}

impl AsMut<PubnubService> for Context {
    fn as_mut(&mut self) -> &mut PubnubService {
        &mut self.pubnub
    }
}

fn main() {
    web_logger::init();
    yew::initialize();

    let context = Context {
        pubnub: PubnubService::new("(your publish key)",
                                   "(your subscribe key)"),
    };

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}