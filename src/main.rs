#![warn(clippy::all, clippy::pedantic)]

mod xor;

use xor::{cipher, decipher};

fn main() {
    cipher();
    decipher();
}
