# visionDB
A Rust and C++ native database engine optimized for computer vision workloads.

Modern computer-vision workloads require images, tensors, embeddings, annotations, and structured metadata to be managed as one logical dataset. Existing storage systems commonly optimize either sequential analytical scans or low-latency random access, creating read amplification, excess I/O, and fragmented data pipelines when applications need both training scans & individual images or tensor retrieval [1], [2]. Computer-vision retrieval also requires queries that combine vector similarity with metadata predicates, yet the most effective execution strategy changes with filter selectivity, vector–attribute correlation, index type, and target recall, making static pre-filtering or post-filtering plans unreliable across workloads [3].

The problem becomes more complex as datasets evolve through ingestion, relabeling, augmentation, deletion, and embedding-model changes. Each modification must update the underlying object, metadata, dataset version, and vector index while preserving transactional consistency. Dataset snapshots must remain reproducible and queryable without duplicating every image or tensor, while dynamic vector indexes must support frequent insertions and deletions without requiring expensive full-index reconstruction or causing unacceptable degradation in search latency and recall [4], [5]. 

**The core engineering problem is to design an ACID compliant database engine for computer-vision workloads that provides a unified multimodal storage layout, fast random image and tensor access, filtered vector query planning with alertness for costs, transactionally consistent dataset versioning, and online vector-index maintenance. The engine must preserve atomicity between raw data, metadata, annotations, embeddings, and index state while supporting reproducible snapshots, continuous updates, predictable query latency, and high vector-search recall without maintaining separate databases or rebuilding indexes after routine dataset changes.**

## References

[1] Gandhi, S., et al. “The Tensor Data Platform: Towards an AI-Centric Database System.” *Conference on Innovative Data Systems Research*, 2023.

[2] Pace, W., She, C., Xu, L., Jones, W., Lockett, A., Wang, J., and Shah, R. “Lance: Efficient Random Access in Columnar Storage through Adaptive Structural Encodings.” *arXiv preprint arXiv:2504.15247*, 2025.

[3] Chronis, Y., Caminal, H., Papakonstantinou, Y., Özcan, F., and Ailamaki, A. “Filtered Vector Search: State-of-the-Art and Research Opportunities.” *Proceedings of the VLDB Endowment*, vol. 18, no. 12, pp. 5488–5492, 2025. doi:10.14778/3750601.3750700. 

[4] Huang, S., Xu, L., Liu, J., Elmore, A. J., and Parameswaran, A. “ORPHEUSDB: Bolt-on Versioning for Relational Databases.” *Proceedings of the VLDB Endowment*, vol. 10, no. 10, pp. 1130–1141, 2017. 

[5] Zhong, S., Mo, D., and Luo, S. “LSM-VEC: A Large-Scale Disk-Based System for Dynamic Vector Search.” *arXiv preprint arXiv:2505.17152*, 2025.

[1]: https://www.vldb.org/cidrdb/papers/2023/p68-gandhi.pdf " The Tensor Data Platform: Towards an AI-centric Database System"
[2]: https://www.vldb.org/cidrdb/papers/2023/p68-gandhi.pdf?utm_source=chatgpt.com "The Tensor Data Platform: Towards an AI-centric Database System"
[3]: https://arxiv.org/abs/2504.15247?utm_source=chatgpt.com "Lance: Efficient Random Access in Columnar Storage through Adaptive Structural Encodings"
[4]: https://www.vldb.org/pvldb/vol18/p5488-caminal.pdf?utm_source=chatgpt.com "https://www.vldb.org/pvldb/vol18/p5488-caminal.pdf"
[5]: https://arxiv.org/abs/2505.17152?utm_source=chatgpt.com "LSM-VEC: A Large-Scale Disk-Based System for Dynamic Vector Search"

