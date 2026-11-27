---
test: ../medium-05-phone_test.md
status: passed
started: 2026-11-26T23:09:24.920Z
duration_s: 59
session_id: ab02eaf6-e5a2-42bb-a5e9-83b634def515
---

# medium-05: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (20.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (34.4s)
md5: 3edb2f4f909eca3fc8760300e20b75ef
Verify that the Result region contains the text 'rheumatoid arthritis'.
Verify that the Result region contains the text 'morning stiffness lasting 90 minutes'.
Verify that the Result region contains the text 'Methotrexate 15 mg weekly'.
Verify that the Result region contains the text 'Folic acid 1 mg daily'.
Verify that the Result region contains the text 'ESR is 42 mm/hr'.
Verify that the Result region contains the text 'Hydroxychloroquine 200 mg twice daily'.
Verify that the Result region does not contain the text '504.555.0121'.
