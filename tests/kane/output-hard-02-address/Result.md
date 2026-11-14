---
test: ../hard-02-address_test.md
status: passed
started: 2026-11-14T04:42:19.279Z
duration_s: 85
session_id: c554ee63-88e0-4af9-896b-32ca427a1f3d
---

# hard-02: address is removed, medical content is kept — Result

## Submit the record ✓ passed (34.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (45s)
md5: 0dcdde95635e9f9948d1ec3b0837f328
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'apartment over the pharmacy on Kessler Street'.
