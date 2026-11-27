---
test: ../medium-04-address_test.md
status: passed
started: 2026-11-26T23:37:07.276Z
duration_s: 66
session_id: 2ce1ffce-fcc9-45c6-8249-919c18f15ef2
---

# medium-04: address is removed, medical content is kept — Result

## Submit the record ✓ passed (25.2s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted address is gone ✓ passed (36.5s)
md5: 07d86723acc299807bb1dcad73445d8c
Verify that the Result region contains the text '81-year-old man'.
Verify that the Result region contains the text 'community-acquired pneumonia'.
Verify that the Result region contains the text 'Ceftriaxone 1 g IV daily'.
Verify that the Result region contains the text 'Azithromycin 500 mg daily'.
Verify that the Result region contains the text 'Amoxicillin-Clavulanate 875 mg twice daily for 5 more days'.
Verify that the Result region contains the text 'Follow-up chest X-ray in 6 weeks'.
Verify that the Result region does not contain the text '740 Sycamore Hollow Road'.
Verify that the Result region does not contain the text 'Graniteville Park'.
Verify that the Result region does not contain the text '19199'.
