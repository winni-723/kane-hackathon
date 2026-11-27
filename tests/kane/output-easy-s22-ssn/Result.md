---
test: ../easy-s22-ssn_test.md
status: passed
started: 2026-11-26T22:42:00.028Z
duration_s: 49
session_id: 96ac2bbe-8829-4ea3-8fce-a5b3d26507c8
---

# easy-s22: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (24.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (20.8s)
md5: b32a3c66c4bcfc81e87c1f0ed5b0f109
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text '999-86-1441'.
