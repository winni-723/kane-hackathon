---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [mrn, medium]
variables:
  record:
    value: |-
      ED Triage Note

      Patient is Hector Salazar, a 45-year-old man brought in by his brother Luis after two hours of crushing substernal chest pain radiating to the left arm. Luis can be reached at 213-555-0147 and says Hector lives with him at 902 Alameda Verde Avenue, Santa Rilla, CA.

      ECG shows ST elevation in leads II, III and aVF. Troponin I is 3.2 ng/mL. Diagnosis: acute inferior myocardial infarction. Aspirin 325 mg chewed, Heparin bolus given, cardiology paged for emergent catheterization. Hector's date of birth is 06/17/1980 and his record number is 55801276.
---

# medium-02: mrn is removed, medical content is kept

Generated from tests/fixtures/medium-02.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted mrn is gone
Verify that the Result region contains the text '45-year-old man'.
Verify that the Result region contains the text 'crushing substernal chest pain'.
Verify that the Result region contains the text 'ST elevation in leads II, III and aVF'.
Verify that the Result region contains the text 'Troponin I is 3.2 ng/mL'.
Verify that the Result region contains the text 'acute inferior myocardial infarction'.
Verify that the Result region contains the text 'Aspirin 325 mg chewed'.
Verify that the Result region does not contain the text '55801276'.
