---
test: ../medium-102-email_test.md
status: passed
started: 2026-11-15T17:56:13.661Z
duration_s: 70
session_id: 0879daaf-a9c9-46cf-ad56-4b84b9d98f07
---

# medium-102: email is removed, medical content is kept — Result

## Submit the record ✓ passed (26.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (39s)
md5: f96111338671b5452f92b3843b7cbac9
Verify that the Result region contains the text 'post-op day 2'.
Verify that the Result region contains the text 'right hemicolectomy'.
Verify that the Result region contains the text 'adenocarcinoma of the ascending colon'.
Verify that the Result region contains the text 'Hydromorphone PCA'.
Verify that the Result region contains the text 'Ambulated 200 feet'.
Verify that the Result region contains the text 'Drain output 40 mL'.
Verify that the Result region contains the text 'Cefazolin 2 g IV every 8 hours'.
Verify that the Result region contains the text 'Heparin 5000 units subcutaneously every 12 hours'.
Verify that the Result region does not contain the text 'yusuf.abdullahi@example.com'.
