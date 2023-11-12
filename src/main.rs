use hydrigo::prelude::*;

fn main() {
    let state = State::new("hello", &updater::print);

    state.set("world");
    state.set("hello world");
}
