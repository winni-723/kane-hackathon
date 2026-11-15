---
test: ../hard-101-dob_test.md
status: passed
started: 2026-11-15T17:47:32.163Z
duration_s: 63
session_id: 6b1bbed5-e2d6-480f-9955-a70980e7a047
---

# hard-101: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (26.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30.8s)
md5: b2a9cc8d45ab25ca947c34e89a8e9475
Verify that the Result region contains the text 'witnessed tonic-clonic seizure'.
Verify that the Result region contains the text 'GCS 13 improving to 15'.
Verify that the Result region contains the text 'Glucose 96, sodium 138'.
Verify that the Result region contains the text 'CT head negative'.
Verify that the Result region contains the text 'Levetiracetam 1500 mg IV'.
Verify that the Result region contains the text 'traumatic brain injury'.
Verify that the Result region does not contain the text '12.07.1978'.
