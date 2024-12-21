package main

import "golang.org/x/exp/constraints"

func main() {

}

func BinarySearch[T constraints.Ordered](a []T, v T) bool{
  l := 0
  r := len(a) - 1
  for l <= r {
    m := (l + r) / 2
    if a[m] < v {
      l = m + 1
    } else if  a[m] > v {
      r = m - 1
    } else {
      return true
    }
  }
  return false
}
