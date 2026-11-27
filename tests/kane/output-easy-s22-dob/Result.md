---
test: ../easy-s22-dob_test.md
status: passed
started: 2026-11-26T22:41:00.663Z
duration_s: 49
session_id: 3e8ba7cf-03b7-4859-928b-9973919aa232
---

# easy-s22: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (22.2s)
md5: 4580dd20245d2b2b5a1519758b081a1c
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text '2005-10-09'.
