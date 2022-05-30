# Technology roadmap for 2022

## Language Packs

The first language pack has been released with v1.11.38 for Python.

Our next language pack will be Node.js. It is code complete as of 5/30/2022. 
It will be released as soon as it passes our quality check.

To enable polyglot development, you can run the "language-connector" service application as a "side-car" 
so that language packs can use a very thin pipe to connect to the platform-core and the side-car 
will do the heavy lifting.

Each language pack would provide the following functionality:
- In-memory event stream for private functions
- Registration of public and private functions
- Persistent web socket connection to the "language-connector" cloud network proxy

All the communication patterns (RPC, async, callback, pipeline, streaming and broadcast) are supported 
in language packs.
