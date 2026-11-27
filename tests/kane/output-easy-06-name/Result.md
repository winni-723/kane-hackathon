---
test: ../easy-06-name_test.md
status: passed
started: 2026-11-26T22:29:02.571Z
duration_s: 52
session_id: adf27ae9-6926-42f7-b2d1-709a6e82118c
---

# easy-06: name is removed, medical content is kept — Result

## Submit the record ✓ passed (21.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone ✓ passed (26.7s)
md5: ffcdd9eb65662e0fe1cedd28c5ba6e26
Verify that the Result region contains the text 'Major depressive disorder'.
Verify that the Result region contains the text 'Sertraline 100 mg daily'.
Verify that the Result region contains the text 'PHQ-9 score 14'.
Verify that the Result region contains the text 'improved from 19'.
Verify that the Result region contains the text 'cognitive behavioral therapy weekly'.
Verify that the Result region does not contain the text 'Rosalind Achterberg'.
