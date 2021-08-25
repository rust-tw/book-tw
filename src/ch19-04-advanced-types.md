## 進階型別

我們提及 Rust 的型別系統有諸多特色，不過尚未深入討論。本章將從一般角度切入討論新型別（newtype）並檢驗為何作為型別來說，新型別非常好用。再來，接續看看型別別名（type alias）這個類似新型別但語意上不盡相同的特色。我們也會探討 `!` 型別與動態大小型別（dynamically sized type）。

### 透過新型別模式達成型別安全與抽象

> 注意：接下來一節假定你已閱讀前面的章節 [「使用新型別模式替外部型別實作外部特徵」][使用新型別模式]。

目前為止，我們討論過的任務中，新型別模式皆游刃有餘，包括靜態強制不讓值被混淆，同時能表示該值的單位。在範例 19-15 可以見到如何善用新型別表示該值的單位：回憶一下，`Millimeters` 與 `Meters` 將 `u32` 的值封裝在新型別內，若我們寫了一個函式需要型別為 `Millimeters` 的參數，我們不可能編譯出一支可以誤傳 `Meters` 型別或 `u32` 來呼叫這個函式的程式。

另一個新型別的使用情境是替一型別的實作細節建立抽象層：如果我們直接將新型別作為限制可用功能的手段，新型別就可以公開有別於私有內部型別的 API。

新型別也可以隱藏內部實作。例如，我們可以提供 `People` 型別，封裝用來儲存人們的 ID 與姓名之間的關聯的 `HashMap<i32, String>`。使用 `People` 的程式碼僅能與我們提供的公開 API 互動，例如透過一個方法替 `People` 集合添加名字字串，這段程式碼就不需知道內部會將 `i32` 作為 ID 並映射到姓名上。我們在第十七章的[「隱藏實作細節的封裝」][隱藏實作細節的封裝]一節也曾提及，利用新型別模式來達到封裝與隱藏實作細節，不失為一種輕量的方法。

### 透過型別別名建立型別同義詞

此外，Rust 提供了替一個既有型別宣告型別別名的方式。對此我們會使用 `type` 關鍵字，例如我們可以建立 `i32` 的別名 `Kilometers`，如範例所示：

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

現在，`Kilometers`  別名就是 `i32` 的**同義詞**。不像我們在範例 19-15 建立的 `Millimeters` 與 `Meters` 型別，`Kilometers` 並非獨立的新型別。型別為 `Kilometers` 的值會被當作型別是 `i32` 的值。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

由於 `Kilometers` 與 `i32` 實際上是同個型別，所以兩者可以相加，我們也可以將 `Kilometers` 值傳入需要 `i32` 參數的函式。然而，這種作法並不像前面討論的新型別模式一樣有益於型別檢查。

型別同義詞的主要使用情境在於減少重複。例如我們有一個又臭又長的型別：

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

到處在函式簽名與型別註解寫這個型別既累人又容易失誤。想像你有一個專案的程式碼都長得像範例 19-24。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-24/src/main.rs:here}}
```

<span class="caption">範例 19-24：在多處使用很長的型別</span>

使用型別別名減少重複，讓程式碼更可控。範例 19-25，我們替落落長的型別導入一個 `Thunk` 別名，所有用到該型別之處都能用短小的 `Thunk` 替代。

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-25/src/main.rs:here}}
```

<span class="caption">範例 19-25：導入型別別名 `Thunk` 來減少重複</span>

這段程式碼更容易讀寫了！選擇有意義的型別別名也有助於溝通傳達你的意圖（*thunk* 是一個表示會在未來對此程式碼求值，所以很適用表達儲存起來的閉包）。

型別別名同樣十分常用在 `Result<T, E>` 來減少重複。試想標準函式庫的 `std::io` 模組，輸入輸出（I/O）操作通常會藉由回傳 `Result<T, E>` 來處理失敗的操作。標準函式庫有個 `std::io::Error` 結構體來表示所有可能的 I/O 錯誤。許多在 `std::io` 內的函式會回傳 `E` 為 `std::io::Error` 的 `Result<T, E>` ，例如這些 `Write` 特徵下的函式：

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

這些 `Result<..., Error`> 不斷重複，有鑑於此，`std::io` 宣告了這個型別的別名：

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

由於這個宣告是在 `std::io` 模組內，因此我們可直接使用完全限定的別名 `std::io::Result<T>`，實際上就是 `E` 預先填入 `std::io::Error` 的 `Result<T, E>`。最終，`Write` 特徵的函式簽名就會長得這樣：

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

型別別名有助於兩個面向：讓程式碼更容易撰寫，**且**對所有 `std::io` 提供一致的介面。因為它僅僅是別名，所以就是一個 `Result<T, E>` 而已，這意味著我們能使用任何可與 `Result<T, E>` 互動的方法，以及使用類似 `?` 運算子這種特殊語法。

### 永不回傳的永不型別

Rust 有一個特殊的型別叫做 `!`，由於它沒有任何值，在型別理論的行話中又稱為**空型別**（empty type）。不過我們更喜歡稱之為**永不型別**（never type），因為當一個函式永遠不會回傳，永不型別將會替代原本的回傳型別。這裡來個範例:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

這段程式碼可讀作「函式 `bar` 永不回傳」。永不回傳的函數稱為**發散函式**（diverging function），我們無法建立 `!` 型別，所以 `bar` 永遠無法回傳。

不過，若永遠無法替這個型別建立值，那要這個型別幹嘛呢？回想一下，範例 2-5 的程式碼，在我們的範例 19-26 又重現了。

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

<span class="caption">範例 19-26：`match` 其中一個分支結束在 `continue`</span>

當時我們跳過了這段程式碼的一些細節。在第六章[「`match` 控制流運算子」][match-控制流運算子]一節，我們探討了每個 `match` 分支必須回傳相同的型別，所以，例如以下程式碼就不能執行：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

這段程式碼中 `guess` 的型別必須是**同時是**整數與字串，並且 Rust 要求 `guess` 只能是一種型別。那 `contiunue` 回傳了什麼？範例 19-26 中，為什麼允許一個分支回傳 `u32` 但同時有另一分支結束在 `continue`？


如你所猜，`continue` 具有 `!` 值。意即當 Rust 根據兩個分支來推算 `guess` 型別時，會觀察到前者會是 `u32`，而後者是 `!`。因為 `!` 永遠不會有值，Rust 於是決定 `guess` 的型別為 `u32`。

描述這種行為的正確方式是：`!` 型別的表達式能夠轉型為任意其他型別。我們允許 `match` 分支結束在 `continue` 就是因為 `continue` 不會回傳任何值，相反地，它將控制流移至迴圈的最上面，所以在 `Err` 的情況，我們不會對 `guess` 賦值。


永不型別在使用 `panic!` 巨集很實用。還記得當我們對 `Option<T>` 呼叫 `unwrap` 函式，會產生一個值或是恐慌嗎？這裡就是它的定義：

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

和範例 19-26 `match` 相同的情況，在這段程式碼再度上演：Rust 看到 `val` 的型別是 `T` 且 `panic` 是 `!` 型別，所以 `match` 表達式的總體結果是 `T`。這段程式碼可執行是因為 `panic!` 會結束程式而不會產生值。當遇上 `None` 的情形，我們不會從 `unwrap` 回傳任何值，所以這段程式碼合法有效。


最後一個具有 `!` 型別的表達式是 `loop`：

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

這裡迴圈永不結束，所以 `!` 就是迴圈表達式的值。但當我們有一個 `break` 時，這就不成立了，因為迴圈會在抵達 `break` 時終止。

### 動態大小型別與 `Sized` 特徵

由於 Rust 需要了解特定細節，例如需替特定型別之值分配多少空間，導致其類型系統有個地方令人困惑，就是**動態大小型別**（dynamically sized type）的概念。 有時稱為 *DST* 或**不定大小（unsize）型別**，這些型別賦予我們寫出僅能在執行期（runtime）得知值的大小之程式碼。

讓我們深入研究一個貫穿全書到處使用的動態大小型別 `str` 的細節。你沒看錯，不是 `&str` 而是 `str` 本身就是 DST。在執行期前我們無從得知字串多長，也就表示無法建立一個型別為 `str` 的變數，更不能將 `str` 型別作為引數。試想以下不能執行的程式碼：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust 必須知道該配置多少記憶體給特定型別之值，且所有該型別之值都會使用相同的記憶體量。若 Rust 允許我們寫出這種程式碼，代表這兩個 `str` 值會用相同的空間大小，但它們長度不同：`s1` 需要 12 位元組來儲存，而 `s2` 需要 15 位元組。這就是為什麼不可能建立一個持有動態大小型別的變數。

那我們該如何是好？這種情況下，你其實已經知道答案：將 `s1` 與 `s2` 的型別從 `str` 改成 `&str`。回憶以下，第四章[「字串切片」][字串-slice]一節我們說了，切片資料結構會儲存該切片的開始位置與長度。

雖然 `&T` 是單一的值，儲存了 `T` 所在的記憶體位址，`&str` 卻儲存**兩個**值：`str` 的位址與它的長度。如此一來，無論 `&str` 指向的字串有多長，我們都可以在編譯期得知 `&str` 的大小。一般來說，這就是動態大小型別在 Rust 中的使用方式，通常具有額外的資料紀錄動態資訊的大小。動態大小型別的黃金法則即是我們必將動態大小型別的值放在指向某種指標之後。

我們將各種指標與 `str` 結合，例如 `Box<str>` 或 `Rc<str>`。事實上，你早已看過此類作法，不過是在其他動態大小型別上看過，那個型別就是特徵（trait）。每個特徵都是一個動態大小型別，我們可以透過使用特徵的名字來指涉它。在第十七章的[「允許不同型別數值的特徵物件」][允許不同型別數值的特徵物件]部分，我們提及欲將特徵做為特徵物件來使用，必須將特徵放在指標之後，例如 `&dyn Trait` 或 `Box<dyn Trait>`（`Rc<dyn Trait>` 也行）。

為了使用 DST，Rust 提供一個 `Sized` 特徵，來決定一個型別的大小可否在編譯期就確定下來。對於能在編譯期得知大小的所有東西，都會自動實作這個特徵。此外 Rust 自動替所有泛型函式隱含加上 `Sized` 的約束。也就是說若一泛型函數定義如下：

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

實際上就如同寫成這樣：

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

預設情形下，泛型函式只能在編譯器得知大小的型別上使用。然而，你可以加上以下這個特殊語言來放寬這個限制：

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized` 特徵界限代表「`T` 可能是或不是 `Sized`」，而此詮釋會覆蓋原本預設泛型型別必須在編譯期就已知大小。`?Trait` 的語法與語義只能用在 `Sized`，不適用於其他特徵。

也請注意，我們將參數 `t` 的型別由 `T` 轉為 `&T`，是因為這個型別可能不是 `Sized`，所以我們需要將它放在指標之後才能使用之，而在這例子中，我們選擇將它放在引用之後。
接下來，我們會聊聊函式和閉包！

[隱藏實作細節的封裝]: ch17-01-what-is-oo.html#隱藏實作細節的封裝
[字串-slice]: ch04-03-slices.html#字串切片
[match-控制流運算子]: ch06-02-match.html#match-控制流運算子
[允許不同型別數值的特徵物件]: ch17-02-trait-objects.html#允許不同型別數值的特徵物件
[使用新型別模式]: ch19-03-advanced-traits.html#使用新型別模式替外部型別實作外部特徵

> - translators: [Weihang Lo <me@weihanglo.tw>]
> - commit: [5c71aac](https://github.com/rust-lang/book/blob/5c71aac64380f74f34cd9a158cc2b1d9122b5ceb/src/ch19-04-advanced-types.md)
> - updated: 2020-09-16
