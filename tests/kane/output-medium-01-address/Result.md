---
test: ../medium-01-address_test.md
status: passed
started: 2026-11-14T04:13:59.039Z
duration_s: 83
session_id: dc59abdc-1452-4b28-ab24-e10ca6744e16
---

# medium-01: address is removed, medical content is kept — Result

## Submit the record ✓ passed (33.9s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (43.8s)
md5: 2d75512dc1f4f6de352f5bbfeb14da7d
Verify that the Result region contains the text '72-year-old woman'.
Verify that the Result region contains the text 'atrial fibrillation'.
Verify that the Result region contains the text 'INR today is 2.4'.
Verify that the Result region contains the text 'Warfarin 5 mg daily'.
Verify that the Result region contains the text 'Metoprolol 25 mg twice daily'.
Verify that the Result region contains the text 'Recheck INR in 4 weeks'.
Verify that the Result region does not contain the text '18 Larkspur Lane'.
Verify that the Result region does not contain the text 'Dunmore Heights'.
