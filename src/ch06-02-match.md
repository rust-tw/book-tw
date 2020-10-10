## `match` 控制流運算子

Rust 有個功能非常強大的控制流運算子叫做 `match`，你可以使用一系列模式來配對數值並依據配對到的模式來執行對應的程式。模式（Patterns）可以是字面數值、變數名稱、通配符（wildcards）和其他更多元件來組成。第十八章會涵蓋所有不同類型的模式，以及它們的用途。`match` 強大的地方在於模式表達的清楚程度以及編譯器會確保所有可能的情況都處理了。

你可以想像 `match` 表達式成一個硬幣分類機器：硬幣會滑到不同大小的軌道，然後每個硬幣會滑入第一個符合大小的軌道。同樣地，數值會依序遍歷 `match` 的每個模式，然後進入第一個「配對」到該數值的模式所在的程式碼區塊，並在執行過程中使用。

既然我們都提到硬幣了，就讓我們用它們來作為 `match` 的範例吧！我們可以寫一個接收未知美國硬幣的函式，以類似驗鈔機的方式，決定它是何種硬幣並以美分作為單位回傳其值。如範例 6-3 所示。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">範例 6-3：枚舉以及用枚舉變體作為模式的 `match` 表達式</span>

讓我們一一介紹 `value_in_cents` 函式中 `match` 的每個部分。首先我們使用 `match` 並加上一個表達式，在此例的話就是指 `coin`。這和 `if` 中表達式的用法很像。不過差別在於 `if` 中的表達式必須回傳布林值，而在此它可以是任何型別。在此範例中 `coin` 的型別是我們在第一行定義的枚舉 `Coin`。

接下來是 `match` 的分支，每個分支有兩個部分：一個模式以及對應的程式碼。這邊第一個分支的模式是 `Coin::Penny` 然後 `=>` 會將模式與要執行的程式碼分開來，而在此例的程式碼就只是個 `1`。每個分支之間由逗號區隔開來。

當 `match` 表達式執行時，他會將計算的數據結果依序與每個分支的模式做比較。如果有模式配對到該值的話，其對應的程式碼就會執行。如果該模式與數值不符的話，就繼續執行下一個分支，就像硬幣分類機器。

每個分支對應的程式碼都是表達式，然後在配對到的分支中表達式的數值結果就會是整個 `match` 表達式的回傳值。

如果配對分支的程式碼很短的話，通常就不需要用到的大括號，像是範例 6-3 每個分支就只回傳一個數值。如果你想要在配對分支執行多行程式碼的話，你就可以用大括號。舉例來說，以下程式會在每次配對到 `Coin::Penny` 時印出「幸運幣！」再回傳程式碼區塊最後的數值 `1`：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### 綁定數值的模式

另一項配對分支的實用功能是它們可以綁定配對模式中部分的數值，這讓我們可以取出枚舉變體中的數值。

舉例來說，讓我們改變我們其中一個枚舉變體成擁有資料。從 1999 年到 2008 年，美國在鑄造 25 美分硬幣時，其中一側會有 50 個州不同的設計。不過其他的硬幣就沒有這樣的設計，只有 25 美分會有特殊值而已。我們可以改變我們的 `enum` 中的 `Quarter` 變體成儲存 `UsState` 數值，如範例 6-4所示。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">範例 6-4：修改 `Coin` 枚舉的 `Quarter` 變體來包含一個 `UsState` 數值</span>

讓我們想像我們有一個朋友想要收集所有 50 州的 25 美分硬幣。當我們在排序零錢的同時，我們會在拿到 25 美分時喊出該硬幣對應的州，好讓我們的朋友知道，如果他沒有的話就可以納入收藏。

在此程式中的配對表達式中，我們在 `Coin::Quarter` 變體的配對模式中新增了一個變數 `state`。當 `Coin::Quarter` 配對符合時，變數 `state` 會綁定該 25 美分的數值，然後我們就可以在分支程式碼中使用 `state`，如以下所示：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

如果我們呼叫 `value_in_cents(Coin::Quarter(UsState::Alaska))` 的話，`coin` 就會是 `Coin::Quarter(UsState::Alaska)`。當我們比較每個配對分支時，我們會到 `Coin::Quarter(state)` 的分支才配對成功。此時 `state` 綁定的數值就會是 `UsState::Alaska`。我們就可以在 `println!` 表達式中使用該綁定的值，以此取得 `Coin` 枚舉中 `Quarter` 變體內的值。

### 配對 `Option<T>`

在上一個段落，我們想要在使用 `Option<T>` 時取得 `Some` 內部的 `T` 值。如同枚舉 `Coin`，我們一樣可以使用 `match` 來處理 `Option<T>`！ 相對於比較硬幣，我們要比較的是 `Option<T>` 的變體，不過 `match` 表達式運作的方式一模一樣。

假設我們要寫個接受 `Option<i32>` 的函式，而且如果內部有值的話就將其加上 1。如果內部沒有數值的話，該函式就回傳 `None` 且不再嘗試做任何動作。

拜 `match` 所賜，這樣的函式很容易寫出來，長得就像範例 6-5。

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">範例 6-5：對 `Option<i32>` 使用 `match` 表達式的函式</span>

讓我們來仔細分析 `plus_one` 第一次的執行結果。當我們呼叫 `plus_one(five)`時，`plus_one` 本體中的變數 `x` 會擁有 `Some(5)`。我們接著就拿去和每個配對分支比較。

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

`Some(5)` 並不符合 `None` 這樣的模式，所以我們繼續進行下一個分支。

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` 有符合 `Some(i)` 這樣的模式嗎？這是當然的囉！我們有相同的變體。`i` 會綁定 `Some` 中的值，所以 `i` 會取得 `5`。接下來配對分支中的程式碼就會執行，我們將 1 加入 `i` 並產生新的 `Some` 其內部的值就會是 `6`。

現在讓我們看看範例 6-5 第二次的 `plus_one` 呼叫，這次的 `x` 是 `None`。我們進入 `match` 然後比較第一個分支。

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

配對成功！因為沒有任何數值可以相加，程式就停止並在 `=>` 之後馬上回傳 `None`。因為第一個分支就配對成功了，沒有其他的分支需要在做比較。

用 `match` 與枚舉組合起來在很多地方都很實用。你將會在許多 Rust 程式碼看到這樣的模式，使用 `match` 配對枚舉，綁定內部的資料，然後執行對應的程式碼。一開始使用的確會有點陌生，但當你熟悉以後，你會希望所有語言都能提供這樣的功能。這一直是使用者最愛的功能之一。

### 配對必須是徹底的

我們還有一個 `match` 的細節要討論，今天要是我們像這樣寫了一個有錯誤的 `plus_one` 函式版本，它會無法編譯：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

我們沒有處理到 `None` 的情形，所以此程式碼會產生錯誤。幸運的是這是 Rust 能夠抓到的錯誤。如果我們嘗試編譯此程式的話，我們會得到以下錯誤：

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust 發現我們沒有考慮到所有可能條件，而且還知道我們少了哪些模式！Rust 中的配對必須是*徹底（exhaustive）* 的：我們必須列舉出所有可能的情形，程式碼才能夠被視為有效。尤其是在 `Option<T>` 的情況下，當 Rust 防止我們忘記處理 `None` 的情形時，它也使我們免於以為擁有一個有效實際上卻是空的值。因此要造成之前提過的數十億元錯誤在這邊基本上是不可能的。

### `_` 佔位符

Rust 還有一個模式可以讓我們不必列出所有可能的數值，只需要使用此模式就好。舉例來說 `u8` 可能的數值為 0 到 255，如果我們只在意數值 1、3、5 和 7，我們就不會想要列出 0、2、4、6、8、9 以及剩下一直到 255 的每個值。幸運的是，我們不需要這樣做，我們可以使用特殊模式 `_`：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-11-underscore-placeholder/src/main.rs:here}}
```

`_` 模式會配對任意數值，將它置於所有分支之後，`_`就會配對剩下尚未指明的可能情形。`()` 只是一個單位數值，所以在 `_` 的分支沒有任何事情會發生。所以我們可以說我們不想針對 `_` 佔位符（placeholder）之前沒有列出的可能情形，做任何動作。

不過有時候我們只在意其中*一種*情形的話， `match` 表達式的確會有點囉唆。針對此情形，Rust 提供 `if let`。

而更多有關配對模式的內容可以在[第十八章][ch18-00-patterns]查閱。

[ch18-00-patterns]:
ch18-00-patterns.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [41d9f4c](https://github.com/rust-lang/book/blob/41d9f4c9ae6ba07191f55338e864c713cd49a876/src/ch06-01-defining-an-enum.md)
> - updated: 2020-09-11
