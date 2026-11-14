---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, hard]
variables:
  record:
    value: |-
      Psychiatry Intake

      Pt is a 34 y/o veteran, first name Malik, last name spelled O-S-E-I, self-referred for nightmares and hypervigilance since deployment. He prefers to be called Mal. Lives in the trailer park off Route 9 just past the Dunmore grain silo with his mother, Mrs. Osei.

      PCL-5 score is 52. Diagnosis: post-traumatic stress disorder. Start Sertraline 50 mg daily and Prazosin 1 mg at bedtime for nightmares. Columbia Suicide Severity Rating Scale negative for current ideation. Safety plan completed. Mother's landline: (518) 555 0118. He gave his birthday as March 3rd, '91 and his SSN as 000 12 3456.
---

# hard-04: phone is removed, medical content is kept

Generated from tests/fixtures/hard-04.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text '34 y/o veteran'.
Verify that the Result region contains the text 'nightmares and hypervigilance'.
Verify that the Result region contains the text 'PCL-5 score is 52'.
Verify that the Result region contains the text 'post-traumatic stress disorder'.
Verify that the Result region contains the text 'Sertraline 50 mg daily'.
Verify that the Result region contains the text 'Prazosin 1 mg at bedtime'.
Verify that the Result region contains the text 'Columbia Suicide Severity Rating Scale'.
Verify that the Result region does not contain the text '(518) 555 0118'.
