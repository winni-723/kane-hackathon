---
test: ../medium-101-dob_test.md
status: passed
started: 2026-11-26T23:15:03.106Z
duration_s: 60
session_id: 34b69546-e0f4-4729-9783-c49ce3c94491
---

# medium-101: dob is removed, medical content is kept — Result

## Submit the record ✓ passed (22.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone ✓ passed (33.1s)
md5: 4c920200228690b03d9cf2623125d35b
Verify that the Result region contains the text '34 weeks gestation'.
Verify that the Result region contains the text 'BP 162/104'.
Verify that the Result region contains the text 'Urine protein 2+'.
Verify that the Result region contains the text 'Fetal heart rate 148'.
Verify that the Result region contains the text 'Preeclampsia with severe features'.
Verify that the Result region contains the text 'Magnesium sulfate 4 g IV loading dose'.
Verify that the Result region contains the text 'Labetalol 20 mg IV'.
Verify that the Result region contains the text 'Betamethasone 12 mg IM'.
Verify that the Result region does not contain the text '11/30/1986'.
