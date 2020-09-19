## 透過自訂命命來擴展 Cargo 的功能

Cargo 的設計能讓你在不用修改 Cargo 的情況下擴展新的子命令。如果你 `$PATH` 中有任何叫做 `cargo-something` 的二進制檔案，你就可以用像是執行 Cargo 子命令的方式 `cargo something` 來執行它。像這樣的自訂命令在你執行 `cargo --list` 時也會顯示出來。能夠透過 `cargo install` 來安裝擴展插件並有如內建 Cargo 工具般來執行使用是 Cargo 設計上的一大方便優勢！

## 總結

透過 Cargo 與 [crates.io](https://crates.io/)<!-- ignore --> 分享程式碼是讓 Rust 生態系統能適用於許多不同任務的重要部分之一。Rust 的標準函式庫既小又穩定，但是 crate 可以很容易地分享、使用，並在語言本身不同的時間線來進行改善。千萬別吝嗇於分享你認爲實用的程式碼到 [crates.io](https://crates.io/)<!-- ignore -->，其他人可能也會覺得它很有幫助！

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch14-05-extending-cargo.md)
> - updated: 2020-09-19
