---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [name, hard]
variables:
  record:
    value: |-
      Geriatric Assessment

      Seen at the request of the family. The patient is the widow of the late Judge Aldous Fenwick and is still addressed by staff as Mrs. Judge Fenwick; her given name is Wilhelmina, though she answers to Billie. MMSE 21/30. Geriatric Depression Scale 9/15. Two falls in the past 3 months; orthostatic drop of 24 mmHg systolic.

      Medications reviewed: stop Diphenhydramine; reduce Amlodipine to 2.5 mg daily; continue Donepezil 10 mg nightly. Braden score 16. Her niece, Dr. Priscilla Fenwick-Araujo, holds power of attorney and prefers contact through her office line, 3 0 2 5 5 5 0 1 3 7, or p.fenwick.araujo@example.org. Billie has lived for forty years in the stone cottage at the end of Quaker Bridge Lane, past the covered bridge. Born on Christmas Eve, 1936.
---

# hard-103: name is removed, medical content is kept

Generated from tests/heldout/hard-103.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted name is gone
Verify that the Result region contains the text 'MMSE 21/30'.
Verify that the Result region contains the text 'Geriatric Depression Scale 9/15'.
Verify that the Result region contains the text 'Two falls in the past 3 months'.
Verify that the Result region contains the text 'orthostatic drop of 24 mmHg systolic'.
Verify that the Result region contains the text 'stop Diphenhydramine'.
Verify that the Result region contains the text 'Donepezil 10 mg nightly'.
Verify that the Result region contains the text 'Braden score 16'.
Verify that the Result region does not contain the text 'Mrs. Judge Fenwick'.
Verify that the Result region does not contain the text 'Wilhelmina'.
Verify that the Result region does not contain the text 'Billie'.
