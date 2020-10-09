## 接受命令列引數

一如往常我們用 `cargo new` 建立新的專案，我們將我們的專案命名爲 `minigrep` 來與很可能已經在你系統中的 `grep` 工具做區別。

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

第一項任務是要讓 `minigrep` 能接收兩個命令列引數：檔案名稱與欲搜尋的字串。也就是說，我們想要能夠使用 `cargo run`、欲搜尋的字串與要被搜尋的檔案路徑來執行程式，如以下所示：

```console
$ cargo run searchstring example-filename.txt
```

但現在由 `cargo new` 產生的程式還無法處理我們給予的引數。[crates.io](https://crates.io/) 有些函式庫可以幫助程式接收命令列中的引數，但有鑑於你要學習此概念，讓我們親自來實作一個。

### 讀取引數數值

要讓 `minigrep` 能夠讀取我們傳入的命令列引數數值，我們需要使用 Rust 標準函式庫中提供的函式，也就是 `std::env::args`。此函式會回傳一個包含我們傳給 `minigrep` 的命令列引數的疊代器（iterator）。我們會在[第十三章][ch13]<!-- ignore -->詳細解釋疊代器。現在你只需要知道疊代器的兩項重點：疊代器會產生一系列的數值，然後我們可以對疊代器呼叫 `collect` 方法來將其轉換成像是 像是向量的集合，來包含疊代器產生的所有元素。

使用範例 21-1 的程式碼能讓你的 `minigrep` 程式能夠讀取任何傳入的命令列引數，然後收集數值成一個向量。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

<span class="caption">範例 12-1：收集命令列引數至向量中並顯示它們</span>

首先我們透過 `use` 陳述式將 `std::env` 模組引入作用域，讓我們可以使用它的 `args` 函式。注意到 `std::env::args` 函式位於兩層模組下。如同我們在[第七章][ch7-idiomatic-use]<!-- ignore -->談過的，如果我們要用的函式模組路徑超過一層以上的話，通常就會將上層模組引入作用域中，而不是函式本身。這樣的話，我們可以輕鬆使用 `std::env` 中的其他函式。而且這也比直接加上 `use std::env::args` 然後只使用 `args` 來呼叫函式還要明確些，因爲 `args` 容易被誤認成是由目前模組定義的函式。

> ### `args` 函式與無效的 Unicode
>
> 值得注意的是如果任何引數包含無效 Unicode 的話，`std::env::args` 就會恐慌。如果你的程式想要接受包含無效 Unicode 引數的話，請改使用 `std::env::args_os`。該函式回傳會產生 `OsString` 數值的疊代器，而非 `String` 數值。我們出於簡單方便所以在此使用 `std::env::args`，因爲 `OsString` 在不同平台中數值會有所差異，且會比 `String` 數值還要難處理。

我們在 `main` 中的第一行呼叫 `env::args`，然後馬上使用 `collect` 來將疊代器轉換成向量，這會包含疊代器產生的所有數值。我們可以使用 `collect` 函式來建立許多種集合，所以我們顯式詮釋 `args` 的型別來指定我們想要字串向量。雖然我們很少需要在 Rust 中詮釋型別，`collect` 是其中一個你常常需要詮釋的函式，因爲 Rust 無法推斷出你想要何種集合。

最後，我們使用除錯格式 `:?` 來顯示向量。讓我們先嘗試不用引數來執行程式碼，再用兩個引數來執行：

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

值得注意的是向量中第一個數值爲 `"target/debug/minigrep"`，這是我們的執行檔名稱。這與 C 的引數列表行爲相符，讓程式在執行時能使用它們被呼叫的名稱路徑。存取程式名稱通常是很實用的，像是你能將它顯示在訊息中，或是依據程式被呼叫的命令列別名還改變程式的行爲。但考慮本章節的目的，我們會忽略它並只儲存我們想要的兩個引數。

### 將引數數值儲存至變數

顯示向量中的引數數值能說明程式能夠取得命令列引數指定的數值。現在我們想要將這兩個引數存入變數中，讓我們可以在接下來的程式中使用數值，如範例 12-2 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

<span class="caption">範例 12-2：建立變數來儲存搜尋引數與檔案名稱引數</span>

如我們印出向量時鎖看到的，向量的第一個數值 `args[0]` 會是程式名稱，所以我們從引數 `1` 開始。`minigrep` 接收的第一個引數會是我們要搜尋的字串，所以我們將第一個引數的引用賦值給變數 `query`。第二個引數會是檔案名稱，所以我們將第二個引數的引用賦值給 `filename`。

我們暫時印出這些變數的數值來證明程式碼運作無誤。讓我們用引數 `test` 與 `sample.txt` 來再次執行程式：

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

很好，程式能執行！我們想要的引數數值都有儲存至正確的變數中。之後我們會對特定潛在錯誤情形來加上一些錯誤處理，像是當使用者沒有提供引數的情況。現在我們先忽略這樣的情況，並開始加上讀取檔案的功能。

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch12-01-accepting-command-line-arguments.md)
> - updated: 2020-10-02
