---
test: ../easy-07-name_test.md
status: passed
started: 2026-11-14T04:38:53.997Z
duration_s: 88
session_id: 4a5b5e5e-1bfb-47e3-9fe3-ec4a625411ea
---

# easy-07: name is removed, medical content is kept — Result

## Submit the record ✓ passed (45.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (37.7s)
md5: 91d2c175ac88e1690d12057e7f59e50d
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text 'Tobias Lindqvist'.
