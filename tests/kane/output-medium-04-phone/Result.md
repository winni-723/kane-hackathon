---
test: ../medium-04-phone_test.md
status: passed
started: 2026-11-26T23:07:05.896Z
duration_s: 57
session_id: 944e886d-056e-47c5-94e1-173ac493bd07
---

# medium-04: phone is removed, medical content is kept — Result

## Submit the record ✓ passed (21.4s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone ✓ passed (31.6s)
md5: c9bff4055856b6695ccb36343c7761ff
Verify that the Result region contains the text '81-year-old man'.
Verify that the Result region contains the text 'community-acquired pneumonia'.
Verify that the Result region contains the text 'Ceftriaxone 1 g IV daily'.
Verify that the Result region contains the text 'Azithromycin 500 mg daily'.
Verify that the Result region contains the text 'Amoxicillin-Clavulanate 875 mg twice daily for 5 more days'.
Verify that the Result region contains the text 'Follow-up chest X-ray in 6 weeks'.
Verify that the Result region does not contain the text '(215) 555-0186'.
