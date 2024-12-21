package main

import "testing"

// Checking for a value that is in the array
func TestBinarySearchPasses(t *testing.T){
  array := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17}
  target := 14

  got := BinarySearch(array, target)
  want := true

  if got != want {
    t.Errorf("got %t, wanted %t", got, want)
  }
}

// Checking for a value that isn't in the array
func TestBinarySearchFails(t *testing.T){
  array := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17}
  target := 54

  got := BinarySearch(array, target)
  want := false

  if got != want {
    t.Errorf("got %t, wanted %t", got, want)
  }
}
