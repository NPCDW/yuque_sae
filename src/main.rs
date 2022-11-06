mod config;
mod api;
#[macro_use]
extern crate lazy_static;
pub use crate::config::config::CONFIG;

fn main() {
    println!("Get Config: {:?}", *CONFIG);
    println!("Hello, world!");
}
