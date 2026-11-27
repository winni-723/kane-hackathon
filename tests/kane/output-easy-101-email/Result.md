---
test: ../easy-101-email_test.md
status: passed
started: 2026-11-26T22:32:22.200Z
duration_s: 53
session_id: 7932adc2-0a6e-42cc-a656-a2991fadf2d7
---

# easy-101: email is removed, medical content is kept — Result

## Submit the record ✓ passed (23.7s)
md5: 7ef29b9c7a260041bed51782ba9120f0
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted email is gone ✓ passed (26.1s)
md5: 8f4bedd18684e3590d058e0662fb87ed
Verify that the Result region contains the text 'Burning epigastric pain after meals'.
Verify that the Result region contains the text 'Gastroesophageal reflux disease'.
Verify that the Result region contains the text 'Omeprazole 20 mg daily before breakfast for 8 weeks'.
Verify that the Result region contains the text 'elevate head of bed'.
Verify that the Result region does not contain the text 'oliver.brennan@example.org'.
