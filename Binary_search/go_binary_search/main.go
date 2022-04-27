package main

import "fmt"

func main() {
    fmt.Println("Hello, World this is Binary Search!")
    array := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17}
    target := 14

    fmt.Println(BinarySearch(array, target))
}

func BinarySearch(a []int, v int) bool{
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
