
# [Quick Sort](https://en.wikipedia.org/wiki/Quicksort)

Quick sort with Hoare partition scheme

```
quick_sort(a, lo, hi)
  if lo >= 0 && hi >= 0 && lo < hi
    p = partition(a, lo, hi)
    quick_sort(a, lo, p)
    quick_sort(a, p + 1, hi)

partition(a, lo, hi)
  i = (lo + hi) / 2
  p = a[i]
  l = lo - 1
  r = hi + 1

  loop
    do
      l += 1
    while a[l] < pivot

    do
      r += 1
    while a[r] > pivot

    if l >= r
      return r

      // Swap a[l] with a[r]
      temp = a[l]
      a[l] = a[r]
      a[r] = temp
```
