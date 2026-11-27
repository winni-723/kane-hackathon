---
test: ../hard-04-ssn_test.md
status: passed
started: 2026-11-26T22:49:47.443Z
duration_s: 57
session_id: c7f7ad65-a39d-4a8a-bdc7-571287e2913c
---

# hard-04: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (20.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (32.9s)
md5: b19c8db2eb3abf62202ddf88b4cd3981
Verify that the Result region contains the text '34 y/o veteran'.
Verify that the Result region contains the text 'nightmares and hypervigilance'.
Verify that the Result region contains the text 'PCL-5 score is 52'.
Verify that the Result region contains the text 'post-traumatic stress disorder'.
Verify that the Result region contains the text 'Sertraline 50 mg daily'.
Verify that the Result region contains the text 'Prazosin 1 mg at bedtime'.
Verify that the Result region contains the text 'Columbia Suicide Severity Rating Scale'.
Verify that the Result region does not contain the text '000 12 3456'.
