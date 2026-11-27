---
test: ../hard-02-dob_test.md
status: passed
started: 2026-11-26T23:35:52.416Z
duration_s: 60
session_id: 863fcb40-d9b5-4870-a695-25004306460d
---

# hard-02: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (21.8s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (31.3s)
md5: fc724986a9296f4f7c94deb256374e6c
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'the fourth of July, 1944'.
