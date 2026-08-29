# [Sparse Set](https://en.wikipedia.org/wiki/Set_(abstract_data_type))

<div style="text-align: justify">A sparse set provides `O(1)` insertion, `O(1)` removal, and `O(1)` membership checks over a set of integer ids, by keeping two arrays in sync: `sparse` indexes into `dense` by id, while `dense` packs the live ids together, and `payload` holds the associated value for each id. Backing up the removed element with the last element of `dense` keeps the pack dense with a swap, so iteration visits the live ids in insertion order without touching the empty slots.</div>

```
sparse_set
  insert(id, value)
    if id is out of range, resize sparse
    push id to dense
    push value to payload
    sparse[id] = payload.length - 1

  remove(id)
    dense_idx = sparse[id]
    if missing, return none
    swap dense[dense_idx] with dense[last]
    swap payload[dense_idx] with payload[last]
    sparse[dense[dense_idx]] = dense_idx
    pop dense and payload

  contains(id)
    sparse[id] has a value?
```
