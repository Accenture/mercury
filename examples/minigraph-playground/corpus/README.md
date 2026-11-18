# The synthetic corpus

Four documents written for the `document-ingest` and `document-query` demo graphs: two agreements and two policies of
fictional companies, with the parties, obligations, rules and deadlines an analyst would extract into the agent's
knowledge graph. Nothing here is a real document, a real company or a real contract. Run the language pack's agent
tools with this folder as the workspace (`-Dtools.file.root=<path to this folder>`) and the graph reads a document by
its name, for example `supplier-agreement.md`.

Each document follows one shape so a reader - a person, a model or the test's scripted assistant - finds the same
facts: a header with the parties and the term, then the sections **Obligations**, **Rules** and **Deadlines**, one fact
per line with an identifier.
