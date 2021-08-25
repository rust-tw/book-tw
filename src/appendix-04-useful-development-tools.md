## 附錄 D - 實用開發工具

在此附錄中，我們會討論些 Rust 專案提供的實用開發工具。我們會介紹自動格式化工具、修正警告最快速的方式、linter 以及 IDE 的整合工具。

### 透過 `rustfmt` 自動格式化

`rustfmt` 工具會依據社群程式碼風格來重新格式化你的程式碼。許多協作專案都會使用 `rustfmt` 來避免 Rust 風格的歧義，每個人都能用此工具格式化他們的程式碼。

欲安裝 `rustfmt`，請輸入以下命令：

```console
$ rustup component add rustfmt
```

此命令會給你 `rustfmt` 與 `cargo-fmt`，就像 Rust 會提供你 `rustc` 與 `cargo` 一樣。要格式化任何 Cargo 專案的話，請輸入：

```console
$ cargo fmt
```

執行此命令會重新格式化目前 crate 中所有的 Rust 程式碼。不過這只會變更程式碼風格，並不會影響程式碼語義。想瞭解更多 `rustfmt` 的資訊，歡迎查閱[它的技術文件][rustfmt]。

[rustfmt]: https://github.com/rust-lang/rustfmt

### 透過 `rustfix` 修正你的程式碼

rustfix 工具包含在 Rust 的安裝中，且可以自動修正一些編譯器警告。如果你寫過 Rust 程式碼的話，你應該會看過一些編譯器警告，舉例來說，請參考以下程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

我們在此呼叫 `do_something` 函式 100 次，但是我們在 `for` 迴圈中完全沒用到變數 `i`。Rust 會警告我們：

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

警告訊息建議我們改使用 `_i` 來作為名稱，底線指的是我們認定此變數不會被使用。我們可以透過執行 `cargo fix` 來使用 `rustfix` 工具以自動採用這些建議：

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

當我們再次檢查 *src/main.rs*，我們會看到 `cargo fix` 已經將程式碼修正了：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

`for` 迴圈變數現在改名為 `_i`，而警告也不再出現了。

你也可以使用 `cargo fix` 命令來在不同的 Rust 版號之間做轉換程式碼。版號會在附錄 E 做介紹。

### 透過 Clippy 運用更多功能

Clippy 工具是一系列的 lint 集合，用來分析程式碼以獲取常見錯誤並改善你的 Rust 程式碼。

要安裝 Clippy 的話，輸入以下命令：

```console
$ rustup component add clippy
```

要在任何 Cargo 專案執行 Clippy，輸入以下命令：

```console
$ cargo clippy
```

舉例來說，假設你在寫程式時使用到如 pi 這種數學常數的近似值，如以下所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

對此專案執行 `cargo clippy` 會顯示以下錯誤：

```text
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/master/index.html#approx_constant
```

此錯誤告訴你 Rust 有對該常數提供更精準的定義，如果改使用此定義的話，你的程式會更準確。你可以將你的程式碼改使用 `PI` 常數。以下程式碼就不會透過 Clippy 獲得任何錯誤或警告：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

關於更多 Clippy 的資訊，請查閱[它的技術文件][clippy]。

[clippy]: https://github.com/rust-lang/rust-clippy

### 使用 Rust Language Server 整合 IDE

為了協助 IDE 的整合，Rust 專案有發佈 **Rust Language Server**（`rls`）。此工具會與 [Language Server Protocol][lsp] 溝通，這是 IDE 與程式語言彼此溝通的協定規格。`rls` 可用於各種不同的客戶端，像是 [Visual Studio Code 的 Rust 插件][vscode]。

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

要安裝 `rls` 的話，輸入以下命令：

```console
$ rustup component add rls
```

然後在你指定的 IDE 安裝 language server 對應的支援，你就能獲得許多功能，像是自動補全、跳至定義以及行內錯誤顯示等等。

關於更多 `rls` 的資訊，請查閱[它的技術文件][rls]。

[rls]: https://github.com/rust-lang/rls
