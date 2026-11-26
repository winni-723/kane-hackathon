---
test: ../ui-start-over_test.md
status: passed
started: 2026-11-25T22:11:23.634Z
duration_s: 101
session_id: b0b8dc47-01c3-4a9b-9eb3-fc771a944b58
---

# Start over on the Result goes back to an empty input — Result

## Submit the first record ✓ passed (27.8s)
md5: 329c7f0624911041d37009769e03489e
Type {{first}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## Start over ✓ passed (20s)
md5: fe514eff9af8c21312460ac5c75f643f
Click the Start over button in the Result region.
Verify that the Upload records heading is visible.

## Submit a second record ✓ passed (26s)
md5: 3b048b3788ad1c44c6c4fe0415d38244
Type {{second}} into the medical record text box and click the Process button.
Verify that the Result heading is visible.

## The Result holds only the second record ✓ passed (22.6s)
md5: 0594cd8205f427c2dfb107ae5c8bf981
Verify that the Result region contains the text 'Seasonal allergic rhinitis'.
Verify that the Result region does not contain the text 'Migraine without aura'.
Verify that the Result region does not contain the text '(413) 555-0177'.
