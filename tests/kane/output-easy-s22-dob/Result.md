---
test: ../easy-s22-dob_test.md
status: passed
started: 2026-11-15T17:45:25.862Z
duration_s: 55
session_id: 411dac00-6c16-426f-82e1-b574cda5c9eb
---

# easy-s22: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (26s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (25.1s)
md5: 4580dd20245d2b2b5a1519758b081a1c
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text '2005-10-09'.
