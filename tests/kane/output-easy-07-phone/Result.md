---
test: ../easy-07-phone_test.md
status: passed
started: 2026-11-14T03:28:29.296Z
duration_s: 87
session_id: 00d08cfa-ebf0-4b6d-b848-055f8d1f7ba3
---

# easy-07: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (32.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (48.7s)
md5: c03b1f50fcfa5f5d661b96a8d09f9ab4
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text '(612) 555-0129'.
Verify that the Result region does not contain the text '(612) 555-0181'.
Verify that the Result region does not contain the text '(612) 555-0102'.
