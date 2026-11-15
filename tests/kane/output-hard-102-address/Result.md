---
test: ../hard-102-address_test.md
status: passed
started: 2026-11-15T17:48:56.652Z
duration_s: 65
session_id: 8a23eb62-37c9-4203-a424-503292107bca
---

# hard-102: address is removed, medical content is kept — Result

## Submit the record ✓ passed (29.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (29.5s)
md5: dae261fff5e3a25453f055df387be8c7
Verify that the Result region contains the text 'prednisone taper'.
Verify that the Result region contains the text 'Contact dermatitis, recurrent'.
Verify that the Result region contains the text 'Prednisone 40 mg daily tapering over 12 days'.
Verify that the Result region contains the text 'Triamcinolone 0.1% cream twice daily'.
Verify that the Result region contains the text 'Patch testing referral placed'.
Verify that the Result region does not contain the text 'above the nail salon across from Greeley Park'.
