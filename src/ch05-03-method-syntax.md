## 方法語法

**方法**（Methods）和函式類似，我們用 `fn` 關鍵字並加上它們名稱來宣告，它們都有參數與回傳值，然後它們包含一些程式碼能夠在其他地方呼叫方法。和函式不同的是，方法是針對結構體定義的（或是枚舉和特徵物件，我們會在第六章與第十七章分別介紹它們），且它們第一個參數永遠是 `self`，這代表的是呼叫該方法的結構體實例。

### 定義方法

讓我們把 `Rectangle` 作為參數的 `area` 函式轉換成定義在 `Rectangle` 內的 `area` 方法，如範例 5-13 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">範例 5-13：在 `Rectangle` 中定義 `area` 方法</span>

要定義 `Rectangle` 中的方法，我們先為 `Rectangle` 加個 `impl`（implementation） 區塊來開始。所有在此區塊的內容都跟 `Rectangle` 型別有關。再來將 `area` 移入 `impl` 的大括號中，並將簽名中的第一個參數（在此例中是唯一一個）與其本體中用到的地方改成 `self`。在 `main` 中我們原先使用 `rect1` 作為引數呼叫的 `area`，可以改成使用**方法語法**（method syntax）來呼叫 `Rectangle` 的 `area` 方法。方法語法在實例後面呼叫，我們在其之後加上句點、方法名稱、括號然後任何所需的引數。

在 `area` 的簽名中，我們使用 `&self` 而非 `rectangle: &Rectangle`。`&self` 是 `self: &Self` 的簡寫。在一個 `impl` 區塊內，`Self` 型別是該 `impl` 區塊要實作型別的別名。方法必須有個叫做 `self` 的 `Self` 型別作為它們的第一個參數，所以 Rust 讓你在寫第一個參數時能直接簡寫成 `self`。注意到我們在 `self` 縮寫的前面仍使用 `&`，已表示此方法是借用 `Self` 的實例，就像我們在 `rectangle: &Rectangle` 做的一樣。就和其他參數一樣，方法可以選擇拿走 `self` 的所有權、像我們這裡借用不可變的 `self` 或是借用可變的 `self`。


我們之所以選擇 `&self` 的原因和我們在之前函式版本的 `&Rectangle` 一樣，我們不想取得所有權，只想讀取結構體的資料，而非寫入它。如果我們想要透過方法改變實例的數值的話，我們會使用 `&mut self` 作為第一個參數。而只使用 `self` 取得所有權的方法更是非常少見，這種使用技巧通常是為了想改變 `self` 成你想要的樣子，並且希望能避免原本被改變的實例繼續被呼叫。

使用方法而非函式最大的原因是，除了可以使用方法語法而不必在方法簽名重複 `self` 的型別之外，其更具組織性。我們將所有一個型別所能做的事都放入 `impl` 區塊中了，而不必讓未來的使用者在茫茫函式庫中尋找 `Rectangle` 的功能。

另外我們還可以選擇將方法的名稱取作其結構體的其中一個欄位。舉例來說，我們也可以在 `Rectangle` 定義一個 `width` 方法：

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

這裡我們選擇讓 `width` 方法判斷實例的 `width` 是否大於 0：如果是的話回傳 `true`；如果為 0 的話就回傳 `false`。我們可以讓欄位與方法擁有相同的名稱，並作為任何用途使用。在 `main` 中，當我們在 `rect1.width` 後方加上括號，Rust 就會知道我們指的是 `width` 方法。當我們沒有使用括號時，Rust 會知道我們指的是 `width` 欄位。

雖然不是必定的做法，但通常我們將方法名稱與欄位設為一樣時，我們希望它只回傳該欄位的數值而已。像這樣的方法稱為 **getter**，Rust 並不會像其他語言那樣自動為結構體欄位實作它們。Getter 常用於將欄位隱藏起來，但提供個公開方法並只限讀取該欄位，來做為該型別的公開 API。我們會在第七章討論什麼是公開與私有，以及如何設計方法或欄位為公開或私有的。

> ### `->` 運算子跑去哪了？
>
> 在 C 與 C++ 中，我們有兩種呼叫方式的運算元：我們會用 `.` 來直接呼叫物件的方法；用 `->` 來呼叫需要先解引用的物件。換句話說，如果 `object` 是指標的話，`object->something()` 就會像是`(*object).something()`。
>
> Rust 沒有提供 `->` 這樣的運算子。相反地 Rust 有個功能叫做**自動引用與解引用（automatic referencing and dereferencing）**。呼叫方法是 Rust 少數會有這樣行為的地方。
>
> 運作方式如下：當你呼叫方法像是 `object.something()` 時，Rust 會自動加上`&`、`&mut` 或 `*`，以便符合方法簽名。換句話說，以下範例是相同的：
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 第一個呼叫簡潔多了，這種自動引用的行為之所以可行是因為方法有明確的 `self` 引用型別。依據接收者的方法名稱，Rust 可以知道該方法是在讀取（`&self`）、可變的（`&mut self`）或是會消耗的（`self`）。而 Rust 之所以允許借用方法接收者成隱式的原因，是因為這可以讓所有權更易讀懂。

### 擁有更多參數的方法

讓我們來練習再實作另一個 `Rectangle` 的方法。這次我們要 `Rectangle` 的實例可以接收另一個 `Rectangle` 實例，要是 `self` 本身（第一個 `Rectangle`）可以包含另一個 `Rectangle` 的話我們就回傳 `true`，不然的話就回傳 `false`。也就是我們希望定一個方法 `can_hold` ，如範例 5-14 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">範例 5-14：使用一個還沒定義完的方法 `can_hold`</span>

然後我們預期的輸出結果會如以下所示，因為 `rect2` 的兩個維度都比 `rect1` 小，但 `rect3` 比 `rect1` 寬：

```text
rect1 能容納 rect2 嗎？true
rect1 能容納 rect3 嗎？false
```

我們知道我們要定義方法的話，它一定得在 `impl Rectangle` 區塊底下。方法的名稱會叫做 `can_hold`。它會取得另一個 `Rectangle` 的不可變引用作為參數。我們可以從程式碼呼叫方法的地方來知道參數的可能的型別：`rect1.can_hold(&rect2)` 傳遞了 `&rect2`，這是一個 `rect2` 的不可變引用，同時也是 `Rectangle` 的實例。這是合理的，因為我們只需要讀取 `rect2`（而不是寫入，寫入代表我們需要可變引用），且我們希望 `main` 能夠保持 `rect2` 的所有權，好讓我們之後能在繼續使用它來呼叫 `can_hold` 方法。`can_hold` 的回傳值會是布林值，然後實作細節會是檢查 `self` 的寬度與長度是否都大於其他 `Rectangle` 的寬度與長度。讓我們加入範例 5-13 的 `can_hold` 方法到 `impl` 區塊中，如範例 5-15 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">範例 5-15：在 `Rectangle` 中實作了取得其他 `Rectangle` 作為參數的 `can_hold` 方法</span>

當我們用範例 5-14 的 `main` 函式執行此程式碼的話，我們會得到預期的輸出結果。方法可以在參數 `self` 之後接收更多參數，而那些參數就和函式中的參數用法一樣。

### 關聯函式

所有在 `impl` 區塊內的方法都屬於**關聯函式（associated functions）**，因為它們都與 `impl` 實作的型別相關。要是有方法不需要自己的型別實例的話，我們可以定義個沒有 `self` 作為它們第一個參數的關聯函式（因此不會被稱作方法）。我們已經在 `String` 型別使用過 `String::from` 這種關聯函式了。

不屬於方法的關聯函式很常用作建構子，來產生新的結構體實例。舉例來說，我們可以提供一個只接收一個維度作為參數的關聯函式，讓它賦值給寬度與長度，讓我們可以用 `Rectangle` 來產生正方形，而不必提供兩次相同的值：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

要呼叫關聯函式的話，我們使用 `::` 語法並加上結構體的名稱。比方說 `let sq = Rectangle::square(3);`。此函式用結構體名稱作為命名空間，`::` 語法可以用在關聯函式以及模組的命名空間，我們會在第七章介紹模組。

### 多重 `impl` 區塊

每個結構體都允許有數個 `impl` 區塊。舉例來說，範例 5-15 與範例 5-16 展示的程式碼是一樣的，它讓每個方法都有自己的 `impl` 區塊。

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">範例 5-16：使用多重 `impl` 來重寫範例 5-15</span>

這邊我們的確沒有將方法拆為 `impl` 區塊的理由，不過這樣的語法是合理的。我們會在第十章介紹泛型型別與特徵，看到多重 `impl` 區塊是非常實用的案例。

## 總結

結構體讓你可以自訂對你的領域有意義的型別。使用結構體的話，你可以讓每個資料部分與其他部分具有相關性，並為每個部分讓程式更好讀懂。在 `impl` 區塊中，你可以定義與你的型別有關的函式，而方法就是其中一種關聯函式，能讓你指定你的結構體能有何種行為。

但是結構體並不是自訂型別的唯一方法：讓我們看下去 Rust 的枚舉功能，讓你的工具箱可以再多一項可以使用的工具。
