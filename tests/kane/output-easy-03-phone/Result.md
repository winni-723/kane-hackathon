---
test: ../easy-03-phone_test.md
status: passed
started: 2026-11-14T02:32:55.741Z
duration_s: 92
session_id: ef695af3-605b-449c-97f2-241fadf8f10f
---

# easy-03: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (35.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (51s)
md5: 6ad81dcc71a9ffd14f0ec3f5cd417c14
Verify that the Result region contains the text 'Severe unilateral headache'.
Verify that the Result region contains the text 'photophobia'.
Verify that the Result region contains the text '6 to 12 hours'.
Verify that the Result region contains the text 'Migraine without aura'.
Verify that the Result region contains the text 'Sumatriptan 50 mg as needed'.
Verify that the Result region contains the text 'Propranolol 40 mg twice daily'.
Verify that the Result region does not contain the text '(312) 555-0165'.
Verify that the Result region does not contain the text '(312) 555-0190'.
Verify that the Result region does not contain the text '(312) 555-0133'.
