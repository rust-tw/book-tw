# The Rust Programming Language

[The Rust Programming Language](title-page.md)
[Foreword](foreword.md)
[Introduction](ch00-00-introduction.md)

## Getting started

- [開始入門](ch01-00-getting-started.md)
    - [安裝教學](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Programming a Guessing Game](ch02-00-guessing-game-tutorial.md)

- [常見程式設計概念](ch03-00-common-programming-concepts.md)
    - [變數與可變性](ch03-01-variables-and-mutability.md)
    - [資料型別](ch03-02-data-types.md)
    - [函式](ch03-03-how-functions-work.md)
    - [註解](ch03-04-comments.md)
    - [控制流程](ch03-05-control-flow.md)

- [理解所有權](ch04-00-understanding-ownership.md)
    - [什麼是所有權？](ch04-01-what-is-ownership.md)
    - [引用與借用](ch04-02-references-and-borrowing.md)
    - [Slice 型別](ch04-03-slices.md)

- [使用結構體組織相關資料](ch05-00-structs.md)
    - [定義與實例化結構體](ch05-01-defining-structs.md)
    - [使用結構體的程式範例](ch05-02-example-structs.md)
    - [方法語法](ch05-03-method-syntax.md)

- [枚舉與模式配對](ch06-00-enums.md)
    - [定義枚舉](ch06-01-defining-an-enum.md)
    - [`match` 控制流運算子](ch06-02-match.md)
    - [使用 `if let` 簡化控制流](ch06-03-if-let.md)

## Basic Rust Literacy

- [使用套件、Crate與模組管理成長中的專案](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [套件與 Crates](ch07-01-packages-and-crates.md)
    - [定義模組來控制作用域與隱私權](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [引用模組項目的路徑](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [使用 `use` 關鍵字引入路徑](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [將模組拆成不同檔案](ch07-05-separating-modules-into-different-files.md)

- [常見集合](ch08-00-common-collections.md)
    - [使用 Vector 儲存列表](ch08-01-vectors.md)
    - [使用字串儲存 UTF-8 編碼的文字](ch08-02-strings.md)
    - [使用雜湊映射儲存鍵值配對](ch08-03-hash-maps.md)

- [錯誤處理](ch09-00-error-handling.md)
    - [`panic!` 與無法復原的錯誤](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` 與可復原的錯誤](ch09-02-recoverable-errors-with-result.md)
    - [要 `panic!` 還是不要 `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [泛型型別、特徵與生命週期](ch10-00-generics.md)
    - [泛型資料型別](ch10-01-syntax.md)
    - [特徵：定義共享行爲](ch10-02-traits.md)
    - [使用生命週期驗證引用](ch10-03-lifetime-syntax.md)

- [Writing Automated Tests](ch11-00-testing.md)
    - [How to Write Tests](ch11-01-writing-tests.md)
    - [Controlling How Tests Are Run](ch11-02-running-tests.md)
    - [Test Organization](ch11-03-test-organization.md)

- [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.md)
    - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.md)
    - [Reading a File](ch12-02-reading-a-file.md)
    - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.md)
    - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
    - [Working with Environment Variables](ch12-05-working-with-environment-variables.md)
    - [Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [Functional Language Features: Iterators and Closures](ch13-00-functional-features.md)
    - [Closures: Anonymous Functions that Can Capture Their Environment](ch13-01-closures.md)
    - [使用疊代器來處理一系列的項目](ch13-02-iterators.md)
    - [Improving Our I/O Project](ch13-03-improving-our-io-project.md)
    - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.md)

- [More about Cargo and Crates.io](ch14-00-more-about-cargo.md)
    - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md)
    - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
    - [Installing Binaries from Crates.io with `cargo install`](ch14-04-installing-binaries.md)
    - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.md)

- [Smart Pointers](ch15-00-smart-pointers.md)
    - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
    - [透過 `Deref` 特徵將智慧指標視為一般引用](ch15-02-deref.md)
    - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
    - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md)
    - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md)
    - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Fearless Concurrency](ch16-00-concurrency.md)
    - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
    - [Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.md)
    - [Shared-State Concurrency](ch16-03-shared-state.md)
    - [Extensible Concurrency with the `Sync` and `Send` Traits](ch16-04-extensible-concurrency-sync-and-send.md)

- [Object Oriented Programming Features of Rust](ch17-00-oop.md)
    - [Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.md)
    - [允許不同型別數值的特徵物件](ch17-02-trait-objects.md)
    - [Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.md)

## 進階主題

- [Patterns and Matching](ch18-00-patterns.md)
    - [All the Places Patterns Can Be Used](ch18-01-all-the-places-for-patterns.md)
    - [Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.md)
    - [Pattern Syntax](ch18-03-pattern-syntax.md)

- [進階特色](ch19-00-advanced-features.md)
    - [Unsafe Rust](ch19-01-unsafe-rust.md)
    - [進階特徵](ch19-03-advanced-traits.md)
    - [進階型別](ch19-04-advanced-types.md)
    - [Advanced Functions and Closures](ch19-05-advanced-functions-and-closures.md)
    - [Macros](ch19-06-macros.md)

- [Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.md)
    - [Building a Single-Threaded Web Server](ch20-01-single-threaded.md)
    - [Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.md)
    - [Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.md)

- [Appendix](appendix-00.md)
    - [A - Keywords](appendix-01-keywords.md)
    - [B - Operators and Symbols](appendix-02-operators.md)
    - [C - Derivable Traits](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - Translations of the Book](appendix-06-translation.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
    - [H - 中英術語對照表](appendix-08-terminology.md)
