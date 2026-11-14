---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [name, medium]
variables:
  record:
    value: |-
      Discharge Summary

      This 81-year-old man, Bernard Kowalczyk, was admitted with community-acquired pneumonia. He was treated with Ceftriaxone 1 g IV daily and Azithromycin 500 mg daily, and oxygen was weaned to room air by day 3.

      Bernard will be discharged to the home of his son, Peter Kowalczyk, at 740 Sycamore Hollow Road, Graniteville Park, PA 19199. Peter's phone is (215) 555-0186. Visiting nurse services should use the patient's social security number 987-65-4329 for enrollment. Continue Amoxicillin-Clavulanate 875 mg twice daily for 5 more days. Follow-up chest X-ray in 6 weeks.
---

# medium-04: name is removed, medical content is kept

Generated from tests/fixtures/medium-04.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone
Verify that the Result region contains the text '81-year-old man'.
Verify that the Result region contains the text 'community-acquired pneumonia'.
Verify that the Result region contains the text 'Ceftriaxone 1 g IV daily'.
Verify that the Result region contains the text 'Azithromycin 500 mg daily'.
Verify that the Result region contains the text 'Amoxicillin-Clavulanate 875 mg twice daily for 5 more days'.
Verify that the Result region contains the text 'Follow-up chest X-ray in 6 weeks'.
Verify that the Result region does not contain the text 'Bernard Kowalczyk'.
Verify that the Result region does not contain the text 'Bernard'.
