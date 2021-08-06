#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

use lib::start;

fn main() {
    start();
}