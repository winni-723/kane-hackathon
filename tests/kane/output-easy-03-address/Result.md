---
test: ../easy-03-address_test.md
status: passed
started: 2026-11-26T22:21:25.058Z
duration_s: 63
session_id: 86d58607-4a83-42fb-94e4-349727e0994e
---

# easy-03: address is removed, medical content is kept — Result

## Submit the record ✓ passed (24.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (34.3s)
md5: 90b2248024ca22eef79c85204d2289ae
Verify that the Result region contains the text 'Severe unilateral headache'.
Verify that the Result region contains the text 'photophobia'.
Verify that the Result region contains the text '6 to 12 hours'.
Verify that the Result region contains the text 'Migraine without aura'.
Verify that the Result region contains the text 'Sumatriptan 50 mg as needed'.
Verify that the Result region contains the text 'Propranolol 40 mg twice daily'.
Verify that the Result region does not contain the text '77 Harbor Lane'.
Verify that the Result region does not contain the text 'Fenwick Hollow'.
Verify that the Result region does not contain the text '60699'.
