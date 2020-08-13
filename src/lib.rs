#![feature(with_options)]

use rand::prelude::*;
use rocket::http::RawStr;
use rocket::request::FromParam;
use std::fmt::{self, Display};
use std::fs;
use std::io::prelude::*;

const CHARS: &[u8] = b"1234567890QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm";

#[derive(Debug, Clone, PartialEq)]
pub struct PasteID(String);

impl PasteID {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        if fs::read_dir("uploads").is_err() {
            fs::create_dir("uploads").expect("failed to crate directory to store uploads")
        }

        let mut file = fs::File::with_options()
            .create(true)
            .read(true)
            .append(true)
            .open("uploads/_index.txt")
            .unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let entries: Vec<_> = content.lines().collect();

        let id = loop {
            let utf8: Vec<_> = CHARS.choose_multiple(&mut rng, 16).copied().collect();
            let id = String::from_utf8(utf8).unwrap();

            if !entries.contains(&&*id) {
                writeln!(file, "{}", id).unwrap();
                break id;
            }
        };

        Self(id)
    }
}

impl Default for PasteID {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PasteID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromParam<'_> for PasteID {
    type Error = FromParamError;

    fn from_param(param: &RawStr) -> Result<Self, Self::Error> {
        if param.trim().len() != 16 {
            Err(FromParamError::new("length of parameter is not correct"))
        } else {
            for c in param.trim().as_bytes() {
                if !CHARS.contains(c) {
                    return Err(FromParamError::new("length of parameter is not correct"));
                }
            }

            Ok(Self(param.to_string()))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FromParamError {
    reason: &'static str,
}

impl FromParamError {
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SizeError;
