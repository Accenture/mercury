# Contributing to the Mercury framework

Thanks for taking the time to contribute!

The following is a set of guidelines for contributing to Mercury and its packages, which are hosted
in the [Accenture Organization](https://github.com/accenture) on GitHub. These are mostly
guidelines, not rules. Use your best judgment, and feel free to propose changes to this document
in a pull request.

## Code of Conduct

This project and everyone participating in it is governed by our
[Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.
Please report unacceptable behavior to Kevin Bader, who is the current project maintainer.

## What should I know before I get started?

We follow the [standard GitHub workflow](https://guides.github.com/introduction/flow/).
Before submitting a Pull Request:

- Please write tests.
- Make sure you run all tests and check for warnings.
- Think about whether it makes sense to document the change in some way. For smaller, internal changes, inline documentation might be sufficient (moduledoc), while more visible ones might warrant a change to the developer's guide (TBD) or the [README](./README.md).
- Update `CHANGELOG.md` file with your current change in form of `[Type of change e.g. Config, Kafka, .etc] Short description what it is all about - [#NUMBER](link to issue or pull request)`, and choose a suitable section (i.e., changed, added, fixed, removed, deprecated).

### Design Decisions

When we make a significant decision in how to write code, or how to maintain the project and
what we can or cannot support, we will document it using
[Architecture Decision Records (ADR)](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions).
Take a look at the [doc/architecture/decisions](docs/architecture/decisions/) directory for
existings ADRs. If you have a question around how we do things, check to see if it is documented
there. If it is *not* documented there, please ask us - chances are you're not the only one
wondering. Of course, also feel free to challenge the decisions by starting a discussion on the
mailing list.
