# [Top K Frequent Elements (Bounded Heap)](https://en.wikipedia.org/wiki/Most_frequent_k_items)

<div style="text-align: justify">To find the `k` most frequent elements out of `n` without fully sorting, stream the frequency counts through a heap capped at `k` elements and always evict the worst. The worst must sit on top of the heap, so the frequency comparison is inverted with `Reverse`; the element itself acts as the tie-break key, keeping eviction deterministic. Each push costs `O(log k)` and only the `k` best survive, so the total is `O(n log k)` instead of `O(n log n)`.</div>

```
bounded_top_k(values, k)
  count = map of element -> frequency

  heap = empty min-heap on frequency
  for each (entry, frequency) in count:
    heap.push((Reverse(frequency), entry))
    if heap.len() > k:
      heap.pop()          // evicts the worst (lowest frequency)

  return entries of heap, sorted by frequency descending
```
