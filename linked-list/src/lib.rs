// Why Linked List is better than Arrays/HashMap?
//  - insert/remove have fewer allocations -> vs array
//  - elements are ordered -> vs hashmaps

// Note:
//  - you should always prefer: (read the documentation)
//  - standard library: see std::collections
//  - other crates: w/ specialized approach

// Use std::collections::LinkedList only when:
//  - You want a Vec or VecDeque of unknown size, and can’t tolerate amortization.
//  - You want to efficiently split and append lists.
//  - You are absolutely certain you really, truly, want a doubly linked list.
//  - NOTE: It is almost always better to use Vec or VecDeque
//          because array-based containers are generally faster, more memory efficient,
//          and make better use of CPU cache.

// Since we're only going to implement singly linked list, we will use Vec only

pub mod bad_linked_list;
