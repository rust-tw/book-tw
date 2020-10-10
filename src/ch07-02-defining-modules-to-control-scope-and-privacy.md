## 定義模組來控制作用域與隱私權

在此段落，我們將討論模組以及其他模組系統的部分，像是*路徑（paths）* 允許你來命名項目，而 `use` 關鍵字可以將路徑引入作用域，再來 `pub` 關鍵字可以讓指定的項目對外公開。我們還會討論到 `as` 關鍵字、外部套件以及全域（glob）運算子。現在讓我們先專注在模組吧！

*模組（Modules）* 能讓我們在 crate 內組織程式碼成數個群組以便使用且增加閱讀性。模組也能控制項目的*隱私權*，也就是該項目能否被外部程式碼*公開（public）* 使用，或者只作為內部*私有（private）* 實作細節，對外是無法使用的。

舉例來說，讓我們建立一個提供餐廳功能的函式庫 crate。我們定義一個函式簽名不過本體會是空的，好讓我們專注在程式組織，而非餐廳程式碼的實作。

在餐飲業中，餐廳有些地方會被稱作*前端（front of house）*而其他部分則是*後端（back of house）*。前端是消費者的所在區域，這裡是安排顧客座位、點餐並結帳、吧台調酒的地方。而後台則是主廚與廚師工作的廚房、洗碗工洗碗以及經理管理行政工作的地方。

要讓我們的 crate 像真正的餐廳一樣的話，我們可以組織函式進入模組中。要建立一個新的函式庫叫做 `restaurant` 的話，請執行 `cargo new --lib restaurant`。然後將範例 7-1 的程式碼放入 *src/lib.rs* 中，這定義了一些模組與函式簽名。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs:here}}
```

<span class="caption">範例 7-1：`front_of_house` 模組包含了其他擁有函式的模組</span>

我們用 `mod` 關鍵字加上模組的名稱（在此例為 `front_of_house`）來定義一個模組，並用大括號涵蓋模組的本體。在模組中，我們可以再包含其他模組，在此例中我們包含了 `hosting` 和 `serving`。模組還能包含其他項目，像是結構體、枚舉、常數、特徵、或像是 範例 7-1 的函式。

使用模組的話，我們就能將相關的定義組合起來，並用名稱指出會與它們互相關聯。程式設計師在使用此程式碼時就能快速找到他們想使用的定義，因為他們就不必遍歷所有的定義，只要觀察依據組合起來的模組名稱就好。要對此程式碼增加新功能的開發者也能知道該將程式碼放在哪裡，以維持程式碼的組織。

稍早我們提到說 *src/main.rs* 和 *src/lib.rs* 屬於 crate 的源頭。之所以這樣命名的原因是因為這兩個文件的內容都會在 crate 源頭模組架構中組成一個模組叫做 `crate`，這樣的結構稱之為*模組樹（module tree）*。

範例 7-2 顯示了範例 7-1 的模組樹架構。

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">範例 7-2：範例 7-1 的模組樹</span>

此樹顯示了有些模組是包含在其他模組內的（比方說 `hosting` 就在 `front_of_house` 底下）。此樹也顯示了有些模組是其他模組的*同輩（siblings）*，代表它們是在同模組底下定義的（`hosting` 和 `serving` 都在 `front_of_house` 底下定義）。繼續沿用家庭來譬喻的話，如果模組 A 被包含在模組 B 中，我們會說模組 A 是模組 B 的*下一代（child）*，而模組 B 是模組 A 的*上一代（parent）*。注意到整個模組樹的跟是一個隱性模組叫做 `crate`。

模組樹可能會讓你想到電腦中檔案系統的目錄樹，這是一個非常恰當的比喻！就像檔案系統中的目錄，你使用模組來組織你的程式碼。而且就像目錄中的檔案，我們需要有方法可以找到我們的模組。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch07-01-packages-and-crates.md)
> - updated: 2020-09-11
