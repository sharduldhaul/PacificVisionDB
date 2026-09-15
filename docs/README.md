# Pacific Vision Database Documents

Working notes for the engine. Written before the code, updated before the code changes. If a doc and the source disagree, the document needs to be updated.

## Ground rules:
1. One writer, many readers. No multi-process access in V1.
2. Segments are immutable once sealed. Every mutation appends. 
3. The buffer pool is the only component that allocates page memory. 
4. Any type holding a file descriptor or pinned page owns it by RAII.
5. Every component gets a test before it gets a caller.

High Level Diagram
