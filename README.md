# **Boost**

A library for boosting your develop productivity with Rust.

<br/>

## **Table of Contents**

* [Features](#features)
* [Usage](#usage)
* [Linked Blog](#linked-blog)

<br/>

## **Features**

**boost-rs:**

-   **Bit**: multiple bit operations;
-   **Collection**: 
    -   BloomFilter;
    -   Cache:
        -   LRUCache;
    -   Skiplist;
    -   BinarySearchTree;
    -   LinkedList;
    -   Macros:
        -   `btreemap!`;
        -   `btreeset!`;
        -   `hashmap!`;
        -   `hashset!`;
-   **Env**: environment related utilities, such as: `debug_mode()`;
-   **Generic**: generic related utilities, such as: `ArcMut<T> = Arc<Mutex<T>>`;
-   **Logger**: a logger util to initialize and config a global logger for [`log`](https://crates.io/crates/log) crate;
-   **Macros**: macro related utilities, such as: `empty_trait_impl` (implement multiple empty trait);
-   **Rand**: random related utilities;
-   **Sort**: multiple sort algorithm implementation:
    -   Bubble sort;
    -   Heap sort;
    -   Insertion sort;
    -   Merge sort;
    -   Quick sort;
    -   Selection sort;
-   **Types**: rust type system related utilities;

**macros:**

-   **Elaspesd**: proc-macro for calculating function running time;

<br/>

## **Usage**

Activated full features of the boost-rs crate on Cargo.toml:

```toml
[dependencies]
boost-rs = { version = "x.y.z", features = ["full"] }
```

Or, multiple features only:

```toml
[dependencies]
boost-rs = { version = "x.y.z", features = ["collection", "sort"] }
```

<br/>

## **Linked Blog**

Blogs:

-   [《使用Rust实现一个双向链表》](https://jasonkayzk.github.io/2022/02/20/使用Rust实现一个双向链表/)
-   [《Rust中的默认初始化和初始化重载》](https://jasonkayzk.github.io/2022/11/19/Rust中的默认初始化和初始化重载/)
-   [《Rust反射之Any》](https://jasonkayzk.github.io/2022/11/24/Rust反射之Any/)
-   [《Rust中的比较》](https://jasonkayzk.github.io/2022/11/23/Rust中的比较/)
-   [《Rust反射之过程宏》](https://jasonkayzk.github.io/2022/11/25/Rust反射之过程宏/)
-   [《通过一个例子学习Cargo-Features》](https://jasonkayzk.github.io/2022/11/28/通过一个例子学习Cargo-Features/)
-   [《使用Rust实现布隆过滤器BloomFilter》](https://jasonkayzk.github.io/2022/12/16/使用Rust实现布隆过滤器BloomFilter/)
-   [《使用Rust实现跳表Skiplist》](https://jasonkayzk.github.io/2022/12/16/使用Rust实现跳表Skiplist/)
