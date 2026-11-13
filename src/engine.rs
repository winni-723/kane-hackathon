//! De-identification engine.
//!
//! Step 0: a pass-through. The text comes back unchanged so the
//! end-to-end path (browser -> Axum -> browser) can be verified first.

pub fn process(text: &str) -> String {
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_text_unchanged() {
        assert_eq!(process("hello test"), "hello test");
    }

    #[test]
    fn keeps_line_breaks_and_empty_input() {
        assert_eq!(process("line one\nline two"), "line one\nline two");
        assert_eq!(process(""), "");
    }
}
