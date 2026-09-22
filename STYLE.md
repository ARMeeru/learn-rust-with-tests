# Style guide

This book is written to be read, not consulted. The model is Learn Go with Tests: one developer
explaining a thing to another, in small steps, with an opinion when one is warranted.

## Voice

Write in first person plural for the work we do together: "we write the test", "we run it", "we
refactor". Switch to first person singular for a judgement: "I would not mock this". The reader
should always know whether they are following a step or hearing an opinion.

Use "you" only when addressing the reader about their own situation: "you may be wondering why we
test something this small". Never for the work itself. "You write the test" turns a shared
exercise into an instruction sheet.

Short sentences for steps. Longer ones for explanation. Vary them.

Say what the compiler said. When a step fails to compile, quote the error and then explain the one
line of it that matters.

## Punctuation and words

No em dashes or en dashes in prose. Use a comma, a period, a colon or parentheses. Inside code,
comments and quoted compiler output, leave punctuation exactly as it is. CI checks this with
`tools/prose-check`, which skips fenced blocks and inline code.

Straight quotes only.

Avoid the reflexes that make text sound generated. These are the ones a reviewer reads for, since
no script can catch them:

- Announcing a point before making it: "Let's dive in", "Here's the thing".
- Restating a paragraph in a one-line closer: "That is the real win."
- Contrasting with a position nobody holds: "this is not about X, it is about Y", when nobody
  said it was about X.
- Grouping in threes because three sounds complete. Keep three items when there are three.
- Decorative bold: a bold label on every list item, or bold on a whole sentence for emphasis.

Also avoid the words that arrive with those reflexes: robust, crucial, delve, leverage, seamless,
comprehensive, pivotal, showcase.

## Opinions

Some chapters argue a position: the refactoring checklist, why unit tests, anti-patterns, and any
place a tutorial chapter chooses one tool over another. Those passages follow a few rules.

- Evidence settles it. A position that cannot be tied to evidence or to a stated premise is a
  preference, and is labelled as one.
- Say how sure you are before the claim, and say what would change your mind.
- Check the premise before comparing options. If the premise falls, the comparison was never
  the decision.
- New tools go where failure is cheap. Load-bearing paths get boring, proven parts. This is why
  the fundamentals stay close to std and why the application section standardises on one runtime
  and one framework.
- The safe way to use a thing should be the easy way. Prefer designs where the default is
  correct and the dangerous option is an explicit opt-in visible at the call site.
- Build for decay. Dependencies age and owners leave. Prefer contracts and failures that make
  decay visible and gradual.

## Structure wins

Readability comes first from small steps and one idea per step. If personality and the chapter
template pull in different directions, follow the template.
