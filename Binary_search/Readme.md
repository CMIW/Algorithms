
# [Binary Search](https://en.wikipedia.org/wiki/Binary_search_algorithm)

<div style="text-align: justify">Binary Search compares the target value to the middle element of the array. If they are not equal, the half in which the target cannot lie is eliminated and the search continues on the remaining half, again taking the middle element to compare to the target value, and repeating this until the target value is found. If the search ends with the remaining half being empty, the target is not in the array.</div>

```
binary_search(a, v)
  l = 0
  r = a.length - 1
  while l <= r do
    m = (l + r) / 2
    if a[m] < v then
      l = m + 1
    else if a[m] > v then
      r = m - 1
    else
      return m
  return unsuccessful
``` 
