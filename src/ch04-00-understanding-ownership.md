# 理解所有權

所有權可以說是 Rust 最與衆不同的特色，這讓 Rust 不需要垃圾回收（garbage collector）就可以保障記憶體安全。因此理解 Rust 中的所有權如何運作是非常重要的。在本章節，我們將討論所有權以及一些相關的功能：借用、slices 與 Rust 如何在記憶體分配資料。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [1fedfc4](https://github.com/rust-lang/book/blob/1fedfc4b96c2017f64ecfcf41a0a07e2e815f24f/src/ch04-00-understanding-ownership.md)
> - updated: 2020-09-07