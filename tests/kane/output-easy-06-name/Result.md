---
test: ../easy-06-name_test.md
status: passed
started: 2026-11-14T04:38:50.609Z
duration_s: 83
session_id: c9671b4a-fc3a-466d-897e-ce76c659523f
---

# easy-06: name is removed, medical content is kept — Result

## Submit the record ✓ passed (37.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (40.1s)
md5: ffcdd9eb65662e0fe1cedd28c5ba6e26
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text 'Rosalind Achterberg'.
