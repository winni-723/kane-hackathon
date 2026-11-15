---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, easy]
variables:
  record:
    value: |-
      REFERRAL
      Re: Mr. Kwame Asante-Boateng
      Date of birth: 15 August 1952
      Contact number: 614 555 0193
      Residence: 2040 Old Mill Pike, Wexley Township, OH 43099
      Health plan no.: 7731-0092-6

      Reason for referral: Progressive exertional dyspnea and bilateral leg swelling.
      Findings: Ejection fraction 35% on echocardiogram; BNP 880 pg/mL.
      Current treatment: Carvedilol 6.25 mg twice daily, Spironolactone 25 mg daily.
      Request: Evaluation for implantable cardioverter-defibrillator.
---

# easy-103: address is removed, medical content is kept

Generated from tests/heldout/easy-103.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'Progressive exertional dyspnea'.
Verify that the Result region contains the text 'Ejection fraction 35%'.
Verify that the Result region contains the text 'BNP 880 pg/mL'.
Verify that the Result region contains the text 'Carvedilol 6.25 mg twice daily'.
Verify that the Result region contains the text 'Spironolactone 25 mg daily'.
Verify that the Result region contains the text 'implantable cardioverter-defibrillator'.
Verify that the Result region does not contain the text '2040 Old Mill Pike'.
Verify that the Result region does not contain the text 'Wexley Township'.
Verify that the Result region does not contain the text '43099'.
