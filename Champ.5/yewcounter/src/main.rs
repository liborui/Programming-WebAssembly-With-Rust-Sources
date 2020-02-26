// The code in the main function is pretty standard for all Yew applications—
//     initialize the Yew runtime, 
//     create the context that’s appropriate for your application, 
//     then create an application for your model that takes your context as a parameter, 
//     and finally kick off the execution loop. 
extern crate yew;
extern crate yewcounter; // refers to lib.rs

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yewcounter::Model;

pub struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
