#![allow(proc_macro_derive_resolution_fallback)]

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub gender: String,
    pub year: i32,
    pub price: i32,
}
