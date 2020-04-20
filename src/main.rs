use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;


trait ActionHandler {
    fn do_something(&mut self, input: i32, shared: &mut SharedState);
}

#[derive(PartialEq, Clone, Debug)]
struct FancyHandler {
    value: i32
}


impl FancyHandler {
    pub fn new() -> FancyHandler {
        FancyHandler {
            value: 5,
        }
    }
}

impl ActionHandler for FancyHandler {
    fn do_something(&mut self, input: i32, shared: &mut SharedState) {
        shared.z += 1;
        println!("fancy calculated {}", self.value * input * 3);
    }
}

#[derive(PartialEq, Clone, Debug)]
struct SimpleHandler {
    x: i32,
    y: i32,
}

impl SimpleHandler {
    pub fn new() -> SimpleHandler {
        SimpleHandler {
            x: 2,
            y: 3,
        }
    }
}

impl ActionHandler for SimpleHandler {
    fn do_something(&mut self, input: i32, shared: &mut SharedState) {
        println!("simple calculated {}", self.x + self.y + input + shared.z);
    }
}

#[derive(PartialEq, Clone, Debug)]
enum HandlerWrapper {
    Fancy(FancyHandler),
    Simple(SimpleHandler),
}


impl ActionHandler for HandlerWrapper {
    fn do_something(&mut self, input: i32, shared: &mut SharedState) {
        match self {
            HandlerWrapper::Fancy(inner_handler) => inner_handler.do_something(input, shared),
            HandlerWrapper::Simple(inner_handler) => inner_handler.do_something(input, shared),
        }
    }
}

#[derive(Debug)]
struct SharedState {
    z: i32,
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            z: 5,
        }
    }
}

enum Message {
    Stay {name: String},
    Change,
}

#[derive(Debug)]
struct State {
    handler: HandlerWrapper,
    shared: SharedState,
}

impl State {
    pub fn new() -> State {
        State {
            handler: HandlerWrapper::Simple(SimpleHandler::new()),
            shared: SharedState::new(),
        }
    }

    pub fn do_something(&mut self, input: i32) {
        self.handler.do_something(input, &mut self.shared);
    }

    pub fn advance_handler(&mut self, message: Message) {
        match (self.handler.clone(), message) {
        
            (HandlerWrapper::Fancy(_), Message::Stay{name: _})  => self.handler = HandlerWrapper::Fancy(FancyHandler::new()),
            (HandlerWrapper::Fancy(_), Message::Change)  => self.handler = HandlerWrapper::Simple(SimpleHandler::new()),
            (HandlerWrapper::Simple(_), Message::Stay{name: _})  => self.handler = HandlerWrapper::Simple(SimpleHandler::new()),
            (HandlerWrapper::Simple(_), Message::Change)  => self.handler = HandlerWrapper::Fancy(FancyHandler::new()),
            // _ => panic!("unhandled transition") // remove this in the end
        }
    }
}

fn main() {

    let mutex = Arc::new(Mutex::new(State::new()));

    let data_change = Arc::clone(&mutex);
    let data_print = Arc::clone(&mutex);

    let change_handle = thread::spawn(move || {
        
        loop {
            {
                let mut data = data_change.lock().unwrap();
                data.advance_handler(Message::Change);
            }
            thread::sleep(Duration::from_millis(2000));
        }
    });

    let print_handle = thread::spawn(move || {
        loop {
            {
                let mut data = data_print.lock().unwrap();
                data.do_something(5);
                println!("{:?}", data);
            }
            thread::sleep(Duration::from_millis(500));
        }
    });


    let _ = print_handle.join();
    let _ = change_handle.join();


}
