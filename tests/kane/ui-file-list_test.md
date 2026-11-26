---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [ui, upload]
---

# Loaded files are listed, an unreadable one says why and can be removed

Hand-written UI test. Do not edit: this test is part of the regression suite.
The files are the page's fictional samples. Once one file is in the list,
each further file loaded joins it. File names are shown in the list only,
never in the research table.

## Load three sample files
Select 'Patient list (CSV)' in the sample file menu and click the Load sample file button.
Select 'Clinic note (Word)' in the sample file menu and click the Load sample file button.
Select 'Scanned fax (PDF, no text)' in the sample file menu and click the Load sample file button.
Verify that the Loaded files region, taken as a whole, contains the text 'sample-clinic-note.docx'.
Verify that the Loaded files region, taken as a whole, contains the text 'Scanned documents cannot be read here'.

## Remove the scanned file
Click the 'Remove sample-scanned-fax.pdf' button.
Verify that the page contains the text '6 records loaded'.

## Build the research table
Click the Process button.
Verify that the Research table heading is visible.

## The table holds both readable files and no file names
Verify that the Research table region, taken as a whole, contains the text 'Hashimoto thyroiditis'.
Verify that the Research table region, taken as a whole, contains the text 'Type 2 diabetes mellitus'.
Verify that the Research table region, taken as a whole, contains the text 'Record 6'.
Verify that the Research table region, taken as a whole, does not contain the text 'sample-clinic-note'.
Verify that the Research table region, taken as a whole, does not contain the text 'sample-patient-list'.
