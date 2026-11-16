//! Prints the text read from a test document, then its de-identified text.
//! Usage: cargo run --example show_document -- docx-03
use deid_server::{document, engine};
use std::fs;

fn main() {
    let id = std::env::args().nth(1).expect("usage: show_document <id>");
    let bytes = fs::read(format!("tests/documents/{id}.docx")).expect("document exists");
    let text = document::text_from_docx(&bytes).expect("document is readable");
    println!("{}", engine::process(&text));
}
