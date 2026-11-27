---
test: ../easy-07-name_test.md
status: passed
started: 2026-11-26T22:30:31.115Z
duration_s: 57
session_id: c378511e-1e38-42a2-aef0-bcf06b81c76c
---

# easy-07: name is removed, medical content is kept — Result

## Submit the record ✓ passed (21.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (31.3s)
md5: 91d2c175ac88e1690d12057e7f59e50d
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text 'Tobias Lindqvist'.
