## 透過 `cargo install` 從 Crates.io 安裝二進制執行檔

`cargo install` 命令讓你能本地安裝並使用二進制執行檔 crates。這並不是打算要取代系統套件，這是爲了方便讓 Rust 開發者可以安裝 [crates.io](https://crates.io/)<!-- ignore --> 上分享的工具。注意你只能安裝有二進制目標的套件。*二進制目標（binary target）* 是在 crate 有 *src/main.rs* 檔案或其他指定的二進制檔案時，所建立的可執行程式。而相反地，函式庫目標就無法單獨執行，因爲它提供給其他程式使用的函式庫。通常 crate 都會提供 *README* 檔案說明此 crate 是函式庫還是二進制目標，或者兩者都是。

所有透過 `cargo install` 安裝的二進制檔案都儲存在安裝根目錄的 *bin* 資料夾中。如果你是用 *rustup.rs*  安裝 Rust 且沒有任何自訂設置的話，此目錄會是 *$HOME/.cargo/bin*。請確定該目錄有在你的 `$PATH` 中，這樣才能夠執行 `cargo install` 安裝的程式。

舉例來說，第十二章我們提到有個 Rust 版本的 `grep` 工具叫做 `ripgrep` 能用來搜尋檔案。如果我們想要安裝 `ripgrep` 的話，我們可以執行以下命令：

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
--省略--
   Compiling ripgrep v11.0.2
    Finished release [optimized] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
```

輸出的最後兩行顯示了二進制檔案的安裝位置與名稱，在 `ripgrep` 此例中就是 `rg`。如稍早提到的，只要你的 `$PATH` 有包含安裝目錄，你就可以執行 `rg --help` 並開始使用更快更鏽的搜尋檔案工具！

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch14-04-installing-binaries.md)
> - updated: 2020-09-19
