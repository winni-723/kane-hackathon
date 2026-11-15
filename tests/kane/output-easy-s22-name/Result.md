---
test: ../easy-s22-name_test.md
status: passed
started: 2026-11-15T18:11:50.097Z
duration_s: 76
session_id: 224dfc0d-5b05-47fc-929d-5e27593b1868
---

# easy-s22: name is removed, medical content is kept — Result

## Submit the record ✓ passed (30.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (39.9s)
md5: f9530066a532a398787c5236f614ceb4
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text 'Jack'.
Verify that the Result region does not contain the text 'Wisozk'.
