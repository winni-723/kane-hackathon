---
test: ../hard-102-address_test.md
status: passed
started: 2026-11-26T22:56:30.715Z
duration_s: 61
session_id: 04c06cf9-ec40-4864-afd6-bbeb12f45e39
---

# hard-102: address is removed, medical content is kept — Result

## Submit the record ✓ passed (27.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (29.6s)
md5: dae261fff5e3a25453f055df387be8c7
Verify that the Result region contains the text 'prednisone taper'.
Verify that the Result region contains the text 'Contact dermatitis, recurrent'.
Verify that the Result region contains the text 'Prednisone 40 mg daily tapering over 12 days'.
Verify that the Result region contains the text 'Triamcinolone 0.1% cream twice daily'.
Verify that the Result region contains the text 'Patch testing referral placed'.
Verify that the Result region does not contain the text 'above the nail salon across from Greeley Park'.
