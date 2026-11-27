---
test: ../medium-102-dob_test.md
status: passed
started: 2026-11-26T23:38:21.888Z
duration_s: 63
session_id: 044ef5ec-86f0-4ba3-ac4b-0829afd07002
---

# medium-102: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (23.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (35.1s)
md5: 8ce8b64af481f72da22a3285ddcd665e
Verify that the Result region contains the text 'post-op day 2'.
Verify that the Result region contains the text 'right hemicolectomy'.
Verify that the Result region contains the text 'adenocarcinoma of the ascending colon'.
Verify that the Result region contains the text 'Hydromorphone PCA'.
Verify that the Result region contains the text 'Ambulated 200 feet'.
Verify that the Result region contains the text 'Drain output 40 mL'.
Verify that the Result region contains the text 'Cefazolin 2 g IV every 8 hours'.
Verify that the Result region contains the text 'Heparin 5000 units subcutaneously every 12 hours'.
Verify that the Result region does not contain the text 'January 4, 1949'.
