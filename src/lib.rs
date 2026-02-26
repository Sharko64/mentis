#![deny(unsafe_code)]
#![deny(warnings)]

pub mod core {
    pub mod greeting;
}

pub use core::greeting::{generate_greeting, GreetingError};