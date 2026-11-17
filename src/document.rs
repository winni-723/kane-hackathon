//! Turns an uploaded document into the plain text the engine works on.
//!
//! Nothing here decides what is an identifier. A document is read into text,
//! in reading order, and the text goes through `engine::process` like a pasted
//! record. Page headers and footers are included, because in real records
//! they repeat the patient's name and record number on every page.

use std::fmt;
use std::io::{Cursor, Read};

/// The most text one document may expand to. A .docx file is a zip archive, so
/// a small upload can claim to hold far more than it should.
const MAX_TEXT_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub enum DocumentError {
    /// The bytes are not the kind of file the name says they are.
    Unreadable(&'static str),
    /// The document expands to more text than the limit.
    TooLarge,
    /// The document was read but holds no text.
    NoText,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentError::Unreadable(kind) => write!(f, "This file could not be read as {kind}."),
            DocumentError::TooLarge => write!(f, "This document is too large."),
            DocumentError::NoText => write!(f, "This document contains no text."),
        }
    }
}

/// The text of a Word document: page headers, then the body, footnotes,
/// endnotes and comments, then page footers.
///
/// Paragraphs end in a newline. A table row becomes one line with its cells
/// separated by tabs, so a label and its value stay together.
pub fn text_from_docx(bytes: &[u8]) -> Result<String, DocumentError> {
    const KIND: &str = "a Word document";
    let mut archive =
        zip::ZipArchive::new(Cursor::new(bytes)).map_err(|_| DocumentError::Unreadable(KIND))?;

    let mut names: Vec<String> = archive
        .file_names()
        .filter_map(|name| name.ok())
        .map(|name| name.into_owned())
        .collect();
    names.sort();
    let numbered = |prefix: &str| -> Vec<String> {
        names
            .iter()
            .filter(|n| n.starts_with(prefix) && n.ends_with(".xml"))
            .cloned()
            .collect()
    };
    if !names.iter().any(|n| n == "word/document.xml") {
        return Err(DocumentError::Unreadable(KIND));
    }

    let mut parts = numbered("word/header");
    parts.push("word/document.xml".to_string());
    for optional in ["word/footnotes.xml", "word/endnotes.xml", "word/comments.xml"] {
        if names.iter().any(|n| n == optional) {
            parts.push(optional.to_string());
        }
    }
    parts.extend(numbered("word/footer"));

    let mut text = String::new();
    let mut budget = MAX_TEXT_BYTES;
    for name in parts {
        let file = archive.by_name(&name).map_err(|_| DocumentError::Unreadable(KIND))?;
        let mut xml = String::new();
        file.take(budget + 1)
            .read_to_string(&mut xml)
            .map_err(|_| DocumentError::Unreadable(KIND))?;
        if xml.len() as u64 > budget {
            return Err(DocumentError::TooLarge);
        }
        budget -= xml.len() as u64;
        let part = text_from_word_xml(&xml);
        if !part.is_empty() {
            text.push_str(&part);
            text.push('\n');
        }
    }

    let text = tidy(&text);
    if text.is_empty() { Err(DocumentError::NoText) } else { Ok(text) }
}

/// The most rows one CSV file may hold.
pub const MAX_CSV_ROWS: usize = 1000;

/// The records in a CSV export: one per row, each written as the labelled
/// lines of a form.
///
/// ```text
/// LastName,FirstName,DOB          Last Name: Okonkwo
/// Okonkwo,Chidi,04/12/1968   ->   First Name: Chidi
///                                 DOB: 04/12/1968
/// ```
///
/// The column header becomes the label, so the engine's field rules know what
/// each value is even when its shape says nothing (`19900602` under
/// `Birth Date`). Values are passed on exactly as written; empty cells are
/// left out.
pub fn records_from_csv(bytes: &[u8]) -> Result<Vec<String>, DocumentError> {
    const KIND: &str = "a CSV table";
    let text = String::from_utf8_lossy(bytes);
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    if text.contains('\0') {
        return Err(DocumentError::Unreadable(KIND));
    }

    let mut rows = parse_csv(text).into_iter();
    let header: Vec<String> = rows.next().unwrap_or_default().iter().map(|h| label(h)).collect();
    if header.len() < 2 || header.iter().any(String::is_empty) {
        return Err(DocumentError::Unreadable(KIND));
    }

    let mut records = Vec::new();
    for row in rows {
        let mut record = String::new();
        for (label, value) in header.iter().zip(&row) {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            record.push_str(&format!("{label}: {value}\n"));
        }
        if !record.is_empty() {
            records.push(record.trim_end().to_string());
        }
        if records.len() > MAX_CSV_ROWS {
            return Err(DocumentError::TooLarge);
        }
    }
    if records.is_empty() { Err(DocumentError::NoText) } else { Ok(records) }
}

/// Splits CSV text into rows of cells. Handles quoted cells, doubled quotes,
/// line breaks inside quotes, and `,`, `;` or tab as the separator (whichever
/// the header row uses most).
fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let first_line = text.lines().next().unwrap_or("");
    let separator = [',', ';', '\t']
        .into_iter()
        .max_by_key(|sep| first_line.matches(*sep).count())
        .unwrap_or(',');

    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut cell = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    cell.push('"');
                }
                '"' => quoted = false,
                '\r' => {}
                _ => cell.push(c),
            }
        } else if c == '"' && cell.is_empty() {
            quoted = true;
        } else if c == separator {
            row.push(std::mem::take(&mut cell));
        } else if c == '\n' {
            row.push(std::mem::take(&mut cell));
            rows.push(std::mem::take(&mut row));
        } else if c != '\r' {
            cell.push(c);
        }
    }
    if !cell.is_empty() || !row.is_empty() {
        row.push(cell);
        rows.push(row);
    }
    rows.retain(|row| row.iter().any(|cell| !cell.trim().is_empty()));
    rows
}

/// A column header as a form label: `LastName` -> `Last Name`,
/// `birth_date` -> `birth date`, `PatientDOB` -> `Patient DOB`.
fn label(header: &str) -> String {
    let mut out = String::new();
    let mut previous: Option<char> = None;
    for c in header.trim().chars() {
        let c = if matches!(c, '_' | ':' | '\n' | '\t') { ' ' } else { c };
        if c.is_uppercase() && previous.is_some_and(|p| p.is_lowercase()) {
            out.push(' ');
        }
        out.push(c);
        previous = Some(c);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Reads the visible text out of one WordprocessingML part.
fn text_from_word_xml(xml: &str) -> String {
    let mut out = String::new();
    let mut in_text = false;
    let mut cell_depth = 0usize;
    let mut rest = xml;

    while let Some(open) = rest.find('<') {
        if in_text {
            push_unescaped(&mut out, &rest[..open]);
        }
        let Some(close) = rest[open..].find('>') else { break };
        let tag = &rest[open + 1..open + close];
        rest = &rest[open + close + 1..];

        let closing = tag.starts_with('/');
        let self_closing = tag.ends_with('/');
        let name = tag
            .trim_start_matches('/')
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()
            .unwrap_or("");

        match (name, closing) {
            // Only w:t holds text that is shown. Deleted text (w:delText) and
            // field codes (w:instrText) are skipped.
            ("w:t", false) => in_text = !self_closing,
            ("w:t", true) => in_text = false,
            ("w:tab", false) => out.push('\t'),
            ("w:br" | "w:cr", false) => out.push('\n'),
            ("w:tc", false) if !self_closing => cell_depth += 1,
            ("w:tc", true) => {
                cell_depth = cell_depth.saturating_sub(1);
                trim_end_spaces(&mut out);
                out.push('\t');
            }
            ("w:tr", true) => {
                while out.ends_with('\t') || out.ends_with(' ') {
                    out.pop();
                }
                out.push('\n');
            }
            // A paragraph inside a table cell does not end the row.
            ("w:p", true) => out.push(if cell_depth > 0 { ' ' } else { '\n' }),
            _ => {}
        }
    }
    out
}

fn trim_end_spaces(out: &mut String) {
    while out.ends_with(' ') {
        out.pop();
    }
}

fn push_unescaped(out: &mut String, raw: &str) {
    let mut rest = raw;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let Some(end) = rest.find(';').filter(|&end| end <= 10) else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let entity = &rest[1..end];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => entity
                .strip_prefix("#x")
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .or_else(|| entity.strip_prefix('#').and_then(|dec| dec.parse().ok()))
                .and_then(char::from_u32),
        };
        match decoded {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
}

/// Trims each line and drops runs of blank lines.
fn tidy(text: &str) -> String {
    let mut out = String::new();
    let mut blank = true;
    for line in text.lines() {
        let line = line.trim_end();
        if line.trim().is_empty() {
            if !blank {
                out.push('\n');
            }
            blank = true;
        } else {
            out.push_str(line);
            out.push('\n');
            blank = false;
        }
    }
    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: &str = r#"xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main""#;

    #[test]
    fn paragraphs_become_lines() {
        let xml = format!(
            "<w:document {W}><w:body><w:p><w:r><w:t>First</w:t></w:r></w:p>\
             <w:p><w:r><w:t xml:space=\"preserve\">Second </w:t></w:r><w:r><w:t>half</w:t></w:r></w:p></w:body></w:document>"
        );
        assert_eq!(tidy(&text_from_word_xml(&xml)), "First\nSecond half");
    }

    #[test]
    fn a_table_row_is_one_line_with_tabs() {
        let xml = "<w:tbl><w:tr><w:tc><w:p><w:r><w:t>DOB</w:t></w:r></w:p></w:tc>\
                   <w:tc><w:p><w:r><w:t>03/07/1948</w:t></w:r></w:p></w:tc></w:tr>\
                   <w:tr><w:tc><w:p><w:r><w:t>Sex</w:t></w:r></w:p></w:tc>\
                   <w:tc><w:p><w:r><w:t>Female</w:t></w:r></w:p></w:tc></w:tr></w:tbl>";
        assert_eq!(tidy(&text_from_word_xml(xml)), "DOB\t03/07/1948\nSex\tFemale");
    }

    #[test]
    fn deleted_text_and_field_codes_are_not_text() {
        let xml = "<w:p><w:r><w:delText>Old Name</w:delText></w:r>\
                   <w:r><w:instrText> PAGE </w:instrText></w:r>\
                   <w:r><w:t>Page </w:t></w:r><w:r><w:t>1</w:t></w:r></w:p>";
        assert_eq!(tidy(&text_from_word_xml(xml)), "Page 1");
    }

    #[test]
    fn entities_tabs_and_breaks() {
        let xml = "<w:p><w:r><w:t>Smith &amp; Jones &lt;5 &#233;</w:t><w:tab/><w:t>x</w:t><w:br/><w:t>y</w:t></w:r></w:p>";
        assert_eq!(tidy(&text_from_word_xml(xml)), "Smith & Jones <5 é\tx\ny");
    }

    #[test]
    fn a_csv_row_becomes_a_labelled_record() {
        let csv = "MRN,LastName,FirstName,DOB,PrimaryDx\r\n40071823,Okonkwo,Chidi,04/12/1968,Asthma\r\n";
        assert_eq!(
            records_from_csv(csv.as_bytes()),
            Ok(vec![
                "MRN: 40071823\nLast Name: Okonkwo\nFirst Name: Chidi\nDOB: 04/12/1968\nPrimary Dx: Asthma"
                    .to_string()
            ])
        );
    }

    #[test]
    fn csv_quotes_commas_line_breaks_and_empty_cells() {
        let csv = "\u{feff}\"Patient Name\",\"Email\",\"Visit Note\"\r\n\
                   \"Hollis, Marguerite\",\"\",\"He said, \"\"call Irina\"\".\nSecond line.\"\r\n\
                   \"\",\"\",\"\"\r\n";
        assert_eq!(
            records_from_csv(csv.as_bytes()),
            Ok(vec![
                "Patient Name: Hollis, Marguerite\nVisit Note: He said, \"call Irina\".\nSecond line."
                    .to_string()
            ])
        );
    }

    #[test]
    fn csv_with_semicolons_and_a_compact_birth_date() {
        let csv = "name;birth_date;weight\nA B;19900602;19900602\n";
        assert_eq!(
            records_from_csv(csv.as_bytes()),
            Ok(vec!["name: A B\nbirth date: 19900602\nweight: 19900602".to_string()])
        );
    }

    #[test]
    fn rejects_text_that_is_not_a_table() {
        assert_eq!(records_from_csv(b"just one line"), Err(DocumentError::Unreadable("a CSV table")));
        assert_eq!(records_from_csv(b"a,b\n"), Err(DocumentError::NoText));
    }

    #[test]
    fn rejects_bytes_that_are_not_a_word_document() {
        assert_eq!(
            text_from_docx(b"just some text"),
            Err(DocumentError::Unreadable("a Word document"))
        );
    }
}
