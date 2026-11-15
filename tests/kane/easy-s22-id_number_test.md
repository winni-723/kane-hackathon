---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [id_number, easy]
variables:
  record:
    value: |-
      Jack Wisozk
      ===========
      Race:                White
      Ethnicity:           Non-Hispanic
      Gender:              M
      Age:                 21
      Birth Date:          2005-10-09
      Marital Status:      S
      SSN:                 999-86-1441
      Drivers:             S99913362
      Passport:            X65093286X
      Address:             106 Corkery Stravenue Unit 7
      City:                Salem
      State:               Massachusetts
      Zip:                 01907
      --------------------------------------------------------------------------------
      ALLERGIES:
      No Known Allergies
      --------------------------------------------------------------------------------
      CONDITIONS:
        2026-02-26 - 2026-05-15 : Concussion with no loss of consciousness (disorder)
        2026-02-26 - 2026-05-15 : Concussion injury of brain (disorder)
        2024-12-15 -            : Stress (finding)
        2024-12-15 -            : Social isolation (finding)
        2024-12-15 -            : Not in labor force (finding)
        2023-12-10 -            : Victim of intimate partner abuse (finding)
        2023-12-10 -            : Lack of access to transportation (finding)
        2023-12-10 - 2024-12-15 : Full-time employment (finding)
        2023-12-10 -            : Received higher education (finding)
        2023-05-04 - 2023-05-18 : Acute viral pharyngitis (disorder)
---

# easy-s22: id number is removed, medical content is kept

Generated from tests/synthea/easy-s22.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted id number is gone
Verify that the Result region contains the text 'Concussion with no loss of consciousness (disorder)'.
Verify that the Result region contains the text 'Concussion injury of brain (disorder)'.
Verify that the Result region contains the text 'Stress (finding)'.
Verify that the Result region does not contain the text 'S99913362'.
Verify that the Result region does not contain the text 'X65093286X'.
