---
mode: testing
url: http://127.0.0.1:3000
headless: true
tags: [phone, easy]
variables:
  record:
    value: |-
      PATIENT RECORD
      Patient Name: Priya Raman
      DOB: 07/29/1989
      Home Phone: (312) 555-0165
      Mobile: (312) 555-0190
      Address: 77 Harbor Lane, Fenwick Hollow, IL 60699
      Emergency Contact: Anil Raman (husband), (312) 555-0133

      Chief Complaint: Severe unilateral headache with nausea and photophobia.
      History: Episodes occur twice monthly, each lasting 6 to 12 hours.
      Diagnosis: Migraine without aura.
      Medications: Sumatriptan 50 mg as needed; Propranolol 40 mg twice daily for prophylaxis.
      Plan: Headache diary, avoid known triggers.
---

# easy-03: phone is removed, medical content is kept

Generated from tests/fixtures/easy-03.json by scripts/gen-kane-test.mjs.
Do not edit: this test is part of the regression suite.

## Submit the record
Type {{record}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Medical content is kept and planted phone is gone
Verify that the Result region contains the text 'Severe unilateral headache'.
Verify that the Result region contains the text 'photophobia'.
Verify that the Result region contains the text '6 to 12 hours'.
Verify that the Result region contains the text 'Migraine without aura'.
Verify that the Result region contains the text 'Sumatriptan 50 mg as needed'.
Verify that the Result region contains the text 'Propranolol 40 mg twice daily'.
Verify that the Result region does not contain the text '(312) 555-0165'.
Verify that the Result region does not contain the text '(312) 555-0190'.
Verify that the Result region does not contain the text '(312) 555-0133'.
