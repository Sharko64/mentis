use std::env;

use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum GreetingError {
    #[error("Invalid name provided")]
    InvalidName,
}

pub fn generate_greeting(name: Option<&str>) -> Result<String, GreetingError> {
    let final_name = match name {
        Some(n) if !n.trim().is_empty() => n.trim(),
        Some(_) => return Err(GreetingError::InvalidName),
        None => {
            env::var("HELLO_TRUNK_NAME")
                .unwrap_or_else(|_| "World".to_string())
                .trim()
                .to_string()
        }
    };

    debug!(name = %final_name, "Generating greeting");

    Ok(format!("Hello, {}!", final_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_greeting_with_name() {
        let result = generate_greeting(Some("Alice")).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_generate_greeting_empty_name() {
        let result = generate_greeting(Some(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_greeting_default() {
        std::env::remove_var("HELLO_TRUNK_NAME");
        let result = generate_greeting(None).unwrap();
        assert_eq!(result, "Hello, World!");
    }
}