#![deny(unsafe_code)]
#![deny(warnings)]

pub mod greeting;

pub use greeting::{generate_greeting, GreetingError};