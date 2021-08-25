# Rust 程式設計語言

*由 Steve Klabnik 與 Carol Nichols，以及 Rust 社群的貢獻撰寫而成*

此版本假設你使用的是 Rust 1.54 或更高的版本，並在所有專案中的 *Cargo.toml* 都有 `edition="2018"` 來使用 Rust 2018 版號。請查看[第一章的「安裝」段落][install]<!-- ignore -->來安裝或更新 Rust，並查看[附錄 E][editions]<!-- ignore --> 來瞭解版號的資訊。

Rust 語言的 2018 版號包含一系列的改進使得 Rust 更加易讀易用且更容易學習。本書這次的疊代版本中包含一些更新內容來反映這些改進：

- 第七章的「透過套件、Crate 與模組管理成長中的專案」幾乎完全重寫。模組系統以及其在 2018 版號中的路徑處理變得更一致。
- 第十章新增了「特徵作為參數」與「返回有實作特徵的型別」的段落來解釋新的 `impl Trait` 語法。
- 第十一章有個新段落「在測試中使用 `Result<T, E>`」來展示如何在測試中使用 `?` 運算子。
- 第十九章的「進階生命週期」已被移除，因為編譯器的改善讓使用該段落的時機變得更加稀少了。
- 之前的附錄 D 「巨集」被進一步擴展，將額外涵蓋程序性巨集的內容，且被移至第十九章的「巨集」段落中。
- 附錄 A 「關鍵字」新增了原始標識符的功能介紹，來讓 2015 版號的程式碼可以用在 2018 版號也不會出現問題。
- 附錄 D 現在的標題為「實用開發工具」，其中涵蓋有近期推出的工具，這些能在你撰寫 Rust 程式碼時幫助到你。
- 我們修正了整本書中一些小錯誤與不精確的措辭。感謝各位讀者的回報！

值得注意的是 **Rust 程式設計語言**早期疊代版本中的程式碼在專案中 *Cargo.toml* 移除 `edition="2018"` 的話就能繼續編譯，儘管你升級了 Rust 編譯器也是如此。Rust 的向下相容保證一切可以正常運行！

本書的 HTML 格式可以在線上閱讀：[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)（[正體中文版](https://rust-lang.tw/book-tw/)）。而離線版則包含在 `rustup` 安裝的 Rust 中，輸入 `rustup docs --book` 就能開啟。

本書也有由 [No Starch Press 出版平裝與電子版格式][nsprust]。

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust

> - commit: [e8200d2](https://github.com/rust-lang/book/commit/e8200d27ed1dde8d1c3411cfc9580b0aa53cf7ee)
