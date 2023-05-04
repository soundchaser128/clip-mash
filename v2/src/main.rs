use color_eyre::Report;

mod service;
mod server;
mod data;
mod util;

pub type Result<T> = std::result::Result<T, Report>;

fn main() {
    println!("Hello, world!");
}
