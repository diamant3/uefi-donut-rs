#![no_main]
#![no_std]

use core::time::Duration;
use uefi::{boot, entry, println, Status};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("Hello, World!");
    boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}
