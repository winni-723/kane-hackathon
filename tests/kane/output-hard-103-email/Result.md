---
test: ../hard-103-email_test.md
status: passed
started: 2026-11-15T17:51:20.244Z
duration_s: 66
session_id: fff36e5e-61c4-4984-a113-a9a0cca22f72
---

# hard-103: email is removed, medical content is kept — Result

## Submit the record ✓ passed (27.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (33.7s)
md5: 9f6409b24a0ebd6217db781fe90b9d96
Verify that the Result region contains the text 'MMSE 21/30'.
Verify that the Result region contains the text 'Geriatric Depression Scale 9/15'.
Verify that the Result region contains the text 'Two falls in the past 3 months'.
Verify that the Result region contains the text 'orthostatic drop of 24 mmHg systolic'.
Verify that the Result region contains the text 'stop Diphenhydramine'.
Verify that the Result region contains the text 'Donepezil 10 mg nightly'.
Verify that the Result region contains the text 'Braden score 16'.
Verify that the Result region does not contain the text 'p.fenwick.araujo@example.org'.
