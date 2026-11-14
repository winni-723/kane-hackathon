---
test: ../easy-03-dob_test.md
status: passed
started: 2026-11-14T04:06:09.109Z
duration_s: 88
session_id: e24a5a92-a426-4556-9278-cebaa8affd91
---

# easy-03: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (40.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (40.3s)
md5: 1f874b04b83afffd513bea6ecf1c18c5
Verify that the Result region contains the text 'Severe unilateral headache'.
Verify that the Result region contains the text 'photophobia'.
Verify that the Result region contains the text '6 to 12 hours'.
Verify that the Result region contains the text 'Migraine without aura'.
Verify that the Result region contains the text 'Sumatriptan 50 mg as needed'.
Verify that the Result region contains the text 'Propranolol 40 mg twice daily'.
Verify that the Result region does not contain the text '07/29/1989'.
