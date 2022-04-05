
# [Insertion Sort](https://en.wikipedia.org/wiki/Insertion_sort)

```
insertion_sort (a)
  for i in range 1 .. array.length
    j = i
    while j > 0 and a[j] < a[j - 1]
      temp = a[j]
      a[j] = a[j -1]
      a[i - 1] = temp
    j -= 1
```
