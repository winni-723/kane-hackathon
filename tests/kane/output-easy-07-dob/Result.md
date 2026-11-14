---
test: ../easy-07-dob_test.md
status: passed
started: 2026-11-14T04:10:55.250Z
duration_s: 81
session_id: efb5066a-dba3-4277-936f-3147e791ab0f
---

# easy-07: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (37.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (39.3s)
md5: fecae34503730dc7d41407b06e3b1bf5
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text '05/22/1980'.
