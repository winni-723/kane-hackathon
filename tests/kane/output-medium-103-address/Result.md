---
test: ../medium-103-address_test.md
status: passed
started: 2026-11-26T23:19:46.496Z
duration_s: 65
session_id: 52d9cd88-8c59-41ea-978f-61518940dbdb
---

# medium-103: address is removed, medical content is kept — Result

## Submit the record ✓ passed (24.1s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (37.2s)
md5: af5f065e5b2c66dd626dfde798cb70f5
Verify that the Result region contains the text 'Clarithromycin 500 mg twice daily'.
Verify that the Result region contains the text 'H. pylori'.
Verify that the Result region contains the text 'Simvastatin 40 mg nightly'.
Verify that the Result region contains the text 'until the antibiotic course ends in 14 days'.
Verify that the Result region contains the text 'Amlodipine 10 mg daily'.
Verify that the Result region does not contain the text '6 Kukui Nut Way'.
Verify that the Result region does not contain the text 'Unit 204'.
Verify that the Result region does not contain the text 'Hanalei Ridge'.
Verify that the Result region does not contain the text '96799'.
