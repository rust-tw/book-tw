# 設計猜謎遊戲程式

讓我們透過動手一同完成一項專案來開始上手 Rust 吧！北章節會介紹一些常見 Rust 概念，展示如何在實際程式中使用它們。你會學到 `let`、`match`、方法、關聯函式、使用外部 crate 以及更多等等！之後的章節會更詳細地探討這些概念。在本章中，你會先練習基礎概念。

我們會實作個經典新手程式問題：猜謎遊戲。它的運作方式如下：程式會產生 1 到 100 之間的隨機整數。接著它會通知玩家猜一個數字。在輸入猜測數字之後，程式會回應猜測的數字太低或太高。如果猜對的話，遊戲就會顯示祝賀訊息並離開。

## 設置新專案

要設置新專案的話，前往你在第一章建立的 *projects* 目錄並使用 Cargo 建立一個新的專案，如下所示：

```console
$ cargo new guessing_game
$ cd guessing_game
```

第一道命令 `cargo new` 會接收專案名稱（`guessing_game`）作爲引數。第二道命令會將目錄移至新專案中。

檢查看看產生的 *Cargo.toml* 檔案：

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

如果 Cargo 從你的環境取得的作者資訊不正確的話，請在檔案中修改並儲存。

如同你在第一章看到的，`cargo new` 會產程一支「Hello, world!」程式。請檢查 *src/main.rs* 檔案：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

現在讓我們用 `cargo run` 命令一次完成編譯與執行「Hello, world!」程式：

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

`run` 命令在你需要對專案快速疊代時會很有用，我們要寫的遊戲也是如此，在繼續下一步之前可以快速測試每一步。

請重新開啟 *src/main.rs* 檔案。你要寫的程式碼全會位於此檔案中。

## 處理猜測

猜謎遊戲的第一個部分會要求使用者輸入數字、處理該輸入，並檢查該輸入是否符合格式。所以我們要先讓玩家能夠輸入猜測數字，請輸入範例 2-1 的程式碼至 *src/main.rs*。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">範例 2-1：取得使用者的猜測數字並顯示出來的程式</span>

此程式碼包含大量的資訊，所以讓我們一行一行來慢慢看吧。要取得使用者輸入並印出爲輸出結果，我們需要將 `io` 輸入／輸出（input/output）函式庫引入作用域中。 `io` 函式庫來自標準函式庫（常稱爲 `std`）：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Rust 在預設情況下只會透過 [*prelude*][prelude]<!-- ignore --> 來將一些型別引入每個程式的作用域中。如果你想使用的型別不在 prelude 的話，你需要顯式地使用 `use` 陳述式（statement）將該型別引入作用域。`std::io` 函式庫能提供一系列實用的功能，這包含接收使用者輸入的能力。

[prelude]: ../std/prelude/index.html

如同你在第一章所見的，`main` 函式是程式的入口點（entry point）：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` 語法用來宣告新的函式（function），其中括號 `()` 說明此函式沒有任何參數，然後大括號 `{` 會作爲函式本體的開頭。

同樣如第一章所學的，`println!` 是個能將字串顯示到螢幕上的巨集：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

此程式碼會顯式提示訊息向使用者說明此遊戲該輸入什麼。

### 透過變數儲存數值

接著我們要建立一個變數來儲存使用者輸入，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

現在程式變得越來越有趣了！在短短的這行當中有許多事情發生。先注意到這是個 `let` 陳述式，這用來建立一個*變數（variable）*。以下是另一個例子：

```rust,ignore
let foo = bar;
```

這行建立了一個新的變數叫做 `foo` 並將變數 `bar` 的數值綁定給它。在 Rust 中，變數預設是不可變的（immutable）。我們會在第三章的[「變數與可變性」][variables-and-mutability]<!-- ignore -->段落討論此概念。以下範例展示如何在變數名稱前面使用 `mut` 來讓變數成爲可變的：

```rust,ignore
let foo = 5; // 不可變的
let mut bar = 5; // 可變的
```

> 注意：`//` 語法用來產生註解（comment）直到該行結束。Rust 會忽略註解中所有內容，這會在第三章進一步討論到。

讓我們回到猜謎遊戲程式。你現在就知道 `let mut guess` 會產生一個可變變數叫做 `guess`。在等號（`=`）的另一邊是要綁定給 `guess` 的數值，也就是呼叫 `String::new` 的結果，這是一個回傳新的 `String` 實例（instance）的函式。[`String`][string]<!-- ignore --> 是個標準函式庫提供的字串型別，這是可增長的 UTF-8 編碼文字。

[string]: ../std/string/struct.String.html

`::new` 中的 `::` 語法代表 `new` 是 `String` 型別的*關聯函式（associated function）*。關聯函式是針對型別的實作，在此例中就是 `String`，而不是針對 `String` 特定實例的實作。有些語言會稱之爲*靜態方法（static method）*。

此 `new` 函式建立一個新的空字串。你會在許多型別中找到 `new` 函式，因爲這是函式建立某種新數值的常見名稱。

總結來說， `let mut guess = String::new();` 這行會建立一個可變變數，且目前會得到一個新的空 `String` 實例。

回想一下我們在程式第一行透過 `use std::io;` 來包含標準函式庫中的輸入／輸出功能。現在我們要從 `io` 模組（module）呼叫 `stdin` 函式：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

如果我們沒有將 `use std::io` 這行置於程式最一開始的位置，我們就得寫出 `std::io::stdin` 來呼叫函式。`stdin` 函式會回傳一個 [`std::io::Stdin`][iostdin]<!-- ignore --> 實例，這是代表終端機標準輸入控制代碼（handle）的型別。

[iostdin]: ../std/io/struct.Stdin.html

而程式碼的下個部分 `.read_line(&mut guess)` 會對標準輸入控制代碼呼叫 [`read_line`][read_line]<!-- ignore --> 方法（method）來取得使用者的輸入。我們還傳遞了一個引數（argument）給 `read_line`：`&mut
guess`。

[read_line]: ../std/io/struct.Stdin.html#method.read_line

`read_line` 的任務是取得使用者在標準輸入寫入的任何內容，並放置到字串中，所以它才接收字串作爲引數。字串引數需要是可變的，這樣該方法才能變更字串的內容成使用者的輸入。

`&` 說明此引數是個*引用（reference）*，這讓程式中的多個部分可以取得此資料內容，但不需要每次都得複製資料到記憶體中。引用是個複雜的概念，而 Rust 其中一項主要優勢就是能夠輕鬆又安全地使用引用。你現在還不用知道一堆細節才能完成程式。現在你只需要知道引用和變數一樣，預設都是不可變的。因此你必須寫 `&mut guess` 而不是 `&guess` 才能讓它成爲可變的。（第四章會再全面詳細解釋引用。）

### 使用 `Result` 型別處理可能的錯誤

我們要繼續處理這段程式碼。雖然我們已經討論到第三行了，這仍然是這段單一邏輯程式碼中的一部分。接下來的部分是此方法：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

當你透過 `.foo()` 語法呼叫方法時，通常換行來寫並加上縮排，來拆開一串很長的程式碼會比較好。我們當然可以這樣寫：

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

但是這麼長會很難閱讀，所以最後是能夠分段。現在讓我們來討論這行在做什麼。

如稍早提過的，`read_line` 會將使用者任何輸入轉換至我們傳入的字串，但它還回傳了一個數值，在此例中就是 [`io::Result`][ioresult]<!-- ignore -->。在 Rust 標準函式庫中有一系列的型別都叫做 `Result`，這包含泛型 [`Result`][result]<!-- ignore -->以及每個子模組（submodule）中的特別版本，像是 `io::Result`。

[ioresult]: ../std/io/type.Result.html
[result]: ../std/result/enum.Result.html

`Result` 型別是種[*枚舉（enumerations）*][enums]<!-- ignore -->，常稱爲 *enums*。枚舉是種擁有固定集合數值的型別，而這些數值會被稱之爲枚舉的*變體（variants）*。第六章會更詳細地介紹枚舉。

[enums]: ch06-00-enums.html

對於 `Result` 來說，其變體會是 `Ok` 或 `Err`。`Ok` 變體指的是該動作成功完成，且 `Ok` 內部會包含成功產生的數值。而 `Err` 變體代表動作失敗，且 `Err` 會包含該動作如何與爲何會失敗的資訊。

這些 `Result` 型別的目的是要編碼錯誤處理資訊。`Result` 型別的數值與任何型別的數值一樣，它們都有定義些方法。`io::Result` 的實例有 [`expect` 方法][expect]<!-- ignore --> 讓你能呼叫。如果此 `io::Result` 實例數值爲 `Err` 的話，`expect` 會讓程式崩潰並顯示作爲引數傳給 `expect` 的訊息。如果 `read_line` 回傳 `Err` 的話，這可能就是從底層作業系統傳來的錯誤結果。如果此 `io::Result` 實例數值爲 `Ok` 的話，`expect` 會接收 `Ok` 的回傳值並只回傳該數值，讓你可以使用。在此例中，數值爲使用者輸入到標準輸出的位元組數字。

[expect]: ../std/result/enum.Result.html#method.expect

如果你沒有呼叫 `expect`，程式仍能編譯，但你會收到一個警告：

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust 警告你沒有使用 `read_line` 回傳的 `Result` 數值，這意味著程式沒有處理可能發生的錯誤。

要解決此警告的正確方式是實際進行錯誤處理，但因爲我們只想要當問題發生時直接讓程式崩潰，所以你可以先使用 `expect` 就好。你會在第九章學到如何從錯誤中恢復。

### 透過 `println!` 佔位符印出數值

在結束大括號之前，目前程式碼中還有一行要來討論：也就是以下這行：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

此行會印出存有使用者輸入的字串。其中的大括號 `{}` 是個佔位符（placeholder）：將 `{}` 想成是個小蟹鉗會夾住某個數值。你可以使用大括號印出一個以上的數值，第一個大括號會是格式化字串之後列出的第一個數值，第二個大括號會是第二個數值，以此類推。要呼叫 `println!` 來印數多個數值會如以下所示：

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

此程式碼會印出 `x = 5 and y = 10`。

### 測試第一個部分

讓我們來測試猜謎遊戲中的第一個部分。請使用 `cargo run` 來執行它：

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

到目前爲止，遊戲的第一個部分就完成了：我們取得了鍵盤的輸入然後顯示出來。

## 產生祕密數字

接下來，我們要產生一個能讓使用者猜看看的祕密數字。祕密數字每次都要不同，這樣遊戲才值得多玩幾次。讓我們使用 1 到 100 之間的隨機數字，這樣遊戲才不會太困難。Rust 的標準函式庫並不包含產生隨機數字的功能。然而，Rust 團隊有提供個 [`rand` crate][randcrate]。

[randcrate]: https://crates.io/crates/rand

### 使用 Crate 來取得更多功能

所謂的 crate 是一個 Rust 原始程式碼檔案的集合。我們正在寫的專案屬於*二進制（binary） crate*，也就會是個執行檔。而 `rand` crate 屬於*函式庫（library） crate*，這會包含讓其他程式能夠使用的程式碼。

Cargo 使用外部 crate 的功能正是它的亮點。在我們可以使用 `rand` 來寫程式碼前，我們需要修改 *Cargo.toml* 檔案來包含 `rand` crate 作爲依賴函式庫（dependency）。開啟該檔案然後將以下行數加到 Cargo 自動產生的 `[dependencies]` 標頭（header）段落中最後一行下面：

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

在 *Cargo.toml* 檔案中，標頭以下的所有內容都是該段落的一部分，一直到下個段落出現爲止。`[dependencies]` 段落是告訴 Cargo 此專案要依賴哪些 crate，以及那些 crate 的版本爲何。在此例中，我們透過語意化版本 `0.5.5` 來指定 `rand` crate。Cargo 能夠理解[語意化版本（Semantic Versioning）][semver]<!-- ignore -->，有時也被稱之爲 *SemVer*，這是一種定義版本數字的標準。數字 `0.5.5` 其實是 `^0.5.5` 的縮寫，這代表「任何與版本 0.5.5 的公開 API 相容的版本」。

[semver]: https://semver.org/lang/zh-TW/

現在，在不改變任何程式碼的情況下，讓我們建構（build）專案吧，如範例 2-2 所示。

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.5.5
  Downloaded libc v0.2.62
  Downloaded rand_core v0.2.2
  Downloaded rand_core v0.3.1
  Downloaded rand_core v0.4.2
   Compiling rand_core v0.4.2
   Compiling libc v0.2.62
   Compiling rand_core v0.3.1
   Compiling rand_core v0.2.2
   Compiling rand v0.5.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">範例 2-2：在新增 rand crate 作爲依賴後，執行 `cargo build` 的輸出</span>

你可能會看到不同的版本數字（但多虧有 SemVer，它們都會與程式碼相容！）、不同的行數（依照作業系統可能會不同）以及每行順序可能會不相同。

現在你有了外部依賴，Cargo 會從 *registry* 取得所有 crate 的最新版本訊息，這是份 [Crates.io][cratesio] 的資料副本。Crates.io 是個讓 Rust 生態圈中的每個人都能發佈它們的開源 Rust 專案並讓其他人使用的地方。

[cratesio]: https://crates.io/

在更新 registry 之後，Cargo 會檢查 `[dependencies]` 段落並下載你還沒有的 crate。在此例中，雖然我們只有列出 `rand` 作爲依賴，但 Cargo 還會下載 `libc` 與 `rand_core`，因爲 `rand` 需要這些才能運作。在下載完 crates 之後，Rust 會編譯依賴函式庫以及使用到它們的專案。

如果你立即再次執行 `cargo build` 且沒有作出任何改變的話，你除了 `Finished` 這行以外不會在收到任何輸出。Cargo 知道它已經下載並編譯依賴函式庫了，而且你沒有在 *Cargo.toml* 檔案中再做任何改變。Cargo 也知道你沒有修改任何程式碼，所以也不會再重新編譯它。既然沒事可做，它就只好馬上結束。

如果你開啟 *src/main.rs* 檔案，加些瑣碎的修改，然後儲存並再次建構的話，你會只看到兩行輸出：

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

這幾行表示 Cargo 只更新的你對 *src/main.rs* 檔案的瑣碎修改進行建構。你的依賴沒變，所以 Cargo 知道它可以重複使用已經下載並編譯過的程式碼。它只重新建構了程式碼中的一小部分。

#### 透過 *Cargo.lock* 檔案確保建構可以重現

Cargo 有個機制能確保任何人或你在任何時候重新建構程式碼時，都能產生相同結果。舉例來說，要是下一週 `rand` crate 發佈了版本 0.5.6，其包含重大程式錯誤更新，卻也有個會破壞你的程式碼的回歸（regression）問題，這時會發生什麼事呢？

此問題的答案位於 *Cargo.lock* 檔案中，這會在你第一次執行 `cargo build` 時建立，並位於 *guessing_game* 目錄中。當你第一次建構專案時，Cargo 會決定出符合情境的依賴函式庫版本，然後將它們寫入 *Cargo.lock* 檔案中。當你在未來建構專案時，Cargo 會看到 *Cargo.lock* 的存在並使用其指定的版本，而非重新再次決定該用哪些版本。這讓你有個能自動重現的建構方案。換句話說，你的專案仍會繼續使用 `0.5.5` 直到你顯式升級爲止，這都多虧了 *Cargo.lock* 檔案。

#### 升級 Crate 來取得新版本

當你*真的*想升級 crate 時，Cargo 有提供另一個命令 `update`，這會忽略 *Cargo.lock* 檔案並依據 *Cargo.toml* 指定的規格決定所有合適的最新版本。如果成功的話，Cargo 會將這些版本寫入 *Cargo.lock* 檔案中。

Cargo 預設只會尋找大於 `0.5.5` 且小於 `0.6.0` 的版本。如果 `rand` 有發佈兩個新版本 `0.5.6` 和 `0.6.0`，當你輸入 `cargo update` 時，你會看到以下結果：

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.5.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.5.5 -> v0.5.6
```

此時你也會注意到 *Cargo.lock* 檔案中的變更，指出你現在使用的 `rand` crate 版本爲 `0.5.6`。

如果你想使用 `rand` 版本 `0.6.0` 或任何版本 `0.6.x` 系列，你需要升級 *Cargo.toml* 檔案，如以下所示：

```toml
[dependencies]
rand = "0.6.0"
```

下次你執行 `cargo build` 時，Cargo 將會更新 crate registry ，並依據你指定的新版本來重新評估 `rand` 的確切版本。


[Cargo][doccargo]<!-- ignore --> 與[其生態圈][doccratesio]<!-- ignore --> 還有很多內容可以介紹，我們會在第十四章討論它們。但現在你只需要知道這些就好。Cargo 讓重複使用函式庫變得非常容易，讓 Rustaceans 可以組合許多套件寫出簡潔的專案。

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### 產生隨機數字

現在既然你已經將 `rand` crate 加入 *Cargo.toml* 中，讓我們開始使用 `rand` 吧。下一步是更新 *src/main.rs*，如範例 2-3 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">範例 2-3：新增程式碼來產生隨機數字</span>

首先我們加上 `use` 這行：`use rand::Rng`。`Rng` 特徵（trait）定義了隨機數字產生器實作的方法，所以此特徵必須引入作用域，我們才能使用這些方法。第十章會詳細解釋特徵。

接著，我們在中間加上兩行。`rand::thread_rng` 函式會回傳我們要使用的特定隨機數字產生器：這會位於目前執行緒（thread）並由作業系統提供種子（seed）。然後我們對隨機數字產生器呼叫 `gen_range` 方法。此方法由 `Rng` 特徵所定義，而我們則是用 `use rand::Rng` 陳述式將此特徵引入作用域中。`gen_range` 方法接收兩個數字作爲引數並產生一個在此範圍之間的隨機數字。這個範圍會包含下限但不會包含上限，所以我們需要指定 `1` 與 `101` 來索取 1 到 100 之間的數字。

> 注意：你不可能憑空就知道該使用 crate 中的哪些特徵或是呼叫哪些方法與函式。crate 的使用方式就紀錄在每個 crate 的技術文件中。Cargo 另一大亮點就是你可以執行 `cargo doc --open` 命令，這會建構所有本地端依賴函式庫的技術文件，並在你的瀏覽器中開啟。舉例來說，如果你對 `rand` crate 的其他功能有興趣的話，你可以執行 `cargo doc --open` 然後點擊左側邊欄的 `rand`。

我們在程式碼中間加上的第二行會印出祕密數字。這在開發程式時能用來測試它，不過在最終版本我們會刪除它。如果在遊戲一開始程式就印出答案的話跟本就沒有玩的必要了！

請嘗試執行程式幾次：

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

你應該會得到不同的隨機數字，而且它們都應該要在 1 到 100 的範圍內。做得好！

## 將猜測的數字與祕密數字做比較

現在我們有使用者的輸入與隨機數字，我們可以來比較它們了。這步驟顯示在範例 2-4。注意此程式碼還無法編譯，我們會解釋爲什麼。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">範例 2-4：處理比較兩個數字後的可能數值</span>

首先新加入的第一個部分是另一個 `use` 陳述式，這將 `std::cmp::Ordering` 型別從標準函式庫引入作用域中。就像 `Result` 一樣，`Ordering` 是另一個枚舉，但是 `Ordering` 的變體爲 `Less`、`Greater` 與 `Equal`。當你比較兩個數值時會有三種結果。

然後我們在底下加上五行程式碼來使用 `Ordering` 型別。`cmp` 方法會比較兩個數值，並能在任何可以比較的數值中進行呼叫。其取一個引用至任何你想做比較的數值，在此例中就是將 `guess` 與 `secret_number` 做比較。然後它會回傳我們透過 `use` 陳述式引入作用域的 `Ordering` 枚舉其中一個變體。我們使用 [`match`][match]<!-- ignore --> 表達式來依據透過 `guess` 與 `secret_number` 呼叫 `cmp` 回傳的 `Ordering` 變體來決定下一步要做什麼。

[match]: ch06-02-match.html

`match` 表達式由*分支（arms）*所組成。分支包含一個*模式（pattern）*以及對應的程式碼，這在當 `match` 表達式開頭的數值能與該分支的模式配對時就能執行。Rust 會用 `match` 得到的數值依序遍歷每個分支中的模式。`match` 結構與模式是 Rust 中非常強大的特色，能讓你表達各種程式碼可能會遇上的情形，並確保你有將它們全部處理完。這些特色功能會在第六章與第十八章分別討論其細節。

讓我們看看在此例中使用 `match` 表達式時會發生什麼事。假設使用者猜測的數字是 50 而這次隨機產生的祕密數字是 38。當程式碼比較 50 與 38 時，`cmp` 方法會回傳 `Ordering::Greater`，因爲 50 大於 38。`match` 表達式會取得 `Ordering::Greater` 數值並開始檢查每個分支的模式。它會先查看第一個分支的模式 `Ordering::Less` 並看出數值 `Ordering::Greater` 無法與 `Ordering::Less` 配對，所以它忽略該分支的程式碼，並移到下一個分支。而下個分支的模式 `Ordering::Greater` 能配對到 `Ordering::Greater`！所以該分支對應的程式碼就會執行並印出 `Too big!` 到螢幕上。最後 `match` 表達式就會結束，因爲在此情境中它已經不需要再查看最後一個分支。

然而範例 2-4 的程式碼還無法編譯，讓我們嘗試看看：

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

錯誤的關鍵表示*型別無法配對（mismatched types）*。Rust 有個強力的靜態型別系統，但它也提供了型別推斷。當我們寫 `let mut guess = String::new()` 時，Rust 能夠推斷出 `guess` 應該要是 `String` 讓我們不必親自寫出型別。另一方面，`secret_number` 則是個數字型別。以下是一些可以包含數字 1 到 100 的數字型別：32 位元數字 `i32`、非帶號（unsigned） 32 位元數字 `u32`、64 位元數字 `i64`，以及更多等等。Rust 預設的數字型別爲 `i32`，這就是 `secret_number` 的型別，除非你特地加上型別詮釋，Rust 才會推斷成不同的數字型別。此錯誤原因是因爲 Rust 無法比較將字串與數字型別做比較。

所以我們要將程式從輸入讀取的 `String` 轉換成真正的數字型別，讓我們可以將其與祕密數字做比較。我們可以在 `main` 函式本體加上另一行程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

這行程式碼就是：

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

我們建立了一個變數叫做 `guess`。小等一下，程式不是已經有個變數叫做 `guess`了嗎？的確是的，但 Rust 允許我們*遮蔽（shadow）*之前的 `guess` 數值成新的數值。此功能常用於當你想將一個數值從一個型別轉換成另一個型別的場合中。遮蔽讓我們可以重複使用 `guess` 變數名稱，而不必強迫我們得建立兩個不同的變數，舉例來說像是 `guess_str` 和 `guess`。（第三章會詳細解釋遮蔽。）

我們將 `guess` 綁定給 `guess.trim().parse()` 表達式。表達式中的 `guess` 指的是原本擁有 `String` 來儲存輸入的 `guess`。`String` 中的 `trim` 方法會去除開頭與結尾的任何空白字元。雖然 `u32` 只會包含數字字元，但使用者一定得按下 <span class="keystroke">enter</span> 才能滿足 `read_line`。當使用者按下 <span class="keystroke">enter</span> 時，字串結尾就會加上換行字元。舉例來說，如果使用者輸入 <span class="keystroke">5</span> 並按下 <span class="keystroke">enter</span> 的話，`guess` 看起來會像這樣：`5\n`。`\n` 指的是「換行（newline）」，這是按下 <span class="keystroke">enter</span> 的結果。`trim` 方法能去除 `\n`，讓結果只會是 `5`。

而[字串中的 `parse` 方法][parse]<!-- ignore -->會解析字串成某種數字。因爲此方法可以解析成各種數字型別，我們需要使用 `let guess: u32` 來告訴 Rust 我們想使用的確切數字型別。`guess` 後面的分號（`:`）告訴 Rust 我們會詮釋此變數的型別。Rust 有些內建的數字型別，這裡的 `u32` 是個非帶號（unsigned）的 32 位元整數。對於不大的正整數來說，這是不錯的預設選擇。你會在第三章學到其他數字型別。除此之外，在此範例程式中的 `u32` 詮釋與 `secret_number` 的比較意味著 Rust 也會將 `secret_number` 推斷成 `u32`。所以現在會有兩個相同型別的數值能做比較了！

[parse]: ../std/primitive.str.html#method.parse

`parse` 的呼叫很容易造成錯誤。舉例來說，如果字串包含 `A👍%` 的話，就不可能轉換成數字。因爲它可能會失敗，`parse` 方法回傳的是 `Result` 型別，就和 `read_line` 方法一樣（在之前的[「使用 `Result` 型別處理可能的錯誤」](#使用-result-型別處理可能的錯誤)<!-- ignore -->段落提及）。我們也會用相同的方式來處理此 `Result`，也就是呼叫 `expect` 方法。如果 `parse` 回傳 `Result` 的 `Err` 變體的話，由於它無法從字串建立數字，`expect` 的呼叫會讓遊戲崩潰並顯示我們給予的訊息。如果 `parse` 能成功將字串轉成數字，它將會回傳 `Result` 的 `Ok` 變體，而 `expect` 將會回傳 `Ok` 的內部數值。

現在讓我們執行程式！

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

不錯！儘管我們在猜測數字前加了一些空格，但程式仍能推斷出使用者猜測的是 76。多執行程式幾次來驗證不同種輸入產生的不同行爲：像是正確猜出數字、猜測的數字太高或猜測的數字太低。

我們已經大致上將遊戲完成了，但使用者只能猜測一次。讓我們用迴圈來修改吧！

## 透過迴圈來允許多次猜測

`loop` 關鍵字會產生無限迴圈。我們加入此迴圈讓使用者可能有更多機會可以猜測：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

如同你所見，我們將輸入猜測提示以下的程式碼都移入迴圈中。請確保迴圈中的每一行有用四個空格來做縮排，然後再次執行程式。注意到這裡有個新問題，程式的確照我們所說的去做，但這代表它會不停地尋問要猜測的數字！看起來使用者無法離開遊戲！

使用者的確永遠可以使用快捷鍵 <span class="keystroke">ctrl-c</span> 來中斷程式。但還有其他辦法能逃離這個無限循環，如同在[「將猜測的數字與祕密數字做比較」](#將猜測的數字與祕密數字做比較)<!-- ignore -->中討論 `parse` 時提到的，如果使用者輸入非數字答案的話，程式就會崩潰。使用者可以利用此特性來離開，如以下所示：

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:999:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

輸入 `quit` 就能真的離開遊戲，但是其他非數字輸入也是如此。這並不是最理想的方案，我們想要在猜對數字時自動停止。

### 猜對後離開

讓我們加上 `break` 陳述式來在使用者獲勝時離開遊戲：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

在 `You win!` 之後加上 `break` 這行讓程式在使用者猜對祕密數字時可以離開迴圈。離開迴圈也意味著離開程式，因爲此迴圈是 `main` 中的最後一個部分。

### 處理無效輸入

爲了進一步改善遊戲體驗，當使用者的輸入不是數字時，我們不該讓程式直接崩潰。遊戲程式可以忽略非數字來讓使用者繼續猜測。我們可以修改 `guess` 這段將 `String` 轉換成 `u32` 的程式碼，如範例 2-5 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">範例 2-5：忽略非數字的猜測並要求下一個猜測數字，而不是讓程式崩潰</span>

將 `expect` 的呼叫換成 `match` 表達式，通常就是從錯誤中崩潰改成處理錯誤的方式。你應該還記得 `parse` 回傳的是 `Result` 型別，且 `Result` 是個枚舉，其變體爲 `Ok` 或 `Err`。我們在此使用 `match` 表達式，如同我們對 `cmp` 方法回傳的 `Ordering` 處理方式一樣。

如果 `parse` 能成功將字串轉換成數字，它會回傳 `Ok` 數值內包含的結果數字。該 `Ok` 數值就會配對到第一個分支的模式，然後 `match` 表達式就會回傳 `parse` 產生並填入 `Ok` 內的 `num` 數值。該數字最後就會如我們所願變成我們建立的 `guess` 變數。

如果 `parse` *無法*將字串轉換成數值的話，它會回傳包含與錯誤相關資訊的 `Err` 數值。該 `Err` 數值並不符合 `match` 的第一個分支模式 `Ok(num)`，但它能配對到第二個分支。底線 `_` 是個捕獲數值，在此例中，我們說我們想要配對到所有的 `Err` 數值，無論其中有什麼資訊在裡面。所以程式會執行第二條分支 `continue`，這告訴程式繼續 `loop` 下一個疊代並要求其他猜測數字。如此一來程式就能忽略所有 `parse` 可能會遇到的所有錯誤！

現在程式的每個部分都如我們所預期的了，讓我們試試看：

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

太棒了！有了最後一項小修改，我們終於完成了猜謎遊戲。回想一下程式仍然會印出祕密數字。這在測試很有用，但在實際遊戲時就毀了樂趣了。讓我們刪除會印出祕密數字的 `println!`。範例 2-6 就是最終的程式碼。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">範例 2-6：完整的猜謎遊戲程式碼</span>

## 總結

此時此刻，你已經完成了猜謎遊戲。恭喜你！

此專案讓你能動手實踐並親自體驗許多 Rust 的新概念：`let`、`match`、方法、關聯函式、外部 crate 的使用以及更多等等。在接下來陸續的章節，你將深入學習這些概念。第三章會涵蓋多數程式設計語言都有的概念，像是變數、資料型別與函式，以及如何在 Rust 中使用它們。第四章會探索所有權（ownership），這是 Rust 與其他語言最不同的特色。第五章會討論結構體（structs）與方法語法，而第六章會解釋枚舉。

[variables-and-mutability]:
ch03-01-variables-and-mutability.html#variables-and-mutability

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch02-00-guessing-game-tutorial.md)
> - updated: 2020-10-03
