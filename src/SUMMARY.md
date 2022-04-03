# Rust 程式設計語言

[Rust 程式設計語言](title-page.md)
[前言](foreword.md)
[介紹](ch00-00-introduction.md)

## 開始入門

- [開始入門](ch01-00-getting-started.md)
    - [安裝教學](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [設計猜謎遊戲程式](ch02-00-guessing-game-tutorial.md)

- [常見程式設計概念](ch03-00-common-programming-concepts.md)
    - [變數與可變性](ch03-01-variables-and-mutability.md)
    - [資料型別](ch03-02-data-types.md)
    - [函式](ch03-03-how-functions-work.md)
    - [註解](ch03-04-comments.md)
    - [控制流程](ch03-05-control-flow.md)

- [理解所有權](ch04-00-understanding-ownership.md)
    - [什麼是所有權？](ch04-01-what-is-ownership.md)
    - [引用與借用](ch04-02-references-and-borrowing.md)
    - [切片型別](ch04-03-slices.md)

- [透過結構體組織相關資料](ch05-00-structs.md)
    - [定義與實例化結構體](ch05-01-defining-structs.md)
    - [使用結構體的程式範例](ch05-02-example-structs.md)
    - [方法語法](ch05-03-method-syntax.md)

- [枚舉與模式配對](ch06-00-enums.md)
    - [定義枚舉](ch06-01-defining-an-enum.md)
    - [`match` 控制流建構子](ch06-02-match.md)
    - [透過 `if let` 簡化控制流](ch06-03-if-let.md)

## 基本 Rust 概念

- [透過套件、Crate 與模組管理成長中的專案](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [套件與 Crates](ch07-01-packages-and-crates.md)
    - [定義模組來控制作用域與隱私權](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [引用模組項目的路徑](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [透過 `use` 關鍵字引入路徑](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [將模組拆成不同檔案](ch07-05-separating-modules-into-different-files.md)

- [常見集合](ch08-00-common-collections.md)
    - [透過向量儲存列表](ch08-01-vectors.md)
    - [透過字串儲存 UTF-8 編碼的文字](ch08-02-strings.md)
    - [透過雜湊映射儲存鍵值配對](ch08-03-hash-maps.md)

- [錯誤處理](ch09-00-error-handling.md)
    - [`panic!` 與無法復原的錯誤](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` 與可復原的錯誤](ch09-02-recoverable-errors-with-result.md)
    - [要 `panic!` 還是不要 `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [泛型型別、特徵與生命週期](ch10-00-generics.md)
    - [泛型資料型別](ch10-01-syntax.md)
    - [特徵：定義共同行為](ch10-02-traits.md)
    - [透過生命週期驗證引用](ch10-03-lifetime-syntax.md)

- [編寫自動化測試](ch11-00-testing.md)
    - [如何寫測試](ch11-01-writing-tests.md)
    - [控制程式如何執行](ch11-02-running-tests.md)
    - [測試組織架構](ch11-03-test-organization.md)

- [I/O 專案：建立一個命令列程式](ch12-00-an-io-project.md)
    - [接受命令列引數](ch12-01-accepting-command-line-arguments.md)
    - [讀取檔案](ch12-02-reading-a-file.md)
    - [透過重構來改善模組性與錯誤處理](ch12-03-improving-error-handling-and-modularity.md)
    - [透過測試驅動開發完善函式庫功能](ch12-04-testing-the-librarys-functionality.md)
    - [處理環境變數](ch12-05-working-with-environment-variables.md)
    - [將錯誤訊息寫入標準錯誤而非標準輸出](ch12-06-writing-to-stderr-instead-of-stdout.md)

## 以 Rust 思維思考

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

- [Rust 的物件導向程式設計特色](ch17-00-oop.md)
    - [物件導向語言的特色](ch17-01-what-is-oo.md)
    - [允許不同型別數值的特徵物件](ch17-02-trait-objects.md)
    - [實作物件導向設計模式](ch17-03-oo-design-patterns.md)

## 進階主題

- [模式與配對](ch18-00-patterns.md)
    - [所有能使用模式的地方](ch18-01-all-the-places-for-patterns.md)
    - [可反駁性：何時模式可能會配對失敗](ch18-02-refutability.md)
    - [模式語法](ch18-03-pattern-syntax.md)

- [進階特色](ch19-00-advanced-features.md)
    - [不安全的 Rust](ch19-01-unsafe-rust.md)
    - [進階特徵](ch19-03-advanced-traits.md)
    - [進階型別](ch19-04-advanced-types.md)
    - [進階函式與閉包](ch19-05-advanced-functions-and-closures.md)
    - [巨集](ch19-06-macros.md)

- [最終專案：建立多執行緒網頁伺服器](ch20-00-final-project-a-web-server.md)
    - [建立單一執行緒的網頁伺服器](ch20-01-single-threaded.md)
    - [將單一執行緒伺服器轉換為多執行緒伺服器](ch20-02-multithreaded.md)
    - [正常關機與清理](ch20-03-graceful-shutdown-and-cleanup.md)

- [附錄](appendix-00.md)
    - [A - 關鍵字](appendix-01-keywords.md)
    - [B - 運算子與符號](appendix-02-operators.md)
    - [C - 可推導的特徵](appendix-03-derivable-traits.md)
    - [D - 實用開發工具](appendix-04-useful-development-tools.md)
    - [E - 版號](appendix-05-editions.md)
    - [F - 本書的翻譯本](appendix-06-translation.md)
    - [G - Rust 的開發流程與「夜版 Rust」](appendix-07-nightly-rust.md)
    - [H - 中英術語對照表](appendix-08-terminology.md)
