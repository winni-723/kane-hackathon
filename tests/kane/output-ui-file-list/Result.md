---
test: ../ui-file-list_test.md
status: passed
started: 2026-11-26T04:51:18.038Z
duration_s: 209
session_id: 7a13e314-773f-4314-87e9-41cc903fad37
---

# Loaded files are listed, an unreadable one says why and can be removed — Result

## Load three sample files ✓ passed (65.9s)
md5: 89da7c6a1be37071956859d5e27b596e
Select 'Patient list (CSV)' in the sample file menu and click the Load sample file button.
Select 'Clinic note (Word)' in the sample file menu and click the Load sample file button.
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the Loaded files region, taken as a whole, contains the text 'sample-clinic-note.docx'.
Verify that the Loaded files region, taken as a whole, contains the text 'Scanned documents cannot be read here'.

## Remove the scanned file ✓ passed (28.2s)
md5: fbaf6bcbe49f6f043d11f3147f847cbe
Click the 'Remove sample-scanned-fax.pdf' button.
Verify that the page contains the text '6 records loaded'.

## Build the research table ✓ passed (23.4s)
md5: a28b7837f972474ccda708944cbdb4a9
Click the Process button.
Verify that the Research table heading is visible.

## The table holds both readable files and no file names ✓ passed (27.9s)
md5: 0fb6b7df9e1ddbc294669e4ecb9d881c
Verify that the Research table region, taken as a whole, contains the text 'Hashimoto thyroiditis'.
Verify that the Research table region, taken as a whole, contains the text 'Type 2 diabetes mellitus'.
Verify that the Research table region, taken as a whole, contains the text 'Record 6'.
Verify that the Research table region, taken as a whole, does not contain the text 'sample-clinic-note'.
Verify that the Research table region, taken as a whole, does not contain the text 'sample-patient-list'.
