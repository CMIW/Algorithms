# Algorithms Study Plan

## Classic Algorithms Checklist

### Sorting
- [x] Insertion Sort
- [x] Quick Sort
- [ ] Merge Sort
- [ ] Heap Sort
- [ ] Counting Sort
- [ ] Radix Sort

### Searching
- [x] Binary Search
- [ ] Linear Search
- [ ] Depth-First Search (DFS)
- [ ] Breadth-First Search (BFS)

### Graph Algorithms
- [ ] Floyd-Warshall *(folder exists, no implementation)*
- [ ] Dijkstra
- [ ] Bellman-Ford
- [ ] Kruskal (Minimum Spanning Tree)
- [ ] Prim (Minimum Spanning Tree)
- [ ] Topological Sort

### Data Structures
- [ ] Stack
- [ ] Queue
- [ ] Linked List
- [ ] Binary Tree / BST
- [ ] Heap / Priority Queue
- [ ] Trie
- [ ] Union Find (Disjoint Set)

### Design Patterns
- [x] Builder
- [x] Factory Method

---

## Base Algorithm Techniques Checklist

These are the problem-solving patterns that underlie most LeetCode problems. Distinct from classic algorithms — these are *techniques* you apply at the code level.

### Array / String Techniques
- [ ] Two Pointers
- [ ] Sliding Window (fixed size)
- [ ] Sliding Window (variable size)
- [ ] Prefix Sum
- [ ] Kadane's Algorithm (max subarray)

### Hashing
- [ ] Frequency Map (count occurrences)
- [ ] Hash Set for O(1) lookup
- [ ] Bucket Sort / Pigeonhole

### Recursion & Combinatorics
- [ ] Recursion with backtracking
- [ ] Subset generation
- [ ] Permutation generation
- [ ] Pruning

### Divide & Conquer
- [ ] Merge Sort (canonical D&C)
- [ ] Binary search on answer (parametric search)

### Dynamic Programming
- [ ] Top-down (memoization)
- [ ] Bottom-up (tabulation)
- [ ] 1D state DP
- [ ] 2D state DP
- [ ] Interval DP

### Graph Traversal
- [ ] DFS (iterative + recursive)
- [ ] BFS (level-order)
- [ ] Topological sort (Kahn's algorithm)
- [ ] Cycle detection (directed / undirected)

### Greedy
- [ ] Interval scheduling / greedy selection
- [ ] Greedy with sorting

### Tree-Specific
- [ ] DFS on trees (pre/in/post-order)
- [ ] BFS on trees (level-order)
- [ ] Path problems (root-to-leaf)

### Advanced Structures
- [ ] Monotonic Stack
- [ ] Monotonic Queue (deque)
- [ ] Union Find with path compression + rank
- [ ] Segment Tree (range queries)

---

## LeetCode Patterns Coverage

### Arrays & Hashing
*Base algorithms: Hash Map, Hash Set, Prefix/Suffix Arrays, Bucket Sort*

- [x] Contains Duplicates
- [x] Valid Anagram
- [x] Two Sum
- [x] Group Anagrams
- [x] Top K Frequent Elements
- [x] Product of Array Except Self
- [x] Valid Sudoku
- [x] Longest Consecutive Sequence
- [ ] Encode and Decode Strings

### Two Pointers
*Base algorithm: Two Pointers*

- [x] Valid Palindrome
- [x] Container With Most Water
- [x] Trapping Rain Water
- [ ] Two Sum II (sorted input)
- [ ] 3Sum

### Sliding Window
*Base algorithm: Sliding Window — **not started***

- [ ] Best Time to Buy and Sell Stock
- [ ] Longest Substring Without Repeating Characters
- [ ] Longest Repeating Character Replacement
- [ ] Minimum Window Substring
- [ ] Sliding Window Maximum

### Stack
*Base algorithm: Stack, Monotonic Stack — **not started***

- [ ] Valid Parentheses
- [ ] Min Stack
- [ ] Evaluate Reverse Polish Notation
- [ ] Generate Parentheses
- [ ] Daily Temperatures
- [ ] Car Fleet
- [ ] Largest Rectangle in Histogram

### Binary Search
*Base algorithm: Binary Search (classic done, LeetCode variants missing)*

- [ ] Search in Rotated Sorted Array
- [ ] Find Minimum in Rotated Sorted Array
- [ ] Koko Eating Bananas
- [ ] Search a 2D Matrix

### Linked List
*Base data structure: Linked List — **not started***

- [ ] Reverse Linked List
- [ ] Merge Two Sorted Lists
- [ ] Linked List Cycle
- [ ] Reorder List
- [ ] Remove Nth Node From End
- [ ] Copy List with Random Pointer
- [ ] LRU Cache

### Trees
*Base data structure: Binary Tree / BST — **not started***

- [ ] Invert Binary Tree
- [ ] Maximum Depth of Binary Tree
- [ ] Diameter of Binary Tree
- [ ] Same Tree
- [ ] Subtree of Another Tree
- [ ] Level Order Traversal (BFS)
- [ ] Lowest Common Ancestor of BST
- [ ] Validate BST
- [ ] Kth Smallest Element in BST
- [ ] Construct Tree from Preorder/Inorder

### Tries
*Base data structure: Trie — **not started***

- [ ] Implement Trie
- [ ] Design Add and Search Words
- [ ] Word Search II

### Heap / Priority Queue
*Base data structure: Heap — partially covered via Top K Frequent*

- [ ] Kth Largest Element in Stream
- [ ] K Closest Points to Origin
- [ ] Kth Largest Element in Array
- [ ] Task Scheduler
- [ ] Median from Data Stream

### Backtracking
*Base algorithm: Recursion / Backtracking — **not started***

- [ ] Subsets
- [ ] Combination Sum
- [ ] Permutations
- [ ] Word Search
- [ ] Palindrome Partitioning
- [ ] N-Queens
- [ ] Letter Combinations of Phone Number

### Graphs
*Base algorithms: BFS, DFS, Union Find — **not started***

- [ ] Number of Islands
- [ ] Clone Graph
- [ ] Max Area of Island
- [ ] Pacific Atlantic Water Flow
- [ ] Surrounded Regions
- [ ] Rotting Oranges
- [ ] Course Schedule (topological sort)
- [ ] Course Schedule II
- [ ] Redundant Connection (Union Find)
- [ ] Number of Connected Components (Union Find)
- [ ] Word Ladder

### Advanced Graphs
*Base algorithms: Dijkstra, Bellman-Ford, Kruskal, Prim*

- [ ] Reconstruct Itinerary (Eulerian path)
- [ ] Min Cost to Connect All Points (Prim/Kruskal)
- [ ] Network Delay Time (Dijkstra)
- [ ] Swim in Rising Water (Dijkstra)
- [ ] Cheapest Flights Within K Stops (Bellman-Ford)

### 1D Dynamic Programming
*Base algorithm: DP — **not started***

- [ ] Climbing Stairs
- [ ] Min Cost Climbing Stairs
- [ ] House Robber
- [ ] House Robber II
- [ ] Longest Palindromic Substring
- [ ] Palindromic Substrings
- [ ] Coin Change
- [ ] Maximum Product Subarray
- [ ] Word Break
- [ ] Longest Increasing Subsequence
- [ ] Partition Equal Subset Sum

### 2D Dynamic Programming

- [ ] Unique Paths
- [ ] Longest Common Subsequence
- [ ] Best Time to Buy/Sell Stock with Cooldown
- [ ] Coin Change II
- [ ] Edit Distance
- [ ] Distinct Subsequences
- [ ] Interleaving String
- [ ] Regular Expression Matching

### Greedy

- [ ] Maximum Subarray (Kadane's Algorithm)
- [ ] Jump Game
- [ ] Jump Game II
- [ ] Gas Station
- [ ] Hand of Straights
- [ ] Merge Triplets to Form Target

### Intervals

- [ ] Insert Interval
- [ ] Merge Intervals
- [ ] Non-Overlapping Intervals
- [ ] Meeting Rooms
- [ ] Meeting Rooms II
- [ ] Minimum Interval to Include Each Query

### Math & Geometry

- [ ] Rotate Image
- [ ] Spiral Matrix
- [ ] Set Matrix Zeroes
- [ ] Happy Number
- [ ] Plus One
- [ ] Pow(x, n)

### Bit Manipulation

- [ ] Single Number
- [ ] Number of 1 Bits
- [ ] Counting Bits
- [ ] Reverse Bits
- [ ] Missing Number

---

## Suggested Study Order

1. **Sliding Window** — biggest gap relative to what you've covered
2. **Stack / Monotonic Stack**
3. **Binary Search variants**
4. **Linked List**
5. **Trees (BFS/DFS first, then BST)**
6. **Heap / Priority Queue**
7. **Backtracking**
8. **Graphs**
9. **1D DP**
10. **Greedy, Intervals, 2D DP, Advanced Graphs**
