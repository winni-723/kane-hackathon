---
test: ../easy-02-email_test.md
status: passed
started: 2026-11-26T22:20:17.976Z
duration_s: 54
session_id: 50f6599d-3e4b-4783-8e32-f7c42f9062a3
---

# easy-02: email is removed, medical content is kept — Result

## Submit the record ✓ passed (21.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (28.8s)
md5: 0274165e61d26bd2a17fb3a4cf37fed9
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text 'marcus.delgado@example.com'.
