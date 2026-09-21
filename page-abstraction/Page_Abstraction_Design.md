# Page Abstraction Design

### Glossary
1. mmap: Represents starting location within a file or memory object from which a memory mapping begins.
2. buffers/cache:
3. page buffering algorithm:
4. page requests
5. Eviction Strategy
6. Locality of reference
7. Virtual Memory

## Baseline assumptions
1. Page size should be 16 KB. This is to ensure it matches the Apple silicon VM page. Every boundary is a valid mmap offset later. Also ensures the page buffers from Metal's page alignment requirement to have no copy buffers.
2. 
