---
test: ../easy-101-address_test.md
status: passed
started: 2026-11-15T17:39:36.415Z
duration_s: 104
session_id: a461600a-2ee7-4fc5-bfbd-ecf7ac59575b
---

# easy-101: address is removed, medical content is kept — Result

## Submit the record ✓ passed (31.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (33.4s)
md5: ee0866338ed26cd130f1520d82035772
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text '512 Copperleaf Terrace'.
Verify that the Result region does not contain the text 'Hartwell Glen'.
Verify that the Result region does not contain the text '97099'.
