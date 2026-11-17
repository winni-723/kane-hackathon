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
    /// A PDF with pages but no text on them: a scan or a photo of paper.
    Scanned,
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentError::Unreadable(kind) => write!(f, "This file could not be read as {kind}."),
            DocumentError::TooLarge => write!(f, "This document is too large."),
            DocumentError::NoText => write!(f, "This document contains no text."),
            DocumentError::Scanned => write!(
                f,
                "This PDF has no text in it, only a picture of the page, as a scan does. \
                 Scanned documents cannot be read here."
            ),
        }
    }
}

/// The text of a PDF, page by page, top to bottom, as the PDF's own text layer
/// holds it.
///
/// A PDF has no words, paragraphs or tables, only characters placed on a page.
/// The text is rebuilt from where each character sits: characters at the same
/// height form a line; a small gap between two characters is a space, a wide
/// one (between table cells or form columns) is a tab, the separator a Word
/// table row gets. A long paragraph keeps the line breaks it was printed with.
/// A PDF with no text at all is a scan and is refused: the site does not read
/// pictures.
pub fn text_from_pdf(bytes: &[u8]) -> Result<String, DocumentError> {
    const KIND: &str = "a PDF";
    if !bytes.starts_with(b"%PDF-") {
        return Err(DocumentError::Unreadable(KIND));
    }
    // The PDF library can panic on a damaged file; a bad upload must give an
    // error message, not take the request down.
    let pages = std::panic::catch_unwind(|| -> Option<Vec<Vec<Glyph>>> {
        let mut doc = pdf_extract::Document::load_mem(bytes).ok()?;
        if doc.is_encrypted() {
            // Many PDFs are encrypted only to restrict printing or copying,
            // with an empty password. Others cannot be opened here.
            doc.decrypt("").ok()?;
        }
        let mut collector = GlyphCollector::default();
        pdf_extract::output_doc(&doc, &mut collector).ok()?;
        Some(collector.pages)
    })
    .ok()
    .flatten()
    .ok_or(DocumentError::Unreadable(KIND))?;

    let text = pages.into_iter().map(page_text).filter(|p| !p.is_empty()).collect::<Vec<_>>().join("\n\n");
    if text.len() as u64 > MAX_TEXT_BYTES {
        return Err(DocumentError::TooLarge);
    }
    if text.trim().is_empty() { Err(DocumentError::Scanned) } else { Ok(text) }
}

/// One character on a PDF page. `y` grows up the page, as PDF coordinates do.
#[derive(Clone, Debug)]
struct Glyph {
    x: f64,
    end: f64,
    y: f64,
    size: f64,
    text: String,
}

#[derive(Default)]
struct GlyphCollector {
    pages: Vec<Vec<Glyph>>,
}

impl pdf_extract::OutputDev for GlyphCollector {
    fn begin_page(
        &mut self,
        _page: u32,
        _media_box: &pdf_extract::MediaBox,
        _art_box: Option<(f64, f64, f64, f64)>,
    ) -> Result<(), pdf_extract::OutputError> {
        self.pages.push(Vec::new());
        Ok(())
    }
    fn end_page(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn output_character(
        &mut self,
        trm: &pdf_extract::Transform,
        width: f64,
        _spacing: f64,
        font_size: f64,
        text: &str,
    ) -> Result<(), pdf_extract::OutputError> {
        // The size the character is drawn at, after every scaling the page
        // applies: the side of a square with the area of the scaled em box.
        let sx = font_size * (trm.m11 + trm.m21);
        let sy = font_size * (trm.m12 + trm.m22);
        let size = (sx * sy).abs().sqrt();
        if let Some(page) = self.pages.last_mut()
            && size > 0.0
            && !text.is_empty()
        {
            page.push(Glyph {
                x: trm.m31,
                end: trm.m31 + width * size,
                y: trm.m32,
                size,
                text: text.to_string(),
            });
        }
        Ok(())
    }
    fn begin_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn end_word(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
    fn end_line(&mut self) -> Result<(), pdf_extract::OutputError> {
        Ok(())
    }
}

/// The text of one page: lines from the top down, each read left to right.
fn page_text(mut glyphs: Vec<Glyph>) -> String {
    // Top of the page first; within a height, left to right.
    glyphs.sort_by(|a, b| b.y.total_cmp(&a.y).then(a.x.total_cmp(&b.x)));

    // Characters whose baselines are within a third of their size of the
    // line's first character belong to that line.
    let mut lines: Vec<Vec<Glyph>> = Vec::new();
    for glyph in glyphs {
        match lines.last_mut() {
            Some(line) if (line[0].y - glyph.y).abs() <= line[0].size.min(glyph.size) / 3.0 => {
                line.push(glyph)
            }
            _ => lines.push(vec![glyph]),
        }
    }

    for line in &mut lines {
        line.sort_by(|a, b| a.x.total_cmp(&b.x));
    }
    let columns = column_edges(&lines);

    let rows: Vec<Row> = lines.iter().filter_map(|line| row(line, &columns)).collect();
    join_wrapped_cells(rows)
        .iter()
        .map(|r| r.cells.iter().map(|c| c.text.as_str()).collect::<Vec<_>>().join("\t"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// One printed line, split into cells at its tabs.
struct Row {
    /// Baseline of the line's first and last printed line (a row grows when
    /// a wrapped cell is joined to it).
    top: f64,
    bottom: f64,
    size: f64,
    cells: Vec<Cell>,
}

struct Cell {
    /// Where the cell's text starts on the page.
    x: f64,
    text: String,
}

fn row(line: &[Glyph], columns: &[f64]) -> Option<Row> {
    let breaks = line_breaks(line, columns);
    let mut cells: Vec<Cell> = Vec::new();
    for (glyph, brk) in line.iter().zip(breaks) {
        let blank = glyph.text.trim().is_empty();
        if brk == Break::Tab || cells.is_empty() {
            if blank {
                continue;
            }
            cells.push(Cell { x: glyph.x, text: String::new() });
        }
        let text = &mut cells.last_mut().expect("a cell was just opened").text;
        if brk == Break::Space && !text.ends_with(' ') {
            text.push(' ');
        }
        if !(blank && text.ends_with(' ')) {
            text.push_str(&glyph.text);
        }
    }
    for cell in &mut cells {
        cell.text = cell.text.trim_end().to_string();
    }
    cells.retain(|c| !c.text.is_empty());
    let first = line.first()?;
    (!cells.is_empty()).then_some(Row { top: first.y, bottom: first.y, size: first.size, cells })
}

/// Joins the second line of a wrapped table cell back onto its row.
///
/// When a table cell is too narrow for its text, the text wraps inside the
/// cell, and a name like "Dr. Ruth / Castellano" ends up on two printed lines.
/// Such a line is joined to the row above when every one of its cells starts
/// on the edge of a cell in that row, and it sits closer to the row than the
/// rows of the table sit to each other.
fn join_wrapped_cells(mut rows: Vec<Row>) -> Vec<Row> {
    // How far apart the rows around row `i` are: the larger of the gap from
    // the table row above it and the gap below the line after it.
    let spacing = |rows: &[Row], i: usize| -> Option<f64> {
        let above = (i > 0 && rows[i - 1].cells.len() >= 2).then(|| rows[i - 1].bottom - rows[i].top);
        let below = (i + 2 < rows.len() && rows[i + 2].cells.len() >= 2)
            .then(|| rows[i + 1].bottom - rows[i + 2].top);
        match (above, below) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (a, b) => a.or(b),
        }
    };

    let mut i = 0;
    while i < rows.len() {
        let joins = i + 1 < rows.len() && {
            let (row, next) = (&rows[i], &rows[i + 1]);
            let gap = row.bottom - next.top;
            row.cells.len() >= 2
                && next.cells.iter().all(|c| row.cells.iter().any(|r| (r.x - c.x).abs() <= EDGE_TOLERANCE))
                && spacing(&rows, i).is_some_and(|rows_apart| gap < rows_apart * 0.85)
                && gap < row.size * 2.0
        };
        if joins {
            let next = rows.remove(i + 1);
            let row = &mut rows[i];
            for cell in next.cells {
                if let Some(target) = row.cells.iter_mut().find(|r| (r.x - cell.x).abs() <= EDGE_TOLERANCE) {
                    target.text.push(' ');
                    target.text.push_str(&cell.text);
                }
            }
            row.bottom = next.bottom;
            // The row may wrap onto a third line too: look at it again.
            continue;
        }
        i += 1;
    }
    rows
}

#[derive(Clone, Copy, PartialEq)]
enum Break {
    None,
    Space,
    /// The next cell or column.
    Tab,
    /// A word that starts on a column edge, with no wide gap before it.
    Edge,
}

/// What comes before each character of a line.
fn line_breaks(line: &[Glyph], columns: &[f64]) -> Vec<Break> {
    let mut breaks = Vec::with_capacity(line.len());
    let mut last_end: Option<f64> = None;
    let mut after_blank = false;
    for glyph in line {
        let blank = glyph.text.trim().is_empty();
        let brk = match last_end {
            Some(end) if !blank => {
                let gap = glyph.x - end;
                let word_start = after_blank || gap > glyph.size * 0.2;
                if gap > glyph.size * 1.5 {
                    // A gap of more than one and a half letters.
                    Break::Tab
                } else if word_start && columns.iter().any(|c| (c - glyph.x).abs() <= EDGE_TOLERANCE) {
                    Break::Edge
                } else if gap > glyph.size * 0.2 {
                    Break::Space
                } else {
                    Break::None
                }
            }
            _ => Break::None,
        };
        breaks.push(brk);
        after_blank = blank;
        last_end = Some(glyph.end.max(last_end.unwrap_or(f64::MIN)));
    }
    // A table row breaks into cells more than once. A lone word that merely
    // sits on a column edge, in a heading or a paragraph, is not a new cell.
    let cells = breaks.iter().filter(|b| matches!(b, Break::Tab | Break::Edge)).count();
    for brk in &mut breaks {
        if *brk == Break::Edge {
            *brk = if cells >= 2 { Break::Tab } else { Break::Space };
        }
    }
    breaks
}

/// How far apart, in points, two word starts may be and still share an edge.
const EDGE_TOLERANCE: f64 = 0.6;

/// Left edges of table cells and form columns on a page: positions where a
/// word starts, inside a line, on at least three different lines, and where
/// on at least one of them a wide gap comes before it.
///
/// Word sets table cells side by side, so the text of a full cell can end
/// right where the next cell's text begins, with no wider gap than a space.
/// The cells' edges line up from row to row, and a shorter row (the header,
/// as a rule) shows the gap. Words in a paragraph, or units in a column of
/// numbers, may line up by chance but never have a wide gap before them.
fn column_edges(lines: &[Vec<Glyph>]) -> Vec<f64> {
    let mut starts: Vec<(f64, usize, bool)> = Vec::new();
    for (n, line) in lines.iter().enumerate() {
        let mut last_end = f64::MIN;
        let mut after_blank = false;
        for glyph in line {
            let blank = glyph.text.trim().is_empty();
            let gap = glyph.x - last_end;
            if last_end > f64::MIN && !blank && (after_blank || gap > glyph.size * 0.2) {
                starts.push((glyph.x, n, gap > glyph.size * 1.5));
            }
            after_blank = blank;
            last_end = last_end.max(glyph.end);
        }
    }
    starts.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut edges = Vec::new();
    let mut i = 0;
    while i < starts.len() {
        let mut j = i;
        while j + 1 < starts.len() && starts[j + 1].0 - starts[i].0 <= EDGE_TOLERANCE {
            j += 1;
        }
        let mut lines_here: Vec<usize> = starts[i..=j].iter().map(|s| s.1).collect();
        lines_here.sort_unstable();
        lines_here.dedup();
        let confirmed = starts[i..=j].iter().any(|s| s.2);
        if lines_here.len() >= 3 && confirmed {
            edges.push(starts[i].0);
        }
        i = j + 1;
    }
    edges
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

    #[test]
    fn rejects_bytes_that_are_not_a_pdf() {
        assert_eq!(text_from_pdf(b"just some text"), Err(DocumentError::Unreadable("a PDF")));
        assert_eq!(text_from_pdf(b"%PDF-1.7 but broken"), Err(DocumentError::Unreadable("a PDF")));
    }

    /// Lays out `words` on one line at height `y`, 10 points per letter, each
    /// word starting at its given x.
    fn placed(y: f64, words: &[(f64, &str)]) -> Vec<Glyph> {
        let mut out = Vec::new();
        for (x, word) in words {
            for (i, c) in word.chars().enumerate() {
                let x = x + 10.0 * i as f64;
                out.push(Glyph { x, end: x + 10.0, y, size: 20.0, text: c.to_string() });
            }
        }
        out
    }

    #[test]
    fn pdf_lines_from_top_to_bottom_with_spaces_and_tabs() {
        let mut glyphs = placed(100.0, &[(0.0, "low")]);
        // Out of order on purpose: a PDF may draw text in any order.
        glyphs.extend(placed(200.0, &[(0.0, "Name"), (300.0, "Ann"), (50.0, "x")]));
        // A gap of a third of a letter is a space; none is a word going on.
        glyphs.extend(placed(150.0, &[(0.0, "ab"), (27.0, "cd")]));
        assert_eq!(page_text(glyphs), "Name x\tAnn\nab cd\nlow");
    }

    #[test]
    fn pdf_cells_that_touch_are_split_on_shared_edges() {
        // A header row shows the gap before the second column at x=100;
        // in the rows below, the first cell runs right up to it.
        let lines = [
            placed(300.0, &[(0.0, "Drug"), (100.0, "Dose"), (200.0, "When")]),
            placed(280.0, &[(0.0, "Warfarin"), (85.0, " "), (100.0, "5mg"), (200.0, "daily")]),
            placed(260.0, &[(0.0, "Aspirin"), (75.0, " "), (100.0, "81mg"), (200.0, "daily")]),
        ];
        let text = page_text(lines.concat());
        assert_eq!(text, "Drug\tDose\tWhen\nWarfarin\t5mg\tdaily\nAspirin\t81mg\tdaily");
    }

    #[test]
    fn a_cell_that_wraps_is_joined_to_its_row() {
        // Rows 50 points apart; the prescriber's surname wraps 30 points
        // below its row, in the same cell.
        let lines = [
            placed(300.0, &[(0.0, "Drug"), (100.0, "Doctor"), (200.0, "Rx")]),
            placed(250.0, &[(0.0, "Warfarin"), (100.0, "Dr. Ruth"), (200.0, "6610490")]),
            placed(220.0, &[(100.0, "Castellano")]),
            placed(170.0, &[(0.0, "Aspirin"), (100.0, "Dr. Ng"), (200.0, "6608817")]),
        ];
        assert_eq!(
            page_text(lines.concat()),
            "Drug\tDoctor\tRx\nWarfarin\tDr. Ruth Castellano\t6610490\nAspirin\tDr. Ng\t6608817"
        );
    }

    #[test]
    fn evenly_spaced_rows_are_not_joined() {
        // A short row sitting on the table's edges is still a row of its own
        // when it is as far from the row above as the other rows are.
        let lines = [
            placed(300.0, &[(0.0, "Test"), (100.0, "Result"), (200.0, "Flag")]),
            placed(270.0, &[(0.0, "Glucose"), (100.0, "168"), (200.0, "H")]),
            placed(240.0, &[(0.0, "Sodium"), (100.0, "139")]),
            placed(210.0, &[(0.0, "ALT"), (100.0, "31"), (200.0, "N")]),
        ];
        assert_eq!(
            page_text(lines.concat()),
            "Test\tResult\tFlag\nGlucose\t168\tH\nSodium\t139\nALT\t31\tN"
        );
    }

    #[test]
    fn a_word_alone_on_a_column_edge_is_not_a_cell() {
        let lines = [
            placed(300.0, &[(0.0, "Drug"), (100.0, "Dose"), (200.0, "When")]),
            placed(280.0, &[(0.0, "Warfarin"), (100.0, "5mg"), (200.0, "daily")]),
            placed(260.0, &[(0.0, "Aspirin"), (100.0, "81mg"), (200.0, "daily")]),
            // A sentence below the table whose third word happens to start at x=100.
            placed(240.0, &[(0.0, "Take"), (45.0, "all"), (80.0, " "), (100.0, "doses")]),
        ];
        assert!(page_text(lines.concat()).ends_with("\nTake all doses"));
    }
}
