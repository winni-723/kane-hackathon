---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [dob, hard]
variables:
  record:
    value: |-
      Neurology Consult

      Referred by the family physician for evaluation of memory loss. The patient, a retired schoolteacher whom staff know as Dottie (legal name Dorothea M. Vance), scored 19 out of 30 on the Montreal Cognitive Assessment. D. Vance's son reports she got lost walking back to her apartment over the pharmacy on Kessler Street.

      Impression: probable Alzheimer's disease, mild stage. MRI shows hippocampal atrophy without evidence of Lewy body pathology. Start Donepezil 5 mg nightly. Discussed driving safety with the son, Mr. Vance, reachable at area code 617, 555-0156. DOB: the fourth of July, 1944.
---

# hard-02: dob is removed, medical content is kept

Generated from tests/fixtures/hard-02.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted dob is gone
Verify that the Result region contains the text 'evaluation of memory loss'.
Verify that the Result region contains the text '19 out of 30'.
Verify that the Result region contains the text 'Montreal Cognitive Assessment'.
Verify that the Result region contains the text "probable Alzheimer's disease".
Verify that the Result region contains the text 'hippocampal atrophy'.
Verify that the Result region contains the text 'Lewy body pathology'.
Verify that the Result region contains the text 'Donepezil 5 mg nightly'.
Verify that the Result region does not contain the text 'the fourth of July, 1944'.
