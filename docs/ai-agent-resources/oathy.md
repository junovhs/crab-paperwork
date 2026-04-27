SWEAR TO EACH OF THESE 5 CLAUSES ONE BY ONE, RECITING THE CLAUSE BEFORE YOU SWEAR TO IT:

**Documentation**: Swear that you checked if the issue you worked on touched any aspect of the documentation, and that if it did, you updated the documentation to be aligned with your new change.

**Scope honesty**: Swear you explicitly checked whether the issue was properly scoped before coding. State whether the issue had one concrete change, one main code surface, and one clear proof of done. Swear you did not quietly narrow the issue in your head, do one convenient slice, and then claim the full issue was DONE. If the issue was too broad, swear you said so plainly, decomposed it through `ishoo`, and did not mark the parent DONE prematurely.

**Tests**: Swear the tests you added/changed for this issue are legitimate and non-bullshit: they would fail without the fix, they validate the intended behavior (not just “no panic” / smoke), and they include meaningful coverage (at least one real negative/edge case when applicable). Swear you did not write tautological tests that simply mirror the implementation.

**Issue hygiene**: Swear you updated this issue through the project's current canonical workflow with a comprehensive Resolution (what changed, why, how verified, commands run) and updated its status. Use `ishoo`. If the project is mid-migration, swear you followed the current canonical tracking rules. Answer "Should I update the readme?". Also state whether the issue was executable as written or had to be split first.

**Good Science Clause**: Swear the concrete behavior the change should cause, state what you predicted you would observe, then compare the actual result of your real world behavioral test to the prediction and say whether it matched. Also explain how you did a real world test on real code and examined the output. Do not hide behind abstract/unit tests when the intended behavior is user-facing, repo-facing, or about real-world signal quality.

If you can’t swear to every clause, that is ok! Just say exactly what’s missing and fix it before claiming DONE :)
