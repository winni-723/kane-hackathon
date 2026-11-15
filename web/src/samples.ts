// Invented records, copied from tests/fixtures, so a visitor can try the page
// without having records of their own. Each one states the patient's age and
// sex, so the research table has no empty cells.

// tests/fixtures/medium-01.json
export const SAMPLE_RECORD = `Progress Note

Grace Whitfield is a 72-year-old woman seen today for follow-up of atrial fibrillation. She was born on April 9, 1953 and lives alone at 18 Larkspur Lane in Dunmore Heights. Her daughter Mary called the clinic from 555-0198 this morning to ask whether the Warfarin dose should change.

INR today is 2.4, within the target range. Continue Warfarin 5 mg daily and Metoprolol 25 mg twice daily. No bleeding or bruising reported. Recheck INR in 4 weeks. If Grace cannot be reached, call her at (508) 555-0114.`;

// tests/fixtures/medium-04.json
const DISCHARGE_SUMMARY = `Discharge Summary

This 81-year-old man, Bernard Kowalczyk, was admitted with community-acquired pneumonia. He was treated with Ceftriaxone 1 g IV daily and Azithromycin 500 mg daily, and oxygen was weaned to room air by day 3.

Bernard will be discharged to the home of his son, Peter Kowalczyk, at 740 Sycamore Hollow Road, Graniteville Park, PA 19199. Peter's phone is (215) 555-0186. Visiting nurse services should use the patient's social security number 987-65-4329 for enrollment. Continue Amoxicillin-Clavulanate 875 mg twice daily for 5 more days. Follow-up chest X-ray in 6 weeks.`;

// tests/fixtures/hard-04.json
const PSYCHIATRY_INTAKE = `Psychiatry Intake

Pt is a 34 y/o veteran, first name Malik, last name spelled O-S-E-I, self-referred for nightmares and hypervigilance since deployment. He prefers to be called Mal. Lives in the trailer park off Route 9 just past the Dunmore grain silo with his mother, Mrs. Osei.

PCL-5 score is 52. Diagnosis: post-traumatic stress disorder. Start Sertraline 50 mg daily and Prazosin 1 mg at bedtime for nightmares. Columbia Suicide Severity Rating Scale negative for current ideation. Safety plan completed. Mother's landline: (518) 555 0118. He gave his birthday as March 3rd, '91 and his SSN as 000 12 3456.`;

export const SAMPLE_BATCH: string[] = [SAMPLE_RECORD, DISCHARGE_SUMMARY, PSYCHIATRY_INTAKE];
