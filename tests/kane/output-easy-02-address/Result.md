---
test: ../easy-02-address_test.md
status: passed
started: 2026-11-26T22:19:14.189Z
duration_s: 64
session_id: 6dc6c2d8-e373-4d50-99a8-9c0336b7d6e4
---

# easy-02: address is removed, medical content is kept — Result

## Submit the record ✓ passed (21.6s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (37.3s)
md5: 526fe6dceefe89d0211b285582ad8f45
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'HbA1c 7.8%'.
Verify that the Result region contains the text '152 mg/dL'.
Verify that the Result region contains the text 'Metformin 1000 mg twice daily'.
Verify that the Result region contains the text 'Empagliflozin 10 mg daily'.
Verify that the Result region does not contain the text '1260 Juniper Court'.
Verify that the Result region does not contain the text 'Apt 4B'.
Verify that the Result region does not contain the text 'Alder Creek'.
Verify that the Result region does not contain the text '94016'.
