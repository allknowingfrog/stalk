use std::env;
use std::net::TcpListener;

use stalk::Config;
use stalk::Stalk;

fn main() {
    let config = Config::new(env::args());

    let listener = TcpListener::bind(config.address()).unwrap();

    let mut stalk = Stalk::new();

    stalk.run(listener);
}
