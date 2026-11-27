---
test: ../hard-02-relative_name_test.md
status: passed
started: 2026-11-26T22:45:15.784Z
duration_s: 63
session_id: fe1dd868-5776-4a74-9ecb-e88d21632041
---

# hard-02: relative name is removed, medical content is kept — Result

## Submit the record ✓ passed (24.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted relative name is gone ✓ passed (34.5s)
md5: 40e08a55e72409f8a45055143ba56c83
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'Mr. Vance'.
