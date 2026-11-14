---
test: ../medium-05-dob_test.md
status: passed
started: 2026-11-14T04:24:08.861Z
duration_s: 81
session_id: 014630e5-54b2-40d9-817a-6b61517bd4ec
---

# medium-05: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (32.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (43.4s)
md5: 8e4fcc6f268e79686583cb659ee85fec
Verify that the Result region contains the text 'rheumatoid arthritis'.
Verify that the Result region contains the text 'morning stiffness lasting 90 minutes'.
Verify that the Result region contains the text 'Methotrexate 15 mg weekly'.
Verify that the Result region contains the text 'Folic acid 1 mg daily'.
Verify that the Result region contains the text 'ESR is 42 mm/hr'.
Verify that the Result region contains the text 'Hydroxychloroquine 200 mg twice daily'.
Verify that the Result region does not contain the text '10/03/1974'.
