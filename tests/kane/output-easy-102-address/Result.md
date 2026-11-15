---
test: ../easy-102-address_test.md
status: passed
started: 2026-11-15T17:42:03.073Z
duration_s: 73
session_id: 07ad47a2-cd42-485c-8ebf-d1dc866a6670
---

# easy-102: address is removed, medical content is kept — Result

## Submit the record ✓ passed (27.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (38.9s)
md5: d454541fac3a790466b5603003969c53
Verify that the Result region contains the text 'CBC with differential'.
Verify that the Result region contains the text 'Fatigue and heavy menstrual bleeding for 3 months'.
Verify that the Result region contains the text 'Hemoglobin 9.8 g/dL'.
Verify that the Result region contains the text 'ferritin 6 ng/mL'.
Verify that the Result region contains the text 'Iron deficiency anemia'.
Verify that the Result region contains the text 'Ferrous sulfate 325 mg twice daily'.
Verify that the Result region does not contain the text '88 Rue Cypress Lane'.
Verify that the Result region does not contain the text 'Apt 3C'.
Verify that the Result region does not contain the text 'Bellefontaine Bayou'.
Verify that the Result region does not contain the text '70099'.
