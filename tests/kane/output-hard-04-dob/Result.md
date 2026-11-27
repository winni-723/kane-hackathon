---
test: ../hard-04-dob_test.md
status: passed
started: 2026-11-26T22:48:37.693Z
duration_s: 60
session_id: bdf008cd-9262-499f-b41b-534af8463aa1
---

# hard-04: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (25.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (30s)
md5: 02aab1b37eaac25d117e3f60508f0a30
Verify that the Result region contains the text '34 y/o veteran'.
Verify that the Result region contains the text 'nightmares and hypervigilance'.
Verify that the Result region contains the text 'PCL-5 score is 52'.
Verify that the Result region contains the text 'post-traumatic stress disorder'.
Verify that the Result region contains the text 'Sertraline 50 mg daily'.
Verify that the Result region contains the text 'Prazosin 1 mg at bedtime'.
Verify that the Result region contains the text 'Columbia Suicide Severity Rating Scale'.
Verify that the Result region does not contain the text "March 3rd, '91".
