---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [name, medium]
variables:
  record:
    value: |-
      Progress Note

      Grace Whitfield is a 72-year-old woman seen today for follow-up of atrial fibrillation. She was born on April 9, 1953 and lives alone at 18 Larkspur Lane in Dunmore Heights. Her daughter Mary called the clinic from 555-0198 this morning to ask whether the Warfarin dose should change.

      INR today is 2.4, within the target range. Continue Warfarin 5 mg daily and Metoprolol 25 mg twice daily. No bleeding or bruising reported. Recheck INR in 4 weeks. If Grace cannot be reached, call her at (508) 555-0114.
---

# medium-01: name is removed, medical content is kept

Generated from tests/fixtures/medium-01.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone
Verify that the Result region contains the text '72-year-old woman'.
Verify that the Result region contains the text 'atrial fibrillation'.
Verify that the Result region contains the text 'INR today is 2.4'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region contains the text 'Metoprolol 25 mg twice daily'.
Verify that the Result region contains the text 'Recheck INR in 4 weeks'.
Verify that the Result region does not contain the text 'Grace Whitfield'.
Verify that the Result region does not contain the text 'Grace'.
