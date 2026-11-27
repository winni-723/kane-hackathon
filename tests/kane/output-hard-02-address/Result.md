---
test: ../hard-02-address_test.md
status: passed
started: 2026-11-26T22:44:04.675Z
duration_s: 58
session_id: 5cf168e9-6a83-45c4-bcff-912270b57cdd
---

# hard-02: address is removed, medical content is kept — Result

## Submit the record ✓ passed (21.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (32.8s)
md5: 0dcdde95635e9f9948d1ec3b0837f328
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'apartment over the pharmacy on Kessler Street'.
