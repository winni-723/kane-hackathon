# Extraction answer keys for the held-out records

One file per record in `tests/heldout/`, written to the same rules as
`tests/extraction/README.md`. Approved by the project owner on 2026-11-15,
after the first, untuned run and before any extraction rule was changed for
these records. Never edited to make a test pass.

First run, untuned (2026-11-15): **25/40 cells right** (age 8/10, sex 8/10,
diagnoses 4/10, medications 5/10). Every wrong cell was an empty or
incomplete one; none held a wrong value. That is the honest measure of how
the extraction carries over to records it was not written against. Scores
after that date are tuned on these records and are not a held-out measure.

Measured only, never gated.
