---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, hard]
variables:
  record:
    value: |-
      Nephrology Clinic

      Seen with interpreter. Patient's name is recorded two ways in our system: NGUYEN, Thi Lan and Lan T. Nguyen. Stage 4 chronic kidney disease secondary to IgA nephropathy; eGFR is 22 mL/min. Blood pressure 148/92 despite Losartan 100 mg daily.

      Add Amlodipine 10 mg daily. Discussed dialysis access planning and referred for arteriovenous fistula evaluation. Kayexalate not required; potassium is 4.9. Her nephew Bao translates at home and asked for results by email at bao.ng88@example.com or on his cell, 555.0177 (same area code as the clinic). They live three doors down from the Lotus Garden restaurant on Ferrow Avenue. D.O.B. 1962-05-27.
---

# hard-05: phone is removed, medical content is kept

Generated from tests/fixtures/hard-05.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'Stage 4 chronic kidney disease'.
Verify that the Result region contains the text 'IgA nephropathy'.
Verify that the Result region contains the text 'eGFR is 22 mL/min'.
Verify that the Result region contains the text 'Blood pressure 148/92'.
Verify that the Result region contains the text 'Losartan 100 mg daily'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region contains the text 'arteriovenous fistula evaluation'.
Verify that the Result region contains the text 'potassium is 4.9'.
Verify that the Result region does not contain the text '555.0177'.
