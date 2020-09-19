## 透過發佈設定檔自訂建構

在 Rust 中*發佈設定檔（release profiles）*是個預先定義好並可用不同設置選項來自訂設定檔的，能讓程式設計師掌控更多選項來編譯程式碼。每個設定檔的設置彼此互相獨立。

Cargo 有兩個主要的設定檔：`dev` 設定檔會在當你對 Cargo 執行 `cargo build` 時所使用；`release` 設定檔會在當你對 Cargo 執行 `cargo build --release` 時所使用。`dev` 設定檔預設定義爲適用於開發時使用，而`release` 設定檔預設定義爲適用於發佈時使用。

你可能會覺得這些設定檔名稱很眼熟，因爲它們就已經顯示在輸出結果過：

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

此建構輸出顯示的 `dev` 與 `release` 代表編譯器會使用不同的設定檔。

當專案的 *Cargo.toml* 中沒有任何 `[profile.*]` 段落的話，Cargo 就會使用每個設定檔的預設設置。透過對你想要自訂的任何設定檔加上 `[profile.*]` 段落，你可以覆寫任何預設設定的子集。舉例來說，以下是 `dev` 與 `release` 設定檔中 `opt-level` 設定的預設數值：

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` 設定控制了 Rust 對程式碼進行優化的程度，範圍從 0 到 3。提高優化程度會增加編譯時間，所以如果你在開發過程中得時常編譯程式碼的話，你會比較想要編譯快一點，就算結果程式碼會執行的比較慢。這就是 `dev` 的 `opt-level` 預設爲 0 的原因。當你準備好要發佈你的程式碼時，則最好花多點時間來編譯。你只需要在發佈模式編譯一次，但你的編譯程式則會被執行很多次，所以發佈模式選擇花費多點編譯時間來讓程式跑得比較快。這就是 `release` 的 `opt-level` 預設爲 3 的原因。

你可以在 *Cargo.toml* 加上不同的數值來覆蓋任何預設設定。舉例來說，如果我們希望在開發設定檔使用優化等級 1 的話，我們可以在專案的 *Cargo.toml* 檔案中加上這兩行：

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

這樣就會覆蓋預設設定 `0`。現在當我們執行 `cargo build`，Cargo 就會使用 `dev` 設定檔的預設值以及我們自訂的 `opt-level`。因爲我們將 `opt-level` 設爲 `1`，Cargo 會比原本的預設進行更多優化，但沒有發佈建構那麼多。

對於完整的設置選項與每個設定檔的預設列表，請查閱 [Cargo 的技術文件](https://doc.rust-lang.org/cargo/reference/profiles.html)。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch14-01-release-profiles.md)
> - updated: 2020-09-19
