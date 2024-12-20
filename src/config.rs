//! This module defines the configuration structure and logic for the Age plugin.
//!
//! # Configuration Fields

use nvim_oxi::String;
use nvim_oxi::{conversion::FromObject, Dictionary};

#[derive(Debug, Default)]
pub struct Config {
    pub public_key: String,
    pub private_key: String,
    pub encrypt_and_del: bool,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        Config {
            public_key: options
                .get("public_key")
                .and_then(|public_key_obj| String::from_object(public_key_obj.clone()).ok())
                .unwrap_or_else(|| "".into()),

            private_key: options
                .get("private_key")
                .and_then(|private_key_obj| String::from_object(private_key_obj.clone()).ok())
                .unwrap_or_else(|| "".into()),

            encrypt_and_del: options
                .get("encrypt_and_del")
                .and_then(|encrypt_and_del| bool::from_object(encrypt_and_del.clone()).ok())
                .unwrap_or(false),
        }
    }
}
