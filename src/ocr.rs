//! Reads a scanned PDF: the picture of each page goes through OCR, and the
//! text it gives goes through `engine::process` like any other record.
//!
//! The OCR models are ocrs (https://github.com/robertknight/ocrs), run here in
//! Rust and, built with `--features wasm`, in the visitor's browser. The model
//! files are not compiled in; the caller loads them (the page fetches them
//! from its own site, the tests read them from `web/public/models/`).
//!
//! `document::text_from_pdf` still refuses a scan. Reading one is a separate,
//! deliberate step, because OCR can misread: the page offers it as a choice.

use std::sync::LazyLock;

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use pdf_extract::{Dictionary, Document, Object, Stream};
use regex::Regex;

use crate::document::DocumentError;

/// A picture smaller than this on either side is a logo or a stamp, not a page.
const MIN_SIDE: i64 = 300;
/// The largest page picture read, in pixels (a letter page at 600 dpi).
const MAX_PIXELS: i64 = 5100 * 6600;
/// The most pages read from one scan.
const MAX_PAGES: usize = 20;

/// The OCR engine with its two models loaded. Loading takes a moment, so one
/// reader is kept and used for every scan.
pub struct Reader {
    engine: OcrEngine,
}

impl Reader {
    /// Loads the text detection and text recognition models (`.onnx` bytes).
    pub fn new(detection_model: Vec<u8>, recognition_model: Vec<u8>) -> Result<Reader, String> {
        let detection_model = rten::Model::load(detection_model).map_err(|e| e.to_string())?;
        let recognition_model = rten::Model::load(recognition_model).map_err(|e| e.to_string())?;
        let engine = OcrEngine::new(OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(recognition_model),
            ..Default::default()
        })
        .map_err(|e| e.to_string())?;
        Ok(Reader { engine })
    }

    /// The text of a scanned PDF, page by page, as OCR reads the picture of
    /// each page, with the clean-up of [`tidy`].
    pub fn text_from_scan(&self, bytes: &[u8]) -> Result<String, DocumentError> {
        let pages = page_pictures(bytes)?;
        let mut texts = Vec::new();
        for page in &pages {
            let source = ImageSource::from_bytes(&page.gray, (page.width, page.height))
                .map_err(|_| DocumentError::Unreadable(SCAN))?;
            let input = self.engine.prepare_input(source).map_err(|_| DocumentError::Unreadable(SCAN))?;
            let text = self.engine.get_text(&input).map_err(|_| DocumentError::Unreadable(SCAN))?;
            if !text.trim().is_empty() {
                texts.push(tidy(&text));
            }
        }
        if texts.is_empty() { Err(DocumentError::NoText) } else { Ok(texts.join("\n\n")) }
    }
}

const SCAN: &str = "a scanned PDF";

/// One page picture, as 8-bit gray, one byte a pixel.
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub gray: Vec<u8>,
}

/// The largest picture on each page of a PDF, in page order.
///
/// Pictures stored as Flate-compressed samples (gray, RGB or CMYK, 1 or 8 bits)
/// or as JPEG are read; the fax formats (CCITT, JBIG2) and JPEG 2000 are not.
pub fn page_pictures(bytes: &[u8]) -> Result<Vec<Picture>, DocumentError> {
    if !bytes.starts_with(b"%PDF-") {
        return Err(DocumentError::Unreadable("a PDF"));
    }
    // The PDF library can panic on a damaged file.
    std::panic::catch_unwind(|| -> Result<Vec<Picture>, DocumentError> {
        let mut doc = Document::load_mem(bytes).map_err(|_| DocumentError::Unreadable("a PDF"))?;
        if doc.is_encrypted() {
            doc.decrypt("").map_err(|_| DocumentError::Unreadable("a PDF"))?;
        }
        let mut pictures = Vec::new();
        for (_, page_id) in doc.get_pages().into_iter().take(MAX_PAGES) {
            let mut largest: Option<&Stream> = None;
            for stream in page_images(&doc, page_id) {
                let area = |s: &Stream| int(&s.dict, b"Width") * int(&s.dict, b"Height");
                if int(&stream.dict, b"Width") >= MIN_SIDE
                    && int(&stream.dict, b"Height") >= MIN_SIDE
                    && largest.is_none_or(|l| area(stream) > area(l))
                {
                    largest = Some(stream);
                }
            }
            if let Some(stream) = largest {
                pictures.push(decode(&doc, stream)?);
            }
        }
        if pictures.is_empty() { Err(DocumentError::NoText) } else { Ok(pictures) }
    })
    .unwrap_or(Err(DocumentError::Unreadable("a PDF")))
}

/// The image streams a page draws directly.
fn page_images(doc: &Document, page_id: (u32, u16)) -> Vec<&Stream> {
    let Ok(page) = doc.get_dictionary(page_id) else { return Vec::new() };
    let Ok(resources) = doc.get_dict_in_dict(page, b"Resources") else { return Vec::new() };
    let Ok(xobjects) = doc.get_dict_in_dict(resources, b"XObject") else { return Vec::new() };
    xobjects
        .iter()
        .filter_map(|(_, value)| doc.get_object(value.as_reference().ok()?).ok()?.as_stream().ok())
        .filter(|s| s.dict.get(b"Subtype").and_then(Object::as_name).is_ok_and(|n| n == b"Image"))
        .collect()
}

fn int(dict: &Dictionary, key: &[u8]) -> i64 {
    dict.get(key).and_then(Object::as_i64).unwrap_or(0)
}

fn decode(doc: &Document, stream: &Stream) -> Result<Picture, DocumentError> {
    let unreadable = DocumentError::Unreadable(SCAN);
    let (width, height) = (int(&stream.dict, b"Width"), int(&stream.dict, b"Height"));
    if width <= 0 || height <= 0 || width * height > MAX_PIXELS {
        return Err(DocumentError::TooLarge);
    }
    let filters: Vec<Vec<u8>> = match stream.dict.get(b"Filter") {
        Ok(Object::Name(n)) => vec![n.clone()],
        Ok(Object::Array(a)) => a.iter().filter_map(|o| o.as_name().ok().map(<[u8]>::to_vec)).collect(),
        _ => Vec::new(),
    };
    let (w, h) = (width as usize, height as usize);

    let gray = if filters.last().is_some_and(|f| f == b"DCTDecode") {
        if filters.len() != 1 {
            return Err(unreadable);
        }
        let mut decoder = jpeg_decoder::Decoder::new(stream.content.as_slice());
        let samples = decoder.decode().map_err(|_| DocumentError::Unreadable(SCAN))?;
        let info = decoder.info().ok_or(DocumentError::Unreadable(SCAN))?;
        if info.width as usize != w || info.height as usize != h {
            return Err(unreadable);
        }
        let channels = match info.pixel_format {
            jpeg_decoder::PixelFormat::L8 => 1,
            jpeg_decoder::PixelFormat::RGB24 => 3,
            // Adobe writes CMYK JPEGs inverted.
            jpeg_decoder::PixelFormat::CMYK32 => {
                let inverted: Vec<u8> = samples.iter().map(|v| 255 - v).collect();
                return Ok(Picture { width: w as u32, height: h as u32, gray: to_gray(&inverted, 4, w * h) });
            }
            jpeg_decoder::PixelFormat::L16 => return Err(unreadable),
        };
        to_gray(&samples, channels, w * h)
    } else {
        if filters.iter().any(|f| f != b"FlateDecode" && f != b"LZWDecode") {
            return Err(unreadable);
        }
        let samples = stream.decompressed_content().map_err(|_| DocumentError::Unreadable(SCAN))?;
        let channels = channels(doc, stream.dict.get(b"ColorSpace").ok()).ok_or(DocumentError::Unreadable(SCAN))?;
        match int(&stream.dict, b"BitsPerComponent") {
            8 if samples.len() >= w * h * channels => to_gray(&samples, channels, w * h),
            1 if channels == 1 && samples.len() >= w.div_ceil(8) * h => {
                let row = w.div_ceil(8);
                (0..h)
                    .flat_map(|y| (0..w).map(move |x| (y, x)))
                    .map(|(y, x)| if samples[y * row + x / 8] & (0x80 >> (x % 8)) != 0 { 255 } else { 0 })
                    .collect()
            }
            _ => return Err(unreadable),
        }
    };
    Ok(Picture { width: w as u32, height: h as u32, gray })
}

/// How many samples a pixel has in a color space: 1, 3 or 4.
fn channels(doc: &Document, space: Option<&Object>) -> Option<usize> {
    let space = match space? {
        Object::Reference(id) => doc.get_object(*id).ok()?,
        other => other,
    };
    match space {
        Object::Name(n) => match n.as_slice() {
            b"DeviceGray" | b"CalGray" => Some(1),
            b"DeviceRGB" | b"CalRGB" => Some(3),
            b"DeviceCMYK" => Some(4),
            _ => None,
        },
        Object::Array(a) => match a.first()?.as_name().ok()? {
            b"ICCBased" => {
                let profile = match a.get(1)? {
                    Object::Reference(id) => doc.get_object(*id).ok()?.as_stream().ok()?,
                    _ => return None,
                };
                match int(&profile.dict, b"N") {
                    n @ (1 | 3 | 4) => Some(n as usize),
                    _ => None,
                }
            }
            b"CalGray" => Some(1),
            b"CalRGB" => Some(3),
            _ => None,
        },
        _ => None,
    }
}

fn to_gray(samples: &[u8], channels: usize, pixels: usize) -> Vec<u8> {
    samples
        .chunks_exact(channels)
        .take(pixels)
        .map(|p| match p {
            [g] => *g,
            [r, g, b] => ((*r as u32 * 299 + *g as u32 * 587 + *b as u32 * 114) / 1000) as u8,
            [c, m, y, k] => {
                let ink = |v: u8| (255 - v as u32) * (255 - *k as u32) / 255;
                ((ink(*c) * 299 + ink(*m) * 587 + ink(*y) * 114) / 1000) as u8
            }
            _ => 255,
        })
        .collect()
}

/// Undoes three misreadings OCR makes on printed forms, so that the engine,
/// which expects printed punctuation, sees the label it looks for:
///
/// - a title followed by a comma, `Dr, Miriam Okafor` → `Dr. Miriam Okafor`;
/// - a city followed by a period before a state and ZIP code,
///   `Pinecrest. OR 97499` → `Pinecrest, OR 97499`;
/// - a lower-case `l` closing a capitalised abbreviation, `NPl` → `NPI`.
///
/// Only OCR text goes through this; typed and printed text is never changed.
pub fn tidy(text: &str) -> String {
    static TITLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(Dr|Mr|Mrs|Ms),( [A-Z])").unwrap());
    static CITY: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\b([A-Z][a-z]+)\. ([A-Z]{2} \d{5}(?:-\d{4})?)\b").unwrap());
    static ABBREVIATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b([A-Z]{2,})l\b").unwrap());
    let text = TITLE.replace_all(text, "$1.$2");
    let text = CITY.replace_all(&text, "$1, $2");
    ABBREVIATION.replace_all(&text, "${1}I").into_owned()
}

#[cfg(test)]
mod tests {
    use super::tidy;

    #[test]
    fn tidy_restores_the_punctuation_of_a_form() {
        assert_eq!(tidy("Physician: Dr, Miriam Okafor"), "Physician: Dr. Miriam Okafor");
        assert_eq!(tidy("City/State/ZIP: Pinecrest. OR 97499"), "City/State/ZIP: Pinecrest, OR 97499");
        assert_eq!(tidy("NPl: 1093847562"), "NPI: 1093847562");
    }

    #[test]
    fn tidy_leaves_ordinary_text_alone() {
        for text in [
            "Seen by Dr. Okafor. Follow up in 2 weeks.",
            "Started lisinopril. BP 150/90 at rest.",
            "Lives in Pinecrest, OR 97499.",
            "Taking 10 ml daily. Hgb normal. COPD stable.",
            "Mr, as he prefers, was seen today.",
        ] {
            assert_eq!(tidy(text), text);
        }
    }
}
