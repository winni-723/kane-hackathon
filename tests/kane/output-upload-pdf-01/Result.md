---
test: ../upload-pdf-01_test.md
status: passed
started: 2026-11-18T03:38:29.376Z
duration_s: 232
session_id: 995ed07f-14f8-439f-9a83-a7ddf9bb7130
---

# upload-pdf-01: a PDF file is de-identified, medical content is kept — Result

## Load the sample file ✓ passed (42.5s)
md5: e9c2c6177a1e643cf355fd950b9c60d4
Select 'Lab report (PDF)' in the sample file menu and click the Load sample file button.
Verify that the medical record text box contains the text 'Comprehensive Metabolic Panel'.

## Process it ✓ passed (27.7s)
md5: 167407626c87811fbdb2115478dd6038
Click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted identifiers are gone ✓ passed (107.6s)
md5: 4bb876808143c62bacae5145d7f68a17
Verify that the Result region contains the text 'Comprehensive Metabolic Panel'.
Verify that the Result region contains the text 'Serum, fasting'.
Verify that the Result region contains the text '168 mg/dL'.
Verify that the Result region contains the text '1.12 mg/dL'.
Verify that the Result region contains the text '58 mL/min/1.73m2'.
Verify that the Result region contains the text '4.6 mmol/L'.
Verify that the Result region contains the text 'Hemoglobin A1c'.
Verify that the Result region contains the text '7.9 %'.
Verify that the Result region contains the text '221 mg/dL'.
Verify that the Result region contains the text '142 mg/dL'.
Verify that the Result region contains the text '176 mg/dL'.
Verify that the Result region contains the text 'type 2 diabetes mellitus'.
Verify that the Result region contains the text 'stage 3a chronic kidney disease'.
Verify that the Result region does not contain the text 'DELGADO'.
Verify that the Result region does not contain the text 'TERESA'.
Verify that the Result region does not contain the text '02/09/1958'.
Verify that the Result region does not contain the text '4417760'.
Verify that the Result region does not contain the text '775-555-0142'.
Verify that the Result region does not contain the text '903 Linden Way'.
Verify that the Result region does not contain the text 'Cobalt Springs'.
Verify that the Result region does not contain the text '89077'.
Verify that the Result region does not contain the text 'XHP448210937'.
Verify that the Result region does not contain the text 'Kwame Asante'.
Verify that the Result region does not contain the text 'Asante'.
Verify that the Result region does not contain the text 'Whitcombe'.
Verify that the Result region does not contain the text 'Hannah Lindgren'.
Verify that the Result region does not contain the text 'Brightwater Regional Laboratory'.
Verify that the Result region does not contain the text '45 Quarry Road'.
Verify that the Result region does not contain the text '775-555-0190'.
Verify that the Result region does not contain the text '775-555-0191'.
Verify that the Result region does not contain the text '11/02/2026'.
Verify that the Result region does not contain the text '11/04/2026'.
Verify that the Result region does not contain the text 'BRL-26-0918842'.
Verify that the Result region does not contain the text '29D0998877'.
