---
test: ../medium-05-address_test.md
status: passed
started: 2026-11-26T23:38:01.698Z
duration_s: 62
session_id: a8d4f82e-80f5-4c15-acdc-7c00070e7eef
---

# medium-05: address is removed, medical content is kept — Result

## Submit the record ✓ passed (24.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (34.1s)
md5: 80af0953f35c1ff248eeb2f593d188eb
Verify that the Result region contains the text 'rheumatoid arthritis'.
Verify that the Result region contains the text 'morning stiffness lasting 90 minutes'.
Verify that the Result region contains the text 'Methotrexate 15 mg weekly'.
Verify that the Result region contains the text 'Folic acid 1 mg daily'.
Verify that the Result region contains the text 'ESR is 42 mm/hr'.
Verify that the Result region contains the text 'Hydroxychloroquine 200 mg twice daily'.
Verify that the Result region does not contain the text '2217 Magnolia Crest Boulevard'.
Verify that the Result region does not contain the text 'Bayou Verre'.
Verify that the Result region does not contain the text '70199'.
