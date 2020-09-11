## 使用 `if let` 簡化控制流

`if let` 語法讓你可以用 `if` 與 `let` 的組合來以比較不冗長的方式，來處理只在乎其中一種模式而忽略其餘的數值。現在考慮一支程式如範例 6-6 所示，我們配對 `Option<u8>` 的值，但只想在數值爲 3 時執行程式。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">範例 6-6：`match` 只在數值爲 `Some(3)` 時執行程式</span>

我們想在 `Some(3)` 配對到時做些事情，但不想管其他 `Some<u8>` 的值或是 `None`。爲了滿足 `match` 表達式，我們必須在只處理一種變體的分支後面，再加上 `_ => ()`。這樣就加了不少樣板程式碼。

不過我們可以使用 `if let` 以更精簡的方式寫出來，以下程式碼的行爲就與範例 6-6 的 `match` 一樣：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` 接收一個模式與一個表達式，然後用等號區隔開來。它與 `match` 的運作方式相同，表達式的意義與 `match` 相同，然後前面的模式就是第一個分支。

使用 `if let` 可以少打些字、減少縮排以及不用寫多餘的樣板程式碼。不過你就少了 `match` 強制的徹底窮舉檢查。要何時選擇 `match` 還是 `if let` 得依據你在的場合是要做什麼事情，以及在精簡度與徹底檢查之間做取捨。

換句話說，你可以想像 `if let` 是 `match` 的語法糖，它只會配對一種模式來執行程式碼並忽略其他數值。

我們也可以在 `if let` 之後加上 `else`，`else` 之後的程式碼區塊等同於 `match` 表達式中 `_` 情形的程式碼區塊。這樣一來的 `if let` 和 `else` 組合就等同於 `match` 了。回想一下範例 6-4 的 `Coin` 枚舉定義， `Quarter` 變體擁有數值 `UsState`。如果我們希望統計所有不是 25 美分的硬幣的同時，也能繼續回報 25 美分所屬的州的話，我們可以用 `match` 像這樣寫：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

或是我們也可以用 `if let` 和 `else` 表達式這樣寫：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

如果你的程式碼邏輯遇到使用 `match` 表達會太囉唆的話，記得 `if let` 也在你的 Rust 工具箱中可以使用。

## 總結

我們現在含蓋了如何使用枚舉來建立一系列枚舉數值的自訂型別。我們展示了邊準函式庫的 `Option<T>` 型別如何用型別系統來預防錯誤。當枚舉數值其內有資料時，你可以依照你想處理的情況數量，使用 `match` 或 `if let` 來取出並使用那些數值。

你的 Rust 程式碼現在能夠使用結構體與枚舉來表達你所知領域的概念了。在你的 API 建立自訂型別可以確保型別安全，編譯器會保證你的函式只會取得該函式預期的型別數值。

接下來爲了提供組織完善且直觀的的 API 供你的使用者使用，並只表達出使用者確切所需要的內容，我們需要瞭解 Rust 的模組。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [41d9f4c](https://github.com/rust-lang/book/blob/41d9f4c9ae6ba07191f55338e864c713cd49a876/src/ch06-03-if-let.md)
> - updated: 2020-09-11
