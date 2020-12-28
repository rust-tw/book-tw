## 註解

所有程式設計師均致力於讓他們的程式碼易於閱讀，不過有時候額外的解釋還是需要的。這種情況下，開發者會在他們的程式碼留下一些筆記或是**註解（comments）**，編譯器會忽略這些字，但其他人在閱讀程式碼時可能就會覺得很有幫助。

這是一個簡單地註解：

```rust
// 安安，你好
```

在 Rust 中，慣用的註解風格是用兩行斜線在加上一個空格起頭，然後註解就能一直寫到該行結束為止。如果註解會超過一行的話，你需要在每一行都加上 `//`，如下所示：

```rust
// 這邊處理的事情很複雜，長到
// 我們需要多行註解來能解釋！
// 希望此註解能幫助你理解。
```

註解也可以加在程式碼之後：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

不過你會更常看到它們用用以下格式，註解會位於要說明的程式碼上一行：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust 還有另一種註解：技術文件註解。我們會在第十四章的「發佈 Crate 到 Crates.io」段落提到它。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [25a1530](https://github.com/rust-lang/book/blob/25a1530ccbf0a79c8df2920ee2af8beb106122e8/src/ch03-04-comments.md)
> - updated: 2020-09-06