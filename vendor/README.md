# Vendored crates

## pdf-extract 0.12.1 (MIT, https://github.com/jrmuizel/pdf-extract)

A copy of the published crate with one fix, used through `[patch.crates-io]`
in the top-level `Cargo.toml`.

In a CID font's `W` array, a run of characters can share one width:
`c_first c_last width`. Version 0.12.1 read all three numbers from the same
place, so every character in such a run got the default width. Chrome writes
its fonts this way, and text printed by Chrome came out with spaces inside
words (`C obalt`, `1 1/04/2026`). The fix is in `src/lib.rs`, marked
"Patched for kane-hackathon". The only other change is `#![allow(warnings)]`
at the top of `src/lib.rs`: as a local path crate its upstream warnings would
otherwise print in every build here.
