---
test: ../easy-07-address_test.md
status: passed
started: 2026-11-26T22:30:02.890Z
duration_s: 62
session_id: ae41743f-f7d7-47f8-a9a6-e5bd8b8edacd
---

# easy-07: address is removed, medical content is kept — Result

## Submit the record ✓ passed (22.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (35.8s)
md5: 89c121d6f9bda2786e7bbc2b8cfca3cd
Verify that the Result region contains the text 'Right knee pain and swelling'.
Verify that the Result region contains the text 'MRI shows a medial meniscus tear'.
Verify that the Result region contains the text 'Ibuprofen 600 mg every 8 hours with food'.
Verify that the Result region contains the text 'arthroscopy'.
Verify that the Result region contains the text 'ice 20 minutes three times daily'.
Verify that the Result region does not contain the text '204 Birchwater Trail'.
Verify that the Result region does not contain the text 'Ellery Falls'.
Verify that the Result region does not contain the text '55499'.
