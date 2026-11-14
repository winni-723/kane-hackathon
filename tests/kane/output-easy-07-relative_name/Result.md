---
test: ../easy-07-relative_name_test.md
status: passed
started: 2026-11-14T04:40:23.587Z
duration_s: 94
session_id: 07e76cba-5118-4c13-8d83-bdd8cf051c64
---

# easy-07: relative name is removed, medical content is kept — Result

## Submit the record ✓ passed (31.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted relative name is gone ✓ passed (58s)
md5: 533def169eb9873d1c052e75522556c8
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text 'Ingrid Lindqvist'.
