---
test: ../ui-download_test.md
status: passed
started: 2026-11-14T03:31:48.697Z
duration_s: 86
session_id: 67cc4808-5ddf-4937-8300-c3d68030a26e
---

# The Result offers a download and a way back to the original — Result

## Submit the record ✓ passed (33.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## The Result has its controls ✓ passed (46.6s)
md5: 606a1c81ea6b1fbde8d40f4bc3ccaf6b
Verify that the Result region contains the text 'Migraine without aura'.
Verify that the Result region does not contain the text '(617) 555-0142'.
Verify that a button labelled 'Download result' is visible in the Result region.
Verify that a button labelled 'Edit original' is visible in the Result region.
