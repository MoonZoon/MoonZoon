# Philosophy

**A)** We shouldn't have to learn many languages, tools and patterns to just start a simple web project.

**B)** No dogmas (Functions vs Objects, Mutable vs Immutable, Monolith vs Micro-services, etc.).

**C)** _Example-driven development_ - No API changes without simple examples based on real-world needs.

**D)** No special terminology. Let's try to use words like _portal_, _hook_, _atom_ or _selector_ as little as possible because they have different meanings in different frameworks.

**E)** Don't build artificial barriers for MoonZoon users - if they want to use REST, CSS or SQL, don't try to stop them.

**F)** Respect this order during the development:
   1. Architecture
   1. API
   1. Code

**G)** There is no silver bullet. However there should be a common denominator for most web projects.

**H)** There should be always a clear path forward for MoonZoon users. They will recognize when they need non-core tools and they will integrate them when needed, not before.

**I)** People make mistakes. Linters, compilers, formatters and other tools should be as pedantic as possible. Ideally no configuration options for anything.

**J)** People make things too complex. Breaking changes are encouraged to reduce accidental complexity and technical debt.

---

# Non-Goals

**a)** Satisfy everybody.

**b)** Make the framework super-universal, i.e. natively support all use-cases, protocols and external services.

**c)** Don't change API, goals and features in the future.

**d)** Set and meet deadlines. It's done when it's done.

**e)** Deliver all features at once. Don't expect to deploy scalable realtime apps on a stable cloud with the first MoonZoon release.

**f)** Beginner friendliness. It should be a side-effect of a well-designed API and good documentation.

**g)** The API optimized for the code writing. Readability and good error messages are the most important features.
