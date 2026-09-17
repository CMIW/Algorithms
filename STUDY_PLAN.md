# Algorithms Study Plan

Focused on algorithms actually worth knowing: classic named algorithms, the cross-cutting techniques behind them, and LeetCode-style practice. Course material from *Investigación de Operaciones* is folded in where it maps onto real algorithm families (tagged `IO Semana N`).

---

## 1. Complexity & Analysis

- [ ] Big-O / Big-Θ / Big-Ω
- [ ] Amortized analysis
- [ ] Recurrence relations / recursion trees (`IO Semana 2`)

---

## 2. Data Structures

- [ ] Stack / Queue
- [ ] Linked List
- [ ] Heap / Priority Queue
- [ ] Binary Tree / BST
- [ ] Trie
- [ ] Union Find (with path compression + rank)
- [ ] Monotonic Stack
- [ ] Monotonic Queue (deque)
- [ ] Segment Tree (range queries)
- [x] Sparse Set — `Sparse Set/`
- [x] Bounded Top K (bounded heap) — `Bounded_Top_K_Heap/`

---

## 3. Core Techniques

Cross-cutting patterns. Each technique is a building block reused by the named algorithms in Section 5 and by the LeetCode problems in Section 6.

### Two Pointers & Sliding Window
- [x] Two Pointers
- [ ] Sliding Window (fixed size)
- [x] Sliding Window (variable size)

### Array & String
- [ ] Prefix Sum

### Hashing
- [ ] Frequency map (count occurrences)
- [ ] Hash Set for O(1) lookup
- [ ] Bucket Sort / Pigeonhole

### Sorting & Searching
- [x] Insertion Sort
- [x] Quick Sort
- [ ] Merge Sort
- [ ] Heap Sort
- [ ] Counting Sort
- [ ] Radix Sort
- [x] Binary Search
- [ ] Parametric search (binary search on answer)

### Divide & Conquer
*Merge Sort (see Sorting) is the canonical D&C algorithm.*
- [ ] Binary search on answer (parametric search)

### Greedy
- [ ] Interval scheduling / greedy selection
- [ ] Greedy with sorting

### Recursion & Backtracking
- [ ] Recursion with backtracking
- [ ] Subset generation
- [ ] Permutation generation
- [ ] Pruning

### Graph & Tree Traversal
- [ ] DFS (iterative + recursive)
- [ ] BFS (level-order)
- [ ] Topological sort (Kahn's algorithm)
- [ ] Cycle detection (directed / undirected)
- [ ] Tree walks: pre/in/post-order (DFS)
- [ ] Tree level-order (BFS)
- [ ] Path problems (root-to-leaf)

### Dynamic Programming
- [ ] Top-down (memoization)
- [ ] Bottom-up (tabulation)
- [ ] 1D state DP (e.g., Kadane / max subarray)
- [ ] 2D state DP
- [ ] Interval DP

---

## 4. Technique Notes

Short references for the base techniques — the definition and where a worked example lives in this repo.

### Two Pointers (convergence)

**What it is:** two indices scan the array from opposite ends toward each other; each step discards the candidate the "weaker" end can no longer improve, so at least one pointer moves every iteration. Total work is `O(n)` because the pointers only ever meet in the middle. This is the classic trick for turning a nested loop into a linear scan, and it's the pattern behind the converging-pointer problems in this repo.

**Where in this repo:**
- `Trapping_Rain_Water/rust_trapping_rain_water#max_area_1` — the cleanest convergent pair: `l` and `r` advance inward while tracking `l_max`/`r_max`.
- `Container_Most_Water/rust_container_most_water#max_area` — max-area container; note this repo version evaluates both ends per step instead of moving one pointer, so compare it against the moving-pointer version to see what convergence buys.
- `Valid_Palindrome/rust_valid_palindrome#valid_palindrome` — characters compared from both ends.

**Search terms:** "two pointers technique", "two sun sorted array convergence", "container with most water two pointers", "trapping rain water two pointers".

### Sliding Window

**What it is:** a contiguous window defined by a `left` and a `right` index slides across the input, maintaining a running aggregate in `O(1)` per step. **Fixed-size** windows keep a constant width and just update the aggregate as the window shifts. **Variable-size** windows grow `right` while the window's aggregate still satisfies the constraint, and shrink `left` when it doesn't — which is what "longest substring with at most K distinct chars" style problems use. Both amount to `O(n)` for problems a naive nested loop solves in `O(n^2)`, by observing the ends only move forward.

**Where in this repo:**
- `Longest_Substring_Without_Repeating/rust_longest_substring_without_repeating#length_of_longest_substring` — variable-size window with a `HashSet` to detect duplicates.
- `Longest_Repeating_Character_Replacement/rust_longest_repeating_character_replacement#character_replacement` — variable-size window with a frequency `HashMap` ensuring `window_length - max_freq <= k`.
Fixed-size windows aren't implemented yet (see Sliding Window Maximum).

**Search terms:** "sliding window technique", "fixed-size sliding window", "variable-size sliding window", "longest substring without repeating characters", "minimum window substring".

---

## 5. Named-Algorithm Domains

Named algorithms grouped by the technique/family they belong to. These are the classic, textbook-grade algorithms — the ones worth actually implementing from scratch.

### Dynamic Programming Classics
- [ ] 0/1 Knapsack (`IO Semana 3`)
- [ ] Matrix Chain Multiplication (`IO Semana 6`)
- [ ] Optimal Binary Search Tree (`IO Semana 6`)
- [ ] Equipment Replacement (`IO Semana 4`)
- [ ] Sport series win probability (`IO Semana 5`)

### Shortest Paths & Networks
- [ ] Dijkstra (non-neg weights)
- [ ] Bellman-Ford (negative weights)
- [ ] Floyd-Warshall (all-pairs) *(folder exists, no implementation)*
- [ ] Transportation problem / min-cost flow (`IO Semana 12`)

### Minimum Spanning Tree
- [ ] Kruskal (Union-Find)
- [ ] Prim (heap)

### Matching & Assignment
- [ ] Hungarian method — assignment problem, min-weight bipartite matching (`IO Semana 14`)

### Scheduling
*Task / CPU scheduling — classic algorithms each built on a small set of base techniques (greedy, sorting, heaps, queues):*

- [ ] Activity Selection — **greedy**: per step, keep the job with the earliest finish time that doesn't overlap.
- [ ] Earliest Deadline First (EDF) — **greedy + sort by deadline**.
- [ ] Shortest Job First (SJF) / Shortest Remaining Time First (SRTF) — **min-heap** on total / remaining time, with preemption.
- [ ] Round Robin — **FIFO queue + time quantum**, preempt on quantum expiry.
- [ ] Priority Scheduling — **priority queue / heap**; variants add aging against starvation.
- [ ] Job Sequencing with Deadlines — **greedy by profit + Union Find** over slots (or sort by profit + scan).
- [ ] Multilevel Feedback Queue — **multiple queues + aging**, jobs demote/promote between tiers.

### Project Management
- [ ] CPM — Critical Path Method / longest path in a DAG (`IO Semana 15`)

### Linear Programming
- [ ] Simplex method (`IO Semana 9-10`)
- [ ] Big-M method (`IO Semana 11`)
- [ ] Two-phase method

---

## 6. Design Patterns

- [x] Builder
- [x] Factory Method

---

## 7. LeetCode Patterns by Family

### Arrays & Hashing
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
- [x] Valid Palindrome
- [x] Container With Most Water
- [x] Trapping Rain Water
- [ ] Two Sum II (sorted input)
- [ ] 3Sum

### Sliding Window
*Base technique: Sliding Window (variable-size done)*
- [ ] Best Time to Buy and Sell Stock
- [x] Longest Substring Without Repeating Characters
- [x] Longest Repeating Character Replacement
- [ ] Minimum Window Substring
- [ ] Sliding Window Maximum

### Stack
*Base techniques: Stack, Monotonic Stack — **not started***
- [ ] Valid Parentheses
- [ ] Min Stack
- [ ] Evaluate Reverse Polish Notation
- [ ] Generate Parentheses
- [ ] Daily Temperatures
- [ ] Car Fleet
- [ ] Largest Rectangle in Histogram

### Binary Search
*Base technique: Binary Search (classic done, variants missing)*
- [ ] Search in Rotated Sorted Array
- [ ] Find Minimum in Rotated Sorted Array
- [ ] Koko Eating Bananas (parametric search)
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
*Base technique: Recursion / Backtracking — **not started***
- [ ] Subsets
- [ ] Combination Sum
- [ ] Permutations
- [ ] Word Search
- [ ] Palindrome Partitioning
- [ ] N-Queens
- [ ] Letter Combinations of Phone Number

### Graphs
*Base techniques: BFS, DFS, Union Find — **not started***
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
*Base technique: DP — **not started***
- [ ] Climbing Stairs
- [ ] Min Cost Climbing Stairs
- [ ] House Robber
- [ ] House Robber II
- [ ] Longest Palindromic Substring
- [ ] Palindromic Substrings
- [ ] Coin Change (unbounded knapsack)
- [ ] Maximum Product Subarray
- [ ] Word Break
- [ ] Longest Increasing Subsequence
- [ ] Partition Equal Subset Sum (0/1 knapsack)

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

## 8. Suggested Study Order

1. **Sliding Window** — variable-size in progress (`Longest Substring`, `Character Replacement` done); remaining: `Minimum Window Substring`, fixed-size windows (`Sliding Window Maximum`)
2. **Stack / Monotonic Stack**
3. **Binary Search variants** (incl. parametric search)
4. **Linked List**
5. **Trees (BFS/DFS first, then BST)**
6. **Heap / Priority Queue**
7. **Backtracking**
8. **Graphs & Advanced Graphs** (Dijkstra, Bellman-Ford, Kruskal, Prim)
9. **DP: 1D → 2D → Interval**, then **DP classics** (Knapsack, Matrix Chain, Optimal BST) — these reinforce the LeetCode DP lists
10. **Greedy, Intervals**
11. **Hungarian method + CPM** (short, self-contained, course-aligned)
12. **Linear Programming / Transportation / Scheduling** — as needed for the course; least transferable to LeetCode