---
test: ../medium-06-phone_test.md
status: passed
started: 2026-11-26T23:11:38.284Z
duration_s: 55
session_id: 1646ffa0-0e2c-4b2c-9d65-d8520776204e
---

# medium-06: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (20.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31.3s)
md5: 41943e6b7b852d9602e0ab06569cafb7
Verify that the Result region contains the text '4-year-old son'.
Verify that the Result region contains the text 'fever of 102.4 F'.
Verify that the Result region contains the text 'right ear pain for two days'.
Verify that the Result region contains the text 'acute otitis media'.
Verify that the Result region contains the text 'Amoxicillin 400 mg twice daily for 10 days'.
Verify that the Result region contains the text 'Acetaminophen 160 mg every 6 hours as needed'.
Verify that the Result region contains the text 'beyond 48 hours'.
Verify that the Result region does not contain the text '248-555-0163'.
