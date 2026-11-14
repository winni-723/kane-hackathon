---
test: ../easy-02-phone_test.md
status: passed
started: 2026-11-14T03:26:53.440Z
duration_s: 86
session_id: 35d7d707-3d1f-4888-912a-e63edbb6dacb
---

# easy-02: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (40.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (38s)
md5: c97cdff09f7f98157517041c2bf68508
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text '415-555-0117'.
