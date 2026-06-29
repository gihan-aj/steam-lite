# How to Debug Unfamiliar Complex Systems

A guide to the mental process experienced engineers use to understand and fix
bugs in code they didn't write, in systems they've never seen before.

The examples in this document are drawn directly from the `watch::Sender::send()`
bug we just fixed in this project — a real case that is a good template for this
kind of thinking.

---

## The Core Principle

> **The goal of debugging is not to fix the code. It is to understand what actually happened.**
> The fix is trivial once you truly understand it.

Beginners tend to do the opposite: they look at code, guess at a fix, apply it, and check
if it "works". This is gambling, not debugging. It produces code you don't understand, and
the bug often comes back in a different form.

Experienced engineers spend most of their debugging time *understanding*, and very little
time *changing code*. When they finally touch a file, they usually change 1–3 lines, and
they can explain exactly why each one matters.

---

## Step 1 — Read the Symptoms, Not the Code

When you encounter a bug report, your instinct might be to immediately open the code.
**Resist this.** The code is what you'll read second.

Read the symptoms first:
- What does the user see? (behavior)
- What do the logs say? (facts)
- When does it happen? (conditions)
- When does it NOT happen? (counter-examples)

In our bug:

```
SYMPTOMS:
- After pause → resume, the crawl starts but immediately stops again
- This only happens on the second resume, not the first
- The logs show "Rate-limit wait complete" followed IMMEDIATELY by "Crawl stop requested"
- No user-visible stop action was taken between those two lines
```

The fact that it only happens on the SECOND resume is critical information. That tells you
something is accumulating across cycles — not a one-time error. If you had jumped straight
into the code, you might have missed this and fixed the wrong thing.

---

## Step 2 — Build a Timeline of Facts (Not Assumptions)

Before forming any hypothesis, write down what you know happened in order. Use only
observable facts — log lines, timestamps, user actions. Label each fact with whether
it is **certain** (you saw it in a log) or **inferred** (you deduced it from code).

```
TIMELINE (all facts are from logs):
  t=01:48:02  stop_crawl called → "Stop signal sent to crawl"           [CERTAIN]
  t=01:48:02  task exits       → "Stop signal during page wait"         [CERTAIN]
  t=01:48:09  start_crawl called → "Aborting previous crawl task"       [CERTAIN]
  t=01:48:09  new task spawns  → "Starting catalogue crawl task"        [CERTAIN]
  t=01:48:09  rate-limit wait  → "Resuming — rate limit wait 50s"       [CERTAIN]
  t=01:48:59  sleep completes  → "Rate-limit wait complete"             [CERTAIN]
  t=01:48:59  task stops       → "Crawl stop requested"                 [CERTAIN]
              (no user action between 01:48:09 and 01:48:59)            [CERTAIN]
```

Notice: "no user action" is also a fact. The absence of a log line is data.

---

## Step 3 — Name the Broken Invariant

Every bug is a violated invariant. An **invariant** is a condition that *should always be
true* at a specific point in the program. Naming it precisely is the most important step —
it converts a vague "it doesn't work" into a specific falsifiable claim.

Format: **"At [point in execution], [variable/state] should be [expected value], but was [actual value]."**

In our bug:
> "When the new crawl task first checks `stop_rx.borrow()` at the top of the main loop,
> the watch channel value should be `false` (because `start_crawl` sends `false` before
> spawning). But the task is acting as if the value is `true`."

This immediately transforms the question from "why does the crawl stop?" to a much
narrower: **"What can cause `stop_rx.borrow()` to return `true` even though `send(false)`
was called before the task started?"**

That is a completely different, and much more solvable, question.

---

## Step 4 — Layer the System and Locate the Break

Complex systems are made of layers. Bugs exist in exactly one layer. If you can identify
which layer contains the broken invariant, you can ignore everything else.

Draw your layers for this system:

```
Layer 5: UI / React state      (liveProgress, useState, re-renders)
Layer 4: TanStack Query cache  (invalidateQueries, setQueryData)
Layer 3: Tauri IPC             (invoke, emit, listen)
Layer 2: Rust async runtime    (Tokio tasks, select!, watch channel)
Layer 1: Standard library      (tokio::sync::watch contract)
```

Our invariant is about `stop_rx.borrow()` returning an unexpected value. That is a
**Layer 2 / Layer 1** problem — it has nothing to do with React, TanStack, or the Tauri
IPC. You can stop reading UI code completely.

This is how experienced engineers avoid getting lost — they eliminate layers early.

**How to identify which layer:**

| If the symptom is... | Suspect layer... |
|---|---|
| UI showing wrong data | React state (useState, props) |
| Data that's stale/cached | TanStack Query staleTime, caching |
| Command not responding at all | Tauri registration (generate_handler!) |
| Wrong data crossing the bridge | Serialization / type mismatch |
| Task not stopping / racing | Rust async, channels, Mutex |
| Crash in Rust | Rust safety / lifetime / unwrap |
| Library doing unexpected thing | Library contract / version |

---

## Step 5 — Form Hypotheses as "IF → THEN" Statements

A hypothesis must be testable. The format is:

> "IF [specific mechanism X is true] THEN [observable effect Y would occur]."

Then you check whether your observed facts match Y.

**Bad hypothesis (unfalsifiable):** "Something is going wrong with the channel."

**Good hypothesis:** "IF `send(false)` was called after `subscribe()` instead of before,
THEN the receiver's initial value would be `true` (the old value), and `borrow()` would
return `true` immediately."

Does this match observations? Partially — it would explain the immediate stop. But it
doesn't explain why the FIRST resume works. So this hypothesis is incomplete or wrong.

You keep generating hypotheses and discarding ones that don't fit ALL the facts, not just
the primary symptom. This is the part that takes patience.

**The hypothesis that fit everything:**

> "IF `watch::Sender::send()` silently fails when there are no receivers, and IF the task
> drops its receiver when it exits, THEN after the task finishes, `send(false)` in
> `start_crawl` would not update the channel, leaving it at `true` from the previous
> `stop_crawl`. The new receiver from `subscribe()` would then start with value `true`."

Check against ALL observations:
- Why does it stop immediately after the first resume? ✅ Channel was `true`.
- Why does the FIRST start work? ✅ Channel was initialized to `false` and never
  successfully changed to `true` before — the first `stop_crawl` sets it to `true` but
  the first `start_crawl` fails to set it back. Wait — why did the first resume work then?

Hmm. Check again: the first resume after app open works because at app startup the channel
is created with `watch::channel(false)`, so the value IS `false` without any `send()` call.
The first `send(false)` in `start_crawl` fails (no receiver), but the value is already `false`.
It's only after the first `stop_crawl` that the channel gets stuck at `true`. ✅

Every observation is explained. Hypothesis confirmed.

---

## Step 6 — Read the Library Contract, Not Just the Usage Examples

This is the step most beginners skip and most experienced engineers don't.

Every library function has a **contract** — a precise description of what it does,
what it requires, and what it guarantees. Usage examples in tutorials and StackOverflow
answers often show the *happy path* and omit the edge cases.

In our case, the key was reading this in the Tokio docs:

> `watch::Sender::send()` returns `Err(SendError)` **if there are no active receivers**.
> When it returns `Err`, **the value is NOT updated**.

This is not intuitive. Most channel-like APIs just send and forget. But `watch` requires
at least one receiver to be alive, otherwise the send is a no-op that returns an error we
silently discarded with `let _ = ...`.

**How to find the relevant contract:**

1. When you have a hypothesis involving a specific function, look up that function in the
   official documentation — not examples, the actual API reference.
2. Look for: "Returns", "Panics", "Errors", "Notes", "Platform-specific behavior".
3. Ask yourself: "Does my code handle ALL the return variants?" (`Ok`, `Err`, `None`, etc.)
4. Look at what happens in the failure case — many bugs are in unhandled error paths.

`let _ = some_function()` is a red flag. It means "I'm ignoring the result." Every time
you see that in unfamiliar code, ask: "What does this function return on failure, and
does ignoring it cause problems?"

---

## Step 7 — Verify the Fix Explains the Bug, Not Just Fixes It

Before applying your fix, state clearly:

1. What is the root cause?
2. Why does the fix address the root cause (not just mask the symptom)?
3. Does the fix break any other invariant?

In our case:

> **Root cause:** `send()` fails silently when no receivers exist.
>
> **Fix:** Use `send_replace()` instead, which always updates the value regardless
> of receivers.
>
> **Why it works:** `send_replace()` guarantees the channel holds `false` when
> `subscribe()` is called, so the new receiver correctly starts with `false`.
>
> **Does it break anything?** No — `send_replace()` also notifies any existing
> receivers (the current task), which is the desired behavior.

If you cannot answer question 2 clearly, you don't understand the bug yet.
A fix you don't understand is a time bomb — it works today and breaks in a different
way next month when someone changes something else.

---

## Step 8 — The Debugging Anti-Patterns to Avoid

These are the habits that waste hours:

### "Thrashing" — applying random fixes
Changing things one by one and running to see if it's fixed. This occasionally
works by accident, but you learn nothing and the bug often returns. Worse: you
introduce new bugs while thrashing.

**Signs you're thrashing:** "Let me just try changing this and see..."

**The fix:** Stop. Go back to Step 3. Name the broken invariant.

---

### "Cargo cult debugging" — copying a fix without understanding it
You find a Stack Overflow answer that looks similar, copy the code, and it works.
But you don't know why. Next time a variation of the same bug appears, you're back
to square one.

**Signs you're cargo-culting:** You can't explain why the fix works. You find it by
searching "watch channel rust bug" instead of deriving it from your hypothesis.

**The fix:** After applying the fix, write one sentence explaining why it works.
If you can't, research until you can.

---

### "Wide net" reading — reading all the code
Opening every file in the system hoping to spot something wrong. This works sometimes
for trivial bugs but fails completely for subtle async/timing issues where the bug
isn't visible in any single place.

**Signs you're doing this:** You've read 500 lines and have no hypothesis yet.

**The fix:** Stop reading code. Go back to the logs/symptoms. Form a hypothesis first,
then read only the code that the hypothesis predicts should be wrong.

---

### "Symptom fixing" — fixing what you see, not what caused it
The UI shows a stale status after resume. You add a 2-second delay before reading the
status. The UI looks correct. Bug "fixed". But the underlying channel bug is still there,
and will manifest as a different symptom the next time.

**Signs you're doing this:** Your fix is a `setTimeout`, a retry loop, or a check for
the symptom rather than a change to what caused it.

**The fix:** Ask "why does this state get wrong?" not "how do I show the correct state
despite it being wrong?"

---

## The Mental Loop, Summarized

```
OBSERVE
  → Read logs, record facts, note what's absent

CONSTRAIN
  → Name the broken invariant precisely
  → Identify which layer it belongs to

HYPOTHESIZE
  → "IF [mechanism] THEN [observable]"
  → Does it explain ALL observations including counter-examples?

VERIFY
  → Read the relevant library/language contract
  → Does the contract confirm your hypothesis?
  → Does the fix address the root cause, not the symptom?

FIX
  → Change the minimum number of lines
  → Explain in one sentence why it works
```

---

## How to Build This Skill Over Time

### 1. When a bug is fixed, write down why it happened
The reflex to move on the moment something works is the enemy of learning.
Spend 5 minutes writing: "The bug was X because of Y. The fix was Z because it
addresses Y by doing W." This forces you to verify your own understanding.

### 2. Read library source code, not just docs
Documentation describes what should happen. Source code describes what actually happens.
For critical primitives (channels, mutexes, async runtimes), reading the source once
will save you days over your career. Tokio, sqlx, Tauri — all open source, all readable.

### 3. Practice reading logs as a story
Logs are a narrative of what happened. Train yourself to read them chronologically
and ask after each line: "What was true about the system state at this moment?"

### 4. Get comfortable saying "I don't know yet"
Beginners feel pressure to have an explanation quickly. Experienced engineers are
comfortable saying "I don't have a hypothesis yet, I need more data." Premature
explanations cause anchoring — you start reading code to confirm your guess instead
of to discover the truth.

### 5. The 20-minute rule
If you have been staring at a problem for 20 minutes and have no hypothesis,
STOP and do something different:
- Add more logging (get more facts)
- Write out a timeline (structure what you know)
- Explain the problem to someone else (rubber duck debugging — explaining forces clarity)
- Take a break (your background cognitive processes are real and useful)

Do not continue "looking at code" without a direction. That is not debugging, it is hoping.

---

## Applied to This Project's Bug — Summary Walkthrough

| Step | What we did |
|---|---|
| Read symptoms | "Stops immediately after resume. First resume works, second doesn't." |
| Build timeline | Wrote out 8 log lines with timestamps, noted no user action during the 50s gap |
| Name invariant | "`stop_rx.borrow()` should return `false` when the new task starts" |
| Layer | Rust async (Layer 2) / Tokio library (Layer 1) — no React or Tauri involvement |
| Hypotheses | Tested: wrong subscribe ordering, race between send and spawn, abort timing, borrow_and_update behavior — all failed to explain ALL facts |
| Library contract | Read Tokio docs: `send()` returns `Err` with no receivers, value NOT updated |
| Hypothesis confirmed | Task drops receiver → no receivers → `send(false)` fails → channel stays `true` → new task sees `true` → stops |
| Fix | `send_replace(false)` — unconditionally updates the value |
| Verify | "It works because `send_replace` guarantees the write regardless of receiver count, which is exactly the invariant we needed" |
