//! Prints the de-identified text of one fixture, for reading the output by eye.
//! Usage: cargo run --example show -- hard-05
use deid_server::engine;
use std::fs;

fn main() {
    let id = std::env::args().nth(1).expect("usage: show <record_id>");
    let raw = fs::read_to_string(format!("tests/fixtures/{id}.json")).expect("fixture exists");
    let fixture: serde_json::Value = serde_json::from_str(&raw).expect("fixture is valid JSON");
    let text = fixture["text"].as_str().expect("fixture has text");
    println!("{}", engine::process(text));
}
