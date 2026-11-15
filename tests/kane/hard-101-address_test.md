---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, hard]
variables:
  record:
    value: |-
      ED Provider Note

      47M BIBA after witnessed tonic-clonic seizure at work. Coworker states pt, who everyone calls Deke, has been off his meds for a week. Wallet ID reads DeKalb R. Whitmore III. Postictal on arrival, GCS 13 improving to 15.

      Glucose 96, sodium 138. CT head negative. Levetiracetam level undetectable. Loaded with Levetiracetam 1500 mg IV. Hx of traumatic brain injury per chart. Wife is out of state; reached his sister-in-law Marisol on her mobile, 4 1 2 - 5 5 5 - 0 1 6 2. She says the Whitmores rent the upstairs unit of the brick two-flat next to the Polish bakery on Carrow Street. Birth date on license: 12.07.1978.
---

# hard-101: address is removed, medical content is kept

Generated from tests/heldout/hard-101.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'witnessed tonic-clonic seizure'.
Verify that the Result region contains the text 'GCS 13 improving to 15'.
Verify that the Result region contains the text 'Glucose 96, sodium 138'.
Verify that the Result region contains the text 'CT head negative'.
Verify that the Result region contains the text 'Levetiracetam 1500 mg IV'.
Verify that the Result region contains the text 'traumatic brain injury'.
Verify that the Result region does not contain the text 'upstairs unit of the brick two-flat next to the Polish bakery on Carrow Street'.
