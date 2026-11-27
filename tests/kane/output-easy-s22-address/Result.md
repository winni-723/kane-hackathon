---
test: ../easy-s22-address_test.md
status: passed
started: 2026-11-26T22:40:51.703Z
duration_s: 61
session_id: 7dcd2c3f-488b-4eaf-b77d-a20ead3122e7
---

# easy-s22: address is removed, medical content is kept — Result

## Submit the record ✓ passed (25.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (31.7s)
md5: f76f6b8e3bb646b96abbcfb405d3cad4
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text '106 Corkery Stravenue'.
Verify that the Result region does not contain the text 'Unit 7'.
Verify that the Result region does not contain the text 'Salem'.
Verify that the Result region does not contain the text '01907'.
