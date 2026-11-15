---
test: ../hard-101-address_test.md
status: passed
started: 2026-11-15T17:46:55.268Z
duration_s: 64
session_id: e854e400-63af-492d-9adf-5f700357e0a0
---

# hard-101: address is removed, medical content is kept — Result

## Submit the record ✓ passed (25.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (33.3s)
md5: a9cd7bee5ba30bbd7ff64bbf7ccc528a
Verify that the Result region contains the text 'witnessed tonic-clonic seizure'.
Verify that the Result region contains the text 'GCS 13 improving to 15'.
Verify that the Result region contains the text 'Glucose 96, sodium 138'.
Verify that the Result region contains the text 'CT head negative'.
Verify that the Result region contains the text 'Levetiracetam 1500 mg IV'.
Verify that the Result region contains the text 'traumatic brain injury'.
Verify that the Result region does not contain the text 'upstairs unit of the brick two-flat next to the Polish bakery on Carrow Street'.
