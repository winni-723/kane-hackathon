---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, medium]
variables:
  record:
    value: |-
      Clinic Note

      Ms. Yolanda Baptiste returns for management of rheumatoid arthritis. She reports morning stiffness lasting 90 minutes and swelling of both wrists. She works as a baker and says the pain makes kneading difficult.

      Current therapy is Methotrexate 15 mg weekly with Folic acid 1 mg daily. ESR is 42 mm/hr. Plan to add Hydroxychloroquine 200 mg twice daily. Yolanda asked us to coordinate with her sister Claudette, who drives her to appointments; Claudette's number is 504.555.0121. For the pharmacy, her date of birth is 10/03/1974 and her address is 2217 Magnolia Crest Boulevard, Bayou Verre, LA 70199.
---

# medium-05: address is removed, medical content is kept

Generated from tests/fixtures/medium-05.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'rheumatoid arthritis'.
Verify that the Result region contains the text 'morning stiffness lasting 90 minutes'.
Verify that the Result region contains the text 'Methotrexate 15 mg weekly'.
Verify that the Result region contains the text 'Folic acid 1 mg daily'.
Verify that the Result region contains the text 'ESR is 42 mm/hr'.
Verify that the Result region contains the text 'Hydroxychloroquine 200 mg twice daily'.
Verify that the Result region does not contain the text '2217 Magnolia Crest Boulevard'.
Verify that the Result region does not contain the text 'Bayou Verre'.
Verify that the Result region does not contain the text '70199'.
