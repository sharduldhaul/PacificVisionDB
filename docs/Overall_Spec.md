# Overall Specifications


Build Pacific Vision database as one embedded database engine written in C++, Rust, and Metal that stores images, embeddings, tensors, annotations, and metadata as a single dataset.

Focus on Apple devices, specifically iPhones to address memory constraints. 

### C++ responsibilities
Storage core which contains pages, buffer pool, mmap, slotted pages, segment reader and writer, the F_FULLFSYNC durability path. 

### Metal Responsibilities
Distance computation over embedding columns the IVF probe scan, tombstone bitmap filtering.

### Rust responsibilities
Manifest and catalog, compaction policy, ingest tool, and benchmark harness.


## Architecture

### Storage Layer
Written in C++
1. Page: 
Fixed size block, 16KB, the unit of every read and write. Makes IO size predictable and lets the buffer pool account memory in whole units.
2. Page Header: 
3. File Manager
4. Buffer Pool

### Data Layout
Written in C++ and usage hints for metal.
1. Slotted page
2. Blog store
3. Embedding column
4. Segment
5. Segment Index
6. Tombstone set

### Version Control
1. Manifest
2. Catalog
3. Snapshot

### Maintenance
1. Ingest path
2. Compaction
3. Durability path

### Query
1. Query executor
2. Filter strategy selector
3. Compute kernels

### Surface
1. Public API
2. Benchmark Harness
