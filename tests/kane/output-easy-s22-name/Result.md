---
test: ../easy-s22-name_test.md
status: passed
started: 2026-11-26T22:41:57.221Z
duration_s: 52
session_id: 32821059-9aa8-44a8-af74-882e2d099ff8
---

# easy-s22: name is removed, medical content is kept — Result

## Submit the record ✓ passed (22s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (25.5s)
md5: f9530066a532a398787c5236f614ceb4
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text 'Jack'.
Verify that the Result region does not contain the text 'Wisozk'.
