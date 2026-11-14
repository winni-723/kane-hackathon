---
test: ../hard-02-relative_name_test.md
status: passed
started: 2026-11-14T05:02:04.506Z
duration_s: 89
session_id: 4a232f1f-f847-4e84-9f17-d24a0ce9b36e
---

# hard-02: relative name is removed, medical content is kept — Result

## Submit the record ✓ passed (36s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted relative name is gone ✓ passed (46s)
md5: 40e08a55e72409f8a45055143ba56c83
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'Mr. Vance'.
