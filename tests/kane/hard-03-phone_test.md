---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, hard]
variables:
  record:
    value: |-
      Oncology Follow-up

      J. R. Abernathy-Cole (goes by June) returns after cycle 3 of ABVD chemotherapy for Hodgkin lymphoma, stage IIA. She tolerated treatment well apart from grade 1 nausea managed with Ondansetron 8 mg as needed.

      PET scan shows a Deauville score of 2, consistent with complete metabolic response. Continue Doxorubicin-based regimen for one more cycle. June mentioned she has moved in with her partner Rhea in the blue duplex behind St. Brendan's church in Calloway Flats. Best contact is now Rhea's line: 5 5 5 - 0 1 0 9. Born 22 Nov 1991. Old chart no. was 4471-208-93.
---

# hard-03: phone is removed, medical content is kept

Generated from tests/fixtures/hard-03.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'cycle 3 of ABVD chemotherapy'.
Verify that the Result region contains the text 'Hodgkin lymphoma, stage IIA'.
Verify that the Result region contains the text 'Ondansetron 8 mg as needed'.
Verify that the Result region contains the text 'Deauville score of 2'.
Verify that the Result region contains the text 'complete metabolic response'.
Verify that the Result region contains the text 'Doxorubicin-based regimen'.
Verify that the Result region does not contain the text '5 5 5 - 0 1 0 9'.
