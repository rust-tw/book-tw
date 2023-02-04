## 定義模組來控制作用域與隱私權

在此段落，我們將討論模組以及其他模組系統的部分，像是**路徑**（paths）允許你來命名項目，而 `use` 關鍵字可以將路徑引入作用域，再來 `pub` 關鍵字可以讓指定的項目對外公開。我們還會討論到 `as` 關鍵字、外部套件以及全域（glob）運算子。

首先，讓我們先介紹一些規則好讓你在之後組織程式碼時能更容易理解初步概念。然後我們會再詳細解釋每個規則。

### 模組懶人包

這裡我們先快速帶過模組、路徑、`use` 關鍵字以及 `pub` 關鍵字在編譯器中是怎麼運作的，以及多數開發者會怎麼組織他們的程式碼。我們會在此章節透過範例逐依介紹，不過這裡能讓你快速理解模組是怎麼運作的。

- **從 crate 源頭開始**：在編譯 crate 時，編譯器會先尋找 crate 源頭檔案（函式庫 crate 的話，通常就是 *src/lib.rs*；執行檔 crate 的話，通常就是 *src/main.rs*）來編譯程式碼。
- **宣告模組**：在 crate 源頭檔案中，你可以宣告新的模組，比如說你宣告了一個「garden」模組 `mod garden;`。編譯器會在以下這幾處尋找模組的程式碼：
  - 同檔案內用 `mod garden` 加上大括號，寫在括號內的程式碼
  - *src/garden.rs* 檔案中
  - *src/garden/mod.rs* 檔案中
- **宣告子模組**：除了 crate 源頭以外，其他檔案也可以宣告子模組。舉例來說，你可能會在 *src/garden.rs* 中宣告個 `mod vegetables;`。編譯器會與當前模組同名的目錄底下這幾處尋找子模組的程式碼：
  - 同檔案內，直接用 `mod vegetables` 加上大括號，寫在括號內的程式碼
  - *src/garden/vegetables.rs* 檔案中
  - *src/garden/vegetables/mod.rs* 檔案中
- **模組的路徑**：一旦有個模組成為 crate 的一部分，只要隱私權規則允許，你可以在 crate 裡任何地方使用該模組的程式碼。舉例來說，「garden」模組底下的「vegetables」模組的 `Asparagus` 型別可以用 `crate::garden::vegetables::Asparagus` 來找到。
- **私有 vs 公開**：模組內的程式碼從上層模組來看預設是私有的。要公開的話，將它宣告為 `pub mod` 而非只是 `mod`。要讓公開模組內的項目也公開的話，在這些項目前面也加上 `pub` 即可。
- **`use` 關鍵字**：在一個作用域內，`use` 關鍵字可以建立項目的捷徑，來縮短冗長的路徑名稱。在任何能使用 `crate::garden::vegetables::Asparagus` 的作用域中，你可以透過 `use crate::garden::vegetables::Asparagus;` 來建立捷徑，接著你只需要寫 `Asparagus` 就能在作用域內使用該型別了。

這裡我們建立個執行檔 crate 叫做 `backyard` 來展示這些規則。Crate 的目錄也叫做 `backyard`，其中包含了這些檔案與目錄：

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

此例的 crate 源頭檔案就是 *src/main.rs*，它包含了：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

`pub mod garden;` 這行告訴編譯器要包含在 *src/garden.rs* 中的程式碼，也就是：

<span class="filename">檔案名稱：src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

這裡的 `pub mod vegetables;` 代表 *src/garden/vegetables.rs* 的程式碼也包含在內。而這段程式碼就是：

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

現在讓我們詳細介紹這些規則並解釋如何運作的吧！

### 組織相關程式碼成模組

**模組**（Modules）能讓我們在 crate 內組織程式碼成數個群組以便使用且增加閱讀性。模組也能控制項目的**隱私權**，因為模組內的程式碼預設是私有的。私有項目是內部的實作細節，並不打算讓外部能使用。我們能讓模組與其內的項目公開，讓外部程式碼能夠使用並依賴它們。

舉例來說，讓我們建立一個提供餐廳功能的函式庫 crate。我們定義一個函式簽名不過本體會是空的，好讓我們專注在程式組織，而非餐廳程式碼的實作。

在餐飲業中，餐廳有些地方會被稱作**前台（front of house）**而其他部分則是**後台（back of house）**。前台是消費者的所在區域，這裡是安排顧客座位、點餐並結帳、吧台調酒的地方。而後台則是主廚與廚師工作的廚房、洗碗工洗碗以及經理管理行政工作的地方。

要讓 crate 架構長這樣的話，我們可以組織函式進入模組中。要建立一個新的函式庫叫做 `restaurant` 的話，請執行 `cargo new --lib restaurant`。然後將範例 7-1 的程式碼放入 *src/lib.rs* 中，這定義了一些模組與函式簽名。以下是前台的段落：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">範例 7-1：`front_of_house` 模組包含了其他擁有函式的模組</span>

我們用 `mod` 關鍵字加上模組的名稱（在此例為 `front_of_house`）來定義一個模組，並用大括號涵蓋模組的本體。在模組中，我們可以再包含其他模組，在此例中我們包含了 `hosting` 和 `serving`。模組還能包含其他項目，像是結構體、枚舉、常數、特徵、以及像是範例 7-1 的函式。

使用模組的話，我們就能將相關的定義組合起來，並用名稱指出會與它們互相關聯。程式設計師在使用此程式碼時只要觀察依據組合起來的模組名稱就好，不必遍歷所有的定義。這樣就能快速找到他們想使用的定義。要對此程式碼增加新功能的開發者也能知道該將程式碼放在哪裡，以維持程式碼的組織。

稍早我們提到說 *src/main.rs* 和 *src/lib.rs* 屬於 crate 的源頭。之所以這樣命名的原因是因為這兩個文件的內容都會在 crate 源頭模組架構中組成一個模組叫做 `crate`，這樣的結構稱之為**模組樹（module tree）**。

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

此樹顯示了有些模組是包含在其他模組內的，比方說 `hosting` 就在 `front_of_house` 底下。此樹也顯示了有些模組是其他模組的**同輩（siblings）**，代表它們是在同模組底下定義的，`hosting` 和 `serving` 都在 `front_of_house` 底下定義。如果模組 A 被包含在模組 B 中，我們會說模組 A 是模組 B 的**下一代（child）**，而模組 B 是模組 A 的**上一代（parent）**。注意到整個模組樹的根是一個隱性模組叫做 `crate`。

模組樹可能會讓你想到電腦中檔案系統的目錄樹，這是一個非常恰當的比喻！就像檔案系統中的目錄，你使用模組來組織你的程式碼。而且就像目錄中的檔案，我們需要有方法可以找到我們的模組。
