Page Abstraction

The layer that lets database treat storage as fixed size blocks instead of dealing directly with individual bytes on the disk.

It will be split into the following classes:

1. PageHeader: the label at the top of each sheet
The first 32 bytes of every page say what the page is: its number, what kind of data it holds (records, image bytes, embeddings), and a checksum. A checksum is a number computed from the page's contents; if the page gets corrupted, the number stops matching and you catch it. The header is a plain struct, meaning just fields laid out in memory with no hidden extras. Because of that, the bytes in memory are exactly the bytes on disk, and you can save or load it by copying bytes. A static_assert is a compile-time check. Here it makes the build fail if the header ever stops being exactly 32 bytes.

2. FileManager: the one who touches the actual file
It opens the file, reads page N, writes page N, and forces the data onto the physical disk. Opening a file gives you a file descriptor, a handle number from the OS that you must close when done. F_FULLFSYNC is the Apple call that truly flushes data to storage, so a power cut can't lose it. Nothing else in the engine talks to the file directly.

3. BufferPool: the desk that holds sheets you're working on
Reading from disk is slow, so recently used pages are kept in memory. Each memory slot that holds one page is a frame. The pool has a fixed number of frames, which is how you cap memory on an iPhone. When it's full, it evicts a page nobody is using, writing it back to disk first if it was changed.

4. PageGuard: your claim ticket on a sheet
When code asks the pool for a page, it gets a guard back. While the guard exists, the page is pinned: the pool promises not to evict it, so the memory can't disappear under you. When the guard is destroyed, the page is automatically unpinned. This pattern is called RAII: a resource is acquired when an object is created and released automatically when the object goes away, so you can't forget to clean up. The guard is move-only. You can hand the ticket to someone else, but you can't photocopy it, so there are never two owners each thinking they should unpin.

5. SlottedPage, BlobPage, EmbeddingPage: reading glasses for each kind of sheet
A page is just bytes. These classes know how to interpret those bytes for one kind of page. For example, SlottedPage knows "the slot list is at the front, records are packed at the back." Each one is a view: it doesn't own the page, it borrows the guard and gives you nice methods like insert() and get().

Why not a Page base class with subclasses?

In classic OOP you'd write class SlottedPage : public Page with virtual functions, which are methods chosen at runtime based on the object's real type. To do that, C++ secretly stores a hidden pointer (the vtable pointer) inside each object. That pointer is a memory address from the current run of the program, so it's meaningless once written to disk and would corrupt your page layout. The header already records the page type, so you just check it and use the matching view.


