---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [address, hard]
variables:
  record:
    value: |-
      Patient Portal Message

      hi dr, its bao-tran again (Tran Thi Bao, you saw me tues). the rash came back after i stopped the prednisone taper. my husband minh thinks its the new detergent. pls call me not him - my number changed to five oh three, 555, zero one seven five.

      Staff note: Chart reviewed. Contact dermatitis, recurrent. Restart Prednisone 40 mg daily tapering over 12 days and add Triamcinolone 0.1% cream twice daily. Patch testing referral placed. Verified patient by birthday (tenth of October, 1983). She is staying with her cousin at the cousin's place above the nail salon across from Greeley Park until the 1st.
---

# hard-102: address is removed, medical content is kept

Generated from tests/heldout/hard-102.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone
Verify that the Result region contains the text 'prednisone taper'.
Verify that the Result region contains the text 'Contact dermatitis, recurrent'.
Verify that the Result region contains the text 'Prednisone 40 mg daily tapering over 12 days'.
Verify that the Result region contains the text 'Triamcinolone 0.1% cream twice daily'.
Verify that the Result region contains the text 'Patch testing referral placed'.
Verify that the Result region does not contain the text 'above the nail salon across from Greeley Park'.
