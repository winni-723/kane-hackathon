---
test: ../easy-s22-id_number_test.md
status: passed
started: 2026-11-26T22:41:43.441Z
duration_s: 52
session_id: b7b09bbe-0991-4717-b7d7-bf358fe1ac79
---

# easy-s22: id number is removed, medical content is kept — Result

## Submit the record ✓ passed (22.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted id number is gone ✓ passed (25.6s)
md5: 8daf9a16471e51c309c9a78b9fd20bce
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text 'S99913362'.
Verify that the Result region does not contain the text 'X65093286X'.
