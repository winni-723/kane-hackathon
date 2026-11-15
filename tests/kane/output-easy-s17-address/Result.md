---
test: ../easy-s17-address_test.md
status: passed
started: 2026-11-15T17:43:39.618Z
duration_s: 55
session_id: 09fd2fa7-636d-4a7b-939f-bfa07f9ed72e
---

# easy-s17: address is removed, medical content is kept — Result

## Submit the record ✓ passed (25.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (25.3s)
md5: d7e8a01c634b39c07dd49e03055a27e5
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '517 Bogan Passage'.
Verify that the Result region does not contain the text 'Quincy'.
Verify that the Result region does not contain the text '02169'.
