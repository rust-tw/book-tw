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

- [透過結構體組織相關資料](ch05-00-structs.md)
    - [定義與實例化結構體](ch05-01-defining-structs.md)
    - [使用結構體的程式範例](ch05-02-example-structs.md)
    - [方法語法](ch05-03-method-syntax.md)

- [枚舉與模式配對](ch06-00-enums.md)
    - [定義枚舉](ch06-01-defining-an-enum.md)
    - [`match` 控制流運算子](ch06-02-match.md)
    - [透過 `if let` 簡化控制流](ch06-03-if-let.md)

## Basic Rust Literacy

- [透過套件、Crate與模組管理成長中的專案](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [套件與 Crates](ch07-01-packages-and-crates.md)
    - [定義模組來控制作用域與隱私權](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [引用模組項目的路徑](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [透過 `use` 關鍵字引入路徑](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [將模組拆成不同檔案](ch07-05-separating-modules-into-different-files.md)

- [常見集合](ch08-00-common-collections.md)
    - [透過 Vector 儲存列表](ch08-01-vectors.md)
    - [透過字串儲存 UTF-8 編碼的文字](ch08-02-strings.md)
    - [透過雜湊映射儲存鍵值配對](ch08-03-hash-maps.md)

- [錯誤處理](ch09-00-error-handling.md)
    - [`panic!` 與無法復原的錯誤](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` 與可復原的錯誤](ch09-02-recoverable-errors-with-result.md)
    - [要 `panic!` 還是不要 `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [泛型型別、特徵與生命週期](ch10-00-generics.md)
    - [泛型資料型別](ch10-01-syntax.md)
    - [特徵：定義共同行爲](ch10-02-traits.md)
    - [透過生命週期驗證引用](ch10-03-lifetime-syntax.md)

- [編寫自動化測試](ch11-00-testing.md)
    - [如何寫測試](ch11-01-writing-tests.md)
    - [控制程式如何執行](ch11-02-running-tests.md)
    - [測試組織架構](ch11-03-test-organization.md)

- [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.md)
    - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.md)
    - [Reading a File](ch12-02-reading-a-file.md)
    - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.md)
    - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
    - [Working with Environment Variables](ch12-05-working-with-environment-variables.md)
    - [Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [函式語言功能：疊代器與閉包](ch13-00-functional-features.md)
    - [閉包：能獲取其環境的匿名函式](ch13-01-closures.md)
    - [使用疊代器來處理一系列的項目](ch13-02-iterators.md)
    - [改善我們的 I/O 專案](ch13-03-improving-our-io-project.md)
    - [比較效能：迴圈 vs. 疊代器](ch13-04-performance.md)

- [更多關於 Cargo 與 Crates.io 的內容](ch14-00-more-about-cargo.md)
    - [透過發佈設定檔自訂建構](ch14-01-release-profiles.md)
    - [發佈 Crate 到 Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo 工作空間](ch14-03-cargo-workspaces.md)
    - [透過 `cargo install` 從 Crates.io 安裝二進制執行檔](ch14-04-installing-binaries.md)
    - [透過自訂命命來擴展 Cargo 的功能](ch14-05-extending-cargo.md)

- [智慧指標](ch15-00-smart-pointers.md)
    - [使用 `Box<T>` 指向堆積上的資料](ch15-01-box.md)
    - [透過 `Deref` 特徵將智慧指標視為一般引用](ch15-02-deref.md)
    - [透過 `Drop` 特徵執行清除程式碼](ch15-03-drop.md)
    - [`Rc<T>` 引用計數智慧指標](ch15-04-rc.md)
    - [`RefCell<T>` 與內部可變性模式](ch15-05-interior-mutability.md)
    - [引用循環會導致記憶體泄漏](ch15-06-reference-cycles.md)

- [無懼並行](ch16-00-concurrency.md)
    - [使用執行緒同時執行程式碼](ch16-01-threads.md)
    - [使用訊息傳遞在執行緒間傳送資料](ch16-02-message-passing.md)
    - [共享狀態並行](ch16-03-shared-state.md)
    - [透過 `Sync` 與 `Send` 特徵擴展並行性](ch16-04-extensible-concurrency-sync-and-send.md)

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
    - [不安全的 Rust](ch19-01-unsafe-rust.md)
    - [進階特徵](ch19-03-advanced-traits.md)
    - [進階型別](ch19-04-advanced-types.md)
    - [進階函式與閉包](ch19-05-advanced-functions-and-closures.md)
    - [巨集](ch19-06-macros.md)

- [Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.md)
    - [Building a Single-Threaded Web Server](ch20-01-single-threaded.md)
    - [Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.md)
    - [Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.md)

- [Appendix](appendix-00.md)
    - [A - Keywords](appendix-01-keywords.md)
    - [B - Operators and Symbols](appendix-02-operators.md)
    - [C - 可推導的特徵](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - Translations of the Book](appendix-06-translation.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
    - [H - 中英術語對照表](appendix-08-terminology.md)
