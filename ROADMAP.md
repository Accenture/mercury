# Technology roadmap for 2019

## Language Packs

The first language pack has been released with v1.11.38 for Python.

Upcoming language packs are Node.js and Go. 

To enable PolyGlot development, you can run the "language-connector" service application as a "side-car" so that language packs can use a very thin pipe to connect to the platform-core and let the side-car to do the heavy lifting.

Each language pack would provide the following functionality:
- In-memory event stream for private functions
- Registration of public and private functions
- Persistent socket connection to the "language-connector" sidecar

All the communication patterns (RPC, async, callback, pipeline, streaming and broadcast) are supported in language packs.
