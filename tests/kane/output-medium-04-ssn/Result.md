---
test: ../medium-04-ssn_test.md
status: passed
started: 2026-11-26T23:08:11.526Z
duration_s: 58
session_id: f996f104-393a-4297-b435-12cdd21b695c
---

# medium-04: ssn is removed, medical content is kept — Result

## Submit the record ✓ passed (23.3s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted ssn is gone ✓ passed (30.3s)
md5: ecd21cffb5ad7ab1cfb7232120c3c1b5
Verify that the Result region contains the text '81-year-old man'.
Verify that the Result region contains the text 'community-acquired pneumonia'.
Verify that the Result region contains the text 'Ceftriaxone 1 g IV daily'.
Verify that the Result region contains the text 'Azithromycin 500 mg daily'.
Verify that the Result region contains the text 'Amoxicillin-Clavulanate 875 mg twice daily for 5 more days'.
Verify that the Result region contains the text 'Follow-up chest X-ray in 6 weeks'.
Verify that the Result region does not contain the text '987-65-4329'.
