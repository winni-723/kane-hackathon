---
test: ../easy-02-name_test.md
status: passed
started: 2026-11-14T04:35:39.545Z
duration_s: 88
session_id: 7bab9f63-8df3-499a-9846-57376cd0b60a
---

# easy-02: name is removed, medical content is kept — Result

## Submit the record ✓ passed (41.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (38.8s)
md5: b6ce9a1f8d3201cc66931f667793a133
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text 'Marcus Delgado'.
