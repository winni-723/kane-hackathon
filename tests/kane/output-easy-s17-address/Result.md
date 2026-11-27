---
test: ../easy-s17-address_test.md
status: passed
started: 2026-11-26T22:39:52.207Z
duration_s: 50
session_id: 3cde7d81-6a40-44a5-ad2a-436a4b8e5c5b
---

# easy-s17: address is removed, medical content is kept — Result

## Submit the record ✓ passed (22.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (23.5s)
md5: d7e8a01c634b39c07dd49e03055a27e5
Verify that the Result region contains the text 'Medication review due (situation)'.
Verify that the Result region does not contain the text '517 Bogan Passage'.
Verify that the Result region does not contain the text 'Quincy'.
Verify that the Result region does not contain the text '02169'.
