mod config;
mod format;
mod api;
mod util;
#[macro_use]
extern crate lazy_static;
pub use crate::config::config::CONFIG;

fn main() {
    let download_path = api::download_yuque::download();
    format::format::format(&download_path);
}
