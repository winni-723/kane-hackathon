---
test: ../easy-07-dob_test.md
status: passed
started: 2026-11-26T22:30:08.230Z
duration_s: 58
session_id: a5a25e94-ec70-48eb-9096-c0919d7ca821
---

# easy-07: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (25.5s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (28.8s)
md5: fecae34503730dc7d41407b06e3b1bf5
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text '05/22/1980'.
