---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [relative_name, medium]
variables:
  record:
    value: |-
      S: Teresa Villanueva, 39, comes in with her mother-in-law Consuelo for a blood pressure check at 34 weeks gestation. She reports headaches and swelling of the hands since Sunday. Teresa can be reached on her work line, 915-555-0127, during the day.
      O: BP 162/104 on two readings. Urine protein 2+. Fetal heart rate 148.
      A: Preeclampsia with severe features.
      P: Admit to labor and delivery. Start Magnesium sulfate 4 g IV loading dose and Labetalol 20 mg IV. Betamethasone 12 mg IM for fetal lung maturity. Consuelo will bring the overnight bag from the apartment at 73 Mesa Dorada Court, Los Robles Viejos, TX 79999. Teresa's DOB is 11/30/1986; her chart is filed under 20994518.
---

# medium-101: relative name is removed, medical content is kept

Generated from tests/heldout/medium-101.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted relative name is gone
Verify that the Result region contains the text '34 weeks gestation'.
Verify that the Result region contains the text 'BP 162/104'.
Verify that the Result region contains the text 'Urine protein 2+'.
Verify that the Result region contains the text 'Fetal heart rate 148'.
Verify that the Result region contains the text 'Preeclampsia with severe features'.
Verify that the Result region contains the text 'Magnesium sulfate 4 g IV loading dose'.
Verify that the Result region contains the text 'Labetalol 20 mg IV'.
Verify that the Result region contains the text 'Betamethasone 12 mg IM'.
Verify that the Result region does not contain the text 'Consuelo'.
