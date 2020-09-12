## 使用 `use` 關鍵字引入路徑

我們目前呼叫函式的路徑都很冗長、重複且不方便。舉例來說範例 7-7 我們在考慮要使用絕對或相對路徑來呼叫 `add_to_waitlist` 函式時，每次想要呼叫 `add_to_waitlist` 我們都得指明 `front_of_house` 以及
`hosting`。幸運的是，我們有簡化過程的辦法。我們可以使用 `use` 關鍵字將路徑引入作用域，然後就像它們是本地項目一樣來呼叫它們。

在範例 7-11 中，我們引入了 `crate::front_of_house::hosting` 模組進 `eat_at_restaurant` 函式的作用域中，所以我們要呼叫函式 `add_to_waitlist` 的話我們只需要指明 `hosting::add_to_waitlist`。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs:here}}
```

<span class="caption">範例 7-11：使用 `use` 將模組引入</span>

使用 `use` 將路徑引入作用域就像是在檔案系統中產生符號連結一樣（symbolic link）。在 crate 源頭加上 `use crate::front_of_house::hosting` 後，`hosting` 在作用域內就是個有效的名稱了。使用 `use` 的路徑也會檢查隱私權，就像其他路徑一樣。

你也可以使用 `use` 加上相對路徑來引入項目。範例 7-12 就展示了如何指明相對路徑來達到與範例 7-11 一樣的結果。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs:here}}
```

<span class="caption">範例 7-12：使用 `use` 與相對路徑將項目引入作用域</span>

### 建立慣用的 `use` 路徑

在範例 7-11 你可能會好奇爲何我們指明 `use crate::front_of_house::hosting` 然後在 `eat_at_restaurant` 呼叫，而不是直接用 `use` 指明 `add_to_waitlist` 函式的整個路徑就好。像範例 7-13 這樣寫。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs:here}}
```

<span class="caption">範例 7-13：使用 `use` 將 `add_to_waitlist` 函式引入作用域，但這較不符合習慣</span>

雖然範例 7-11 與範例 7-13 都能完成相同的任務，但是範例 7-11 使用 `use` 講函式引入作用域的方法比較符合習慣用法。使用 `use` 將函式的上層模組引入作用域，讓我們必須在呼叫函式時得指明對應模組。這樣清楚知道該函式並非本地定義的，同時一樣能簡化路徑。範例 7-13 的程式碼會不清楚 `add_to_waitlist` 是在哪定義的。

另一方面，如果是要使用 `use` 引入結構體、枚舉或其他項目的話，直接指明完整路徑反而是符合習慣的方式。範例 7-14 顯示了將標準函式庫的 `HashMap` 引入二進制 crate 作用域的習慣用法。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">範例 7-14：引入 `HashMap` 進作用域的習慣用法</span>

此習慣沒什麼強硬的理由：就只是大家已經習慣這樣的用法來讀寫 Rust 的程式碼。

這樣的習慣有個例外，那就是如果我們將兩個相同名稱的項目使用 `use` 陳述式引入作用域時，因爲 Rust 不會允許。範例 7-15 展示了如何引入兩個同名但屬於不同模組的 `Result` 型別進作用域中並使用的方法。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">範例 7-15：要將兩個同名的型別引入相同作用域的話，必須使用它們所屬的模組</span>

如同你所見使用對應的模組可以分辨出是再使用哪個 `Result` 型別。如果我們直接指明 `use std::fmt::Result` 和 `use std::io::Result` 的話，我們會在同一個作用域中擁有兩個 `Result` 型別，這樣一來 Rust 就無法知道我們想用的 `Result` 是哪一個。

### 使用 `as` 關鍵字提供新名稱

要在相同作用域中使用 `use` 引入兩個同名型別的話，還有另一個辦法。在路徑之後，我們可以用 `as` 指定一個該型別在本地的新名稱，或者說別名。範例 7-16 展示重寫了範例 7-15，將其中一個 `Result` 型別使用 `as` 重新命名。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">範例 7-16：使用 `as` 將型別引入作用域的同時重新命名</span>

在第二個 `use` 陳述式，我們選擇了將 `std::io::Result` 型別重新命名爲 `IoResult`，這樣就不會和同樣引入作用域內 `std::fmt` 的 `Result` 有所衝突。範例 7-15 與 範例 7-16 都屬於習慣用法，你可以選擇你比較喜歡的方式！

### 使用 `pub use` 重新匯出名稱

當我們使用 `use` 關鍵字將名稱引入作用域時，該有效名稱在新的作用域中是私有的。要是我們希望呼叫我們這段程式碼時，也可以使用這個名稱的話（就像該名稱是在此作用域內定義的），我們可以組合 `pub` 和 `use`。這樣的技巧稱之爲 *重新匯出（re-exporting）*，因爲我們將項目引入作用域，並同時公開給其他作用域引用。

範例 7-17 顯示將範例 7-11 在源頭模組中原本的 `use` 改成 `pub use`。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs:here}}
```

<span class="caption">範例 7-17：使用 `pub use` 使名稱公開給任何程式的作用域中引用</span>

使用 `pub use` 可以讓外部程式碼以 `hosting::add_to_waitlist` 的方式來呼叫函式 `add_to_waitlist`。如果我們沒有指明 `pub use`，函式 `eat_at_restaurant` 仍可以在它的作用域呼叫 `hosting::add_to_waitlist`，但外部程式碼就無法利用這個新的路徑。

當程式碼的內部結構與使用程式的開發者對於該領域所想像的結構不同時，重新匯出會很有用。我們再次用餐廳做比喻的話就像是，經營餐廳的人可能會想像餐廳是由「前端」與「後端」所組成，但光顧的顧客可能不會用這些術語來描繪餐廳的每個部分。使用 `pub use` 的話，我們可以用某種架構寫出程式碼，再以不同的架構對外公開。這樣讓我們的的函式庫可以完整的組織起來，且對開發函式庫的開發者與使用函式庫的開發者都提供友善的架構。

### 使用外部套件

在第二章我們寫了一支猜謎遊戲專案時，有用到一個外部套件叫做 `rand` 來取得隨機數字。要在專案內使用 `rand` 的話，我們會在 *Cargo.toml* 加上此行：

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```


在 *Cargo.toml* 新增  `rand` 作爲依賴函式庫會告訴 Cargo 要從 [crates.io](https://crates.io/) 下載  `rand` 以及其他相關的依賴，讓我們可專案可以使用 `rand`。

接下來要將 `rand` 的定義引入我們套件的作用域的話，我們加上一行 `use` 後面接著 crate 的名稱 `rand` 然後列出我們想要引入作用域的項目。回想一下在第二章[「產生隨機數字」][rand]<!-- ignore -->的段落，我們將 `Rng` 特徵引入作用域中，並呼叫函式 `rand::thread_rng`：

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust 社群成員在 [crates.io](https://crates.io/) 發佈了不少套件可供使用，要將這些套件引入到你的套件的步驟是一樣的。在你的套件的 *Cargo.toml* 檔案列出它們，然後使用 `use` 將這些 crate 內的項目引入作用域中。

請注意到標準函式庫（`std`）對於我們的套件來說也是一個外部 crate。由於標準函式庫會跟著 Rust 語言發佈，所以我們不需要更改 *Cargo.toml* 來包含 `std`。但是我們仍然需使用 `use` 來將它的項目引入我們套件的作用域中。舉例來說，要使用 `HashMap` 我們可以這樣寫：

```rust
use std::collections::HashMap;
```

這是個用標準函式庫的 crate 名稱 `std` 起頭的絕對路徑。

### 使用巢狀路徑來大量的 `use` 行數

如果我們要使用在相同 crate 或是相同模組內定義的數個項目，針對每個項目都單獨寫一行的話，會佔據我們檔案內很多空間。舉例來說，範例 2-4 中的猜謎遊戲我們用了這兩個 `use` 陳述式來引入作用域中：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

我們可以改使用巢狀路徑（nested paths）來只用一行就能將數個項目引入作用域中。我們先指明相同路徑的部分，加上雙冒號，然後在大括號內列出各自不同的路徑部分，如範例 7-18 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">範例 7-18：使用巢狀路徑引入有部分相同前綴的數個路徑至作用域中</span>

在較大的程式中，使用巢狀路徑將相同 crate 或相同模組中的許多項目引入作用域，可以大量減少 `use` 陳述式的數量！

我們可以在路徑中的任何部分使用巢狀路徑，這在組合兩個享有相同子路徑的 `use` 陳述式時非常有用。舉例來說，範例 7-19 顯示了兩個 `use` 陳述式：一個將 `std::io` 引入作用域，另一個將 `std::io::Write` 引入作用域。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">範例 7-19：兩個 `use` 陳述式且其中一個是另一個的子路徑</span>

這兩個路徑的相同部分是 `std::io`，這也是整個第一個路徑。要將這兩個路徑合爲一個 `use` 陳述式的話，我們可以在巢狀路徑使用 `self`，如範例 7-20 所示。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">範例 7-20：組合範例 7-19 的路徑爲一個 `use` 陳述式</span>

此行就會將 `std::io` 和 `std::io::Write` 引入作用域。

### 全域運算子

如果我們想要將在一個路徑中所定義的*所有*公開項目引入作用域的話，我們可以在指明路徑之後加上全域（glob）運算子 `*`：

```rust
use std::collections::*;
```

此 `use` 陳述式會將 `std::collections` 定義的所有公開項目都引入作用域中。不過請小心使用全域運算子！它容易讓我們無法分辨作用域內的名稱，以及程式中使用的名稱是從哪定義來的。

全域運算子很常用在 `tests` 模組下，將所有東西引入測試中。我們會在第十一章的[「如何寫測試」][writing-tests]<!-- ignore -->段落來討論。 全域運算子也常拿來用在 prelude 模式中，你可以查閱[標準函式庫的技術文件](../std/prelude/index.html#other-preludes)<!-- ignore -->來瞭解此模式的更多資訊。

[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
> - updated: 2020-09-11
