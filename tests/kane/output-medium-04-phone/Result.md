---
test: ../medium-04-phone_test.md
status: passed
started: 2026-11-14T03:33:54.282Z
duration_s: 76
session_id: 833312da-8683-48fe-a2e3-69eff99c7e73
---

# medium-04: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (34s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (37.2s)
md5: c9bff4055856b6695ccb36343c7761ff
Verify that the Result region contains the text '81-year-old man'.
Verify that the Result region contains the text 'community-acquired pneumonia'.
Verify that the Result region contains the text 'Ceftriaxone 1 g IV daily'.
Verify that the Result region contains the text 'Azithromycin 500 mg daily'.
Verify that the Result region contains the text 'Amoxicillin-Clavulanate 875 mg twice daily for 5 more days'.
Verify that the Result region contains the text 'Follow-up chest X-ray in 6 weeks'.
Verify that the Result region does not contain the text '(215) 555-0186'.
