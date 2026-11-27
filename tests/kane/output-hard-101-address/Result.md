---
test: ../hard-101-address_test.md
status: passed
started: 2026-11-26T22:54:40.484Z
duration_s: 55
session_id: 2bd1cb5d-2147-4878-9dfa-e4cb0eacc8b0
---

# hard-101: address is removed, medical content is kept — Result

## Submit the record ✓ passed (20.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (30.7s)
md5: a9cd7bee5ba30bbd7ff64bbf7ccc528a
Verify that the Result region contains the text 'witnessed tonic-clonic seizure'.
Verify that the Result region contains the text 'GCS 13 improving to 15'.
Verify that the Result region contains the text 'Glucose 96, sodium 138'.
Verify that the Result region contains the text 'CT head negative'.
Verify that the Result region contains the text 'Levetiracetam 1500 mg IV'.
Verify that the Result region contains the text 'traumatic brain injury'.
Verify that the Result region does not contain the text 'upstairs unit of the brick two-flat next to the Polish bakery on Carrow Street'.
