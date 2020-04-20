use std::thread;


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
            _ => panic!("unhandled transition") // remove this in the end
        }
    }
}

// todo
// state transitions
// put it into a mutex/arc, call from two threads

fn main() {

    let mut state = State::new();
    println!("state = {:?}", state);

    state.advance_handler(Message::Stay{name: String::from("bla")});
    println!("state = {:?}", state);
    state.do_something(4);

    state.advance_handler(Message::Change);
    println!("state = {:?}", state);
    state.do_something(4);

    state.advance_handler(Message::Change);
    println!("state = {:?}", state);
    state.do_something(4);



    // let mut fancy = HandlerWrapper::Fancy(FancyHandler::new());
    // let mut simple = SimpleHandler::new();

    // let input = 3;
    // fancy.do_something(input);
    // simple.do_something(input);

}
