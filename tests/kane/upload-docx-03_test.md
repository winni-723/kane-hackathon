---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [upload, docx, hard]
---

# upload-docx-03: a Word file is de-identified, medical content is kept

Generated from tests/documents/keys/docx-03.json by scripts/gen-kane-upload-test.mjs.
Do not edit: this test is part of the regression suite.

## Load the sample file
Select 'Clinic note (Word)' in the sample file menu and click the Load sample file button.
Verify that the medical record text box contains the text '35-year-old woman'.

## Process it
Click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted identifiers are gone
Verify that the Result region contains the text '35-year-old woman'.
Verify that the Result region contains the text 'Hashimoto thyroiditis'.
Verify that the Result region contains the text 'Graves disease'.
Verify that the Result region contains the text 'fatigue and cold intolerance'.
Verify that the Result region contains the text 'weight up 4 lb'.
Verify that the Result region contains the text 'sleeping 10 hours a night'.
Verify that the Result region contains the text '6.8 mIU/L'.
Verify that the Result region contains the text '0.4-4.0'.
Verify that the Result region contains the text '0.8 ng/dL'.
Verify that the Result region contains the text '0.8-1.8'.
Verify that the Result region contains the text '412 IU/mL'.
Verify that the Result region contains the text '75 mcg'.
Verify that the Result region contains the text '88 mcg daily'.
Verify that the Result region contains the text 'Recheck TSH in 6 weeks'.
Verify that the Result region contains the text 'Vitamin D 2000 IU daily'.
Verify that the Result region does not contain the text 'NAKAMURA'.
Verify that the Result region does not contain the text 'Yuki'.
Verify that the Result region does not contain the text 'Jordan Ellis'.
Verify that the Result region does not contain the text 'Jordan'.
Verify that the Result region does not contain the text '1991-06-14'.
Verify that the Result region does not contain the text '06/14/1991'.
Verify that the Result region does not contain the text '7730415'.
Verify that the Result region does not contain the text '415.555.0133'.
Verify that the Result region does not contain the text '(415) 555-0159'.
Verify that the Result region does not contain the text 'yuki.nakamura@example.com'.
Verify that the Result region does not contain the text '58 Alder Court'.
Verify that the Result region does not contain the text 'Redfern'.
Verify that the Result region does not contain the text '94061'.
Verify that the Result region does not contain the text '11/03/2026'.
Verify that the Result region does not contain the text '10/27/2026'.
Verify that the Result region does not contain the text 'Priya Raman'.
Verify that the Result region does not contain the text 'Redfern Community Health Clinic'.
Verify that the Result region does not contain the text '1200 Mission Plaza'.
Verify that the Result region does not contain the text '415-555-0100'.
Verify that the Result region does not contain the text 'L-2026-448817'.
Verify that the Result region does not contain the text 'fdesk02'.
