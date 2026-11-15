---
test: ../hard-101-phone_test.md
status: passed
started: 2026-11-15T17:48:50.399Z
duration_s: 71
session_id: aee8987f-b37f-491a-99cf-3ea4f5777a32
---

# hard-101: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (24.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (39.3s)
md5: 2940a46b974e0918661e603960056891
Verify that the Result region contains the text 'witnessed tonic-clonic seizure'.
Verify that the Result region contains the text 'GCS 13 improving to 15'.
Verify that the Result region contains the text 'Glucose 96, sodium 138'.
Verify that the Result region contains the text 'CT head negative'.
Verify that the Result region contains the text 'Levetiracetam 1500 mg IV'.
Verify that the Result region contains the text 'traumatic brain injury'.
Verify that the Result region does not contain the text '4 1 2 - 5 5 5 - 0 1 6 2'.
