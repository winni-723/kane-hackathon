# OCR models

The two files here are the pre-trained models of ocrs, used to read scanned
PDFs in the browser (`src/ocr.rs`). The page fetches them from this site only
when a visitor asks for a scan to be read.

| File | Size | SHA-256 |
|---|---|---|
| `text-detection.onnx` | 2,499,479 bytes | `a917b23dbd9524b465df7e922641b3ff2981623df4ded5a0234004ef2fee7cfe` |
| `text-recognition.onnx` | 9,713,177 bytes | `86c145c2edb96c8caed5b1ebb8f44d706408922211451c309c625157dd6061c5` |

They are the files ocrs-cli 0.14.0 downloads on first run, unchanged.

## License

Ocrs models (https://github.com/robertknight/ocrs-models), trained on the
HierText dataset by Google Research. Licensed under CC BY-SA 4.0
(https://creativecommons.org/licenses/by-sa/4.0/).

The license covers these two model files only. The ocrs library itself is
MIT or Apache-2.0.
