---
test: ../medium-06-phone_test.md
status: passed
started: 2026-11-14T03:31:38.647Z
duration_s: 89
session_id: 515bd80a-1f8a-4352-a2a7-8628b3441092
---

# medium-06: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (32.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (51.6s)
md5: 41943e6b7b852d9602e0ab06569cafb7
Verify that the Result region contains the text '4-year-old son'.
Verify that the Result region contains the text 'fever of 102.4 F'.
Verify that the Result region contains the text 'right ear pain for two days'.
Verify that the Result region contains the text 'acute otitis media'.
Verify that the Result region contains the text 'Amoxicillin 400 mg twice daily for 10 days'.
Verify that the Result region contains the text 'Acetaminophen 160 mg every 6 hours as needed'.
Verify that the Result region contains the text 'beyond 48 hours'.
Verify that the Result region does not contain the text '248-555-0163'.
