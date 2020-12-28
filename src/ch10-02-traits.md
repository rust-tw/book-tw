## 特徵：定義共同行為

**特徵**（trait）會告訴 Rust 編譯器特定型別與其他型別共享的功能。我們可以使用特徵定義來抽象出共同行為。我們可以使用特徵界限（trait bounds）來指定泛型為擁有特定行為的任意型別。

> 注意：特徵類似於其他語言常稱作**介面**（interfaces）的功能，但還是有些差異。

### 定義特徵

一個型別的行為包含我們對該型別可以呼叫的方法。如果我們可以對不同型別呼叫相同的方法，這些型別就能定義共同行為了。特徵定義是一個將方法簽名統整起來，來達成一些目的而定義一系列行為的方法。

舉例來說，如果我們有數個結構體各自擁有不同種類與不同數量的文字：結構體 `NewsArticle` 儲存特定地點的新聞故事，然後 `Tweet` 則有最多 280 字元的內容，且有個欄位來判斷是全新的推文、轉推或其他推文的回覆。

我們想要建立個多媒體資料庫來顯示可能存在 `NewsArticle` 或 `Tweet` 實例的資料總結。要達成此目的的話，我們需要每個型別的總結，且我們需要呼叫該實例的 `summarize` 方法來索取總結。範例 10-12 顯示了表達此行為的 `Summary` 特徵定義。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">範例 10-12：`Summary` 特徵包含 `summarize` 方法所定義的行為</span>

我們在此使用 `trait` 關鍵字定義一個特徵，其名稱為 `Summary`。在大括號中，我們宣告方法簽名來描述有實作此特徵的型別行為，在此例就是 `fn summarize(&self) -> String`。

在方法簽名之後，我們並沒有加上大括號提供實作細節，而是使用分號。每個有實作此特徵的型別必須提供其自訂行為的方法本體。編譯器會強制要求任何有 `Summary` 特徵的型別都要有定義相同簽名的 `summarize` 方法。

特徵本體中可以有多個方法，每行會有一個方法簽名並都以分號做結尾。

### 為型別實作特徵

現在我們已經用 `Summary` 特徵定義了所需的行為。我們可以在我們多媒體資料庫的型別中實作它。範例 10-13 顯示了 `NewsArticle` 結構體實作 `Summary` 特徵的方式，其使用頭條、作者、位置來建立 `summerize` 的回傳值。至於結構體 `Tweet`，我們使用使用者名稱加上整個推文的文字來定義 `summarize`，因為推文的內容長度已經被限制在 280 個字元以內了。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">範例 10-13：在型別 `NewsArticle` 與 `Tweet` 實作 `Summary` 特徵</span>

為一個型別實作一個特徵類似於實作一般的方法。不同的地方在於在 `impl` 之後我們加上的是想要實作的特徵，然後在用 `for` 關鍵字加上我們想要實作特徵的型別名稱。在 `impl` 的區塊內我們置入該特徵所定義的方法簽名，我們使用大括號並填入方法本體來為對特定型別實作出特徵方法的指定行為。

在實作完後，我們就能像呼叫正常方法一樣，來呼叫 `NewsArticle` 和 `Tweet` 實例的方法，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs:here}}
```

此程式碼會印出「1 則新推文：horse_ebooks: of course, as you probably already
know, people」。

注意到因為我們將範例 10-13 的 `Summary` 特徵、`NewsArticle` 和 `Tweet` 型別都定義在 *lib.rs* ，所以它們都在同個作用域下。如果我們說此 *lib.rs* 對應的 crate 叫做 `aggregator`，然後有人想要使用我們 crate 的功能來對他們函式庫作用域中定義的結構體實作 `Summary` 特徵的話。他們會需要將該特徵引入作用域，可以像這樣指定 `use aggregator::Summary;`，如此一來就能對他們的型別實作 `Summary`。`Summary` 特徵一樣也必須是公開的才能讓其他 crate 使用。這就是為何我們在範例 10-12 的 `trait` 前面就加上 `pub` 關鍵字。

實作特徵時有一個限制，那就是我們只能在該特徵或該型別位於我們的 crate 時，才能對型別實作特徵。舉例來說我們可以對自訂型別像是 `Tweet` 來實作標準函式庫的 `Display` 特徵來為我們 crate `aggregator` 增加更多功能。因為 `Tweet` 位於我們的 `aggregator` crate 裡面。我們也可以在我們的 crate `aggregator` 內對 `Vec<T>` 實作 `Summary`。因為特徵 `Summary` 也位於我們的 `aggregator` crate 裡面。

但是我們無法對外部型別實作外部特徵。舉例來說我們無法在我們的 `aggregator` crate 裡面對 `Vec<T>` 實作 `Display` 特徵。因為 `Display` 與 `Vec<T>` 都定義在標準函式庫中，並沒有在我們 `aggregator` crate 裡面。此限制叫做「連貫性（coherence）」是程式屬性的一部分。更具體來說我們會稱作「孤兒原則（orphan rule）」，因為上一代（parent）型別不存這在。此原則能確保其他人的程式碼不會破壞你的程式碼，反之亦然。沒有此原則的話，兩個 crate 可以都對相同型別實作相同特徵，然後 Rust 就會不知道該用哪個實作。

### 預設實作

有時候對特徵內的一些或所有方法定義預設行為是很實用的，而不必要求每個型別都實作所有方法。然後當我們對特定型別實作特徵時，我們可以保留或覆蓋每個方法的預設行為。

範例 10-14 展示如何在 `Summary` 特徵內指定  `summarize` 方法的預設字串，而不必像範例 10-12 只定義了方法簽名。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">範例 10-14：`Summary` 特徵定義了 `summarize` 方法的預設實作</span>

要使用預設實作來總結 `NewsArticle` 而不是定義自訂實作的話，我們可以指定一個空的 `impl` 區塊，像是 `impl Summary for NewsArticle {}`。

我們沒有直接對 `NewsArticle` 定義 `summarize` 方法，因為我們使用的是預設實作並聲明對 `NewsArticle` 實作 `Summary` 特徵。所以最後我們仍然能在 `NewsArticle` 實例中呼叫 `summarize`，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

此程式碼會印出 `有新文章發佈！(閱讀更多...)`。

建立 `summarize` 的預設實作不會影響範例 10-13 中 `Tweet` 實作的 `Summary`。因為要取代預設實作的語法，與當沒有預設實作時實作特徵方法的語法是一樣的。

預設實作也能呼叫同特徵中的其他方法，就算那些方法沒有預設實作。這樣一來，特徵就可以提供一堆實用的功能，並要求實作者只需處理一小部分就好。舉例來說，我們可以定義 `Summary` 特徵，使其擁有一個必須要實作的`summarize_author` 方法，以及另一個擁有預設實作會呼叫 `summarize_author` 的方法：

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

要使用這個版本的 `Summary`，我們只需要在對型別實作特徵時定義 `summarize_author` 就好：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

在我們定義 `summarize_author` 之後，我們可以在結構體 `Tweet` 的實例呼叫 `summarize`，然後 `summarize` 的預設實作會呼叫我們提供的 `summarize_author`。因為我們已經定義了`summarize_author`，且 `Summary` 特徵有提供 `summarize` 方法的預設實作，所以我們不必在寫任何程式碼。

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

此程式碼會印出 `1 則新推文：(從 @horse_ebooks 閱讀更多...)`。

注意要是對相同方法覆寫實作的話，就無法呼叫預設實作。

### 特徵作為參數

現在你知道如何定義與實作特徵，我們可以來探討如何使用特徵來定義函式來接受多種不同的型別。

舉例來說，在範例 10-13 我們對 `NewsArticle` 與 `Tweet` 實作了 `Summary` 特徵。我們可以定義一個函式 `notify` 使用它自己的參數 `item` 來呼叫 `summarize` 方法，所以此參數的型別預期有實作 `Summary` 特徵。
為此我們可以使用 `impl Trait` 語法，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

與其在 `item` 參數指定實際型別，我們用的是 `impl` 關鍵字並加上特徵名稱。這樣此參數就會接受任何有實作指定特徵的型別。在 `notify` 本體中我們就可以用 `item` 呼叫 `Summary` 特徵的任何方法，像是 `summarize`。我們可以呼叫 `notify` 並傳遞任何 `NewsArticle` 或 `Tweet` 的實例。但如果用其他型別像是 `String` 或 `i32` 來呼叫此程式碼的話會無法編譯，因為那些型別沒有實作 `Summary`。

#### 特徵界限語法

`impl Trait` 語法看起來很直觀，不過它其實是一個更長格式的語法糖，這個格式稱之為「特徵界限（trait bound）」，它長得會像這樣：

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("頭條新聞！{}", item.summarize());
}
```

此格式等同於之前段落的範例，只是比較長一點。我們將特徵界限置於泛型型別參數的宣告中，在尖括號內接在冒號之後。

`impl Trait` 語法比較方便，而且在簡單的案例中可以讓程式碼比較簡潔；特徵界限語法則適合用於其他比較複雜的案例。舉例來說我們可以有兩個有實作 `Summary` 的參數，使用 `impl Trait` 語法看起來會像這樣：

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

如果我們想要此函式允許 `item1` 和 `item2` 是不同型別的話，使用 `impl Trait` 的確是正確的（只要它們都有實作 `Summary`）。不過如果我們希望兩個參數都是同一型別的話，我們就得使用特徵界限來表達，如以下所示：

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

泛型型別 `T` 作為 `item1` 和 `item2` 的參數會限制函式，讓傳遞給 `item1` 和 `item2` 參數的數值型別必須相同。

#### 透過 `+` 來指定多個特徵界限

我們也可以指定不只一個特徵界限。假設我們還想要 `notify` 中的 `item` 不只能夠呼叫 `summarize` 方法，還能顯示格式化訊息的話，我們可以在 `notify` 定義中指定 `item` 必須同時要有 `Display` 和
`Summary`。這可以使用 `+` 語法來達成：

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

`+` 也能用在泛型型別的特徵界限中：

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

有了這兩個特徵界限，`notify` 本體就能呼叫 `summarize` 以及使用 `{}` 來格式化 `item`。

#### 透過 `where` 來使特徵界限更清楚

使用太多特徵界限也會帶來壞處。每個泛型都有自己的特徵界限，所以有數個泛型型別的函式可以在函式名稱與參數列表之間包含大量的特徵界限資訊，讓函式簽名難以閱讀。因此 Rust 有提供另一個在函式簽名之後指定特徵界限的語法 `where`。所以與其這樣寫：

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

我們可以這樣寫 `where` 的語法，如以下所示：

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

此函式簽名就沒有這麼複雜了，函式名稱、參數列表與回傳型別能靠得比較近，就像沒有一堆特徵界限的函式一樣。

### 返回有實作特徵的型別

我們也能在回傳的位置使用 `impl Trait` 語法來回傳某個有實作特徵的型別數值，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

將 `impl Summary` 作為回傳型別的同時，我們在函式 `returns_summarizable` 指定回傳有實作 `Summary` 特徵的型別而不必指出實際型別。在此例中，`returns_summarizable` 回傳 `Tweet`，但呼叫此函式的程式碼不會知道。

回傳一個只有指定所需實作特徵的型別在閉包（closures）與疊代器（iterators）中非常有用，我們會在第十三章介紹它們。閉包與疊代器能建立只有編譯器知道的型別，或是太長而難以指定的型別。`impl Trait` 語法允許你不用寫出很長的型別，而是只要指定函數會回傳有實作 `Iterator` 特徵的型別就好。

然而如果你使用 `impl Trait` 的話，你就只能回傳單一型別。舉例來說此程式碼指定回傳型別為 `impl Summary` ，但是寫說可能會回傳 `NewsArticle` 或 `Tweet` 的話就會無法執行：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

寫說可能返回 `NewsArticle` 或 `Tweet` 的話是不被允許的，因為 `impl Trait` 語法會限制在編譯器中最終決定的型別。我們會在第十七章的[「允許不同型別數值的特徵物件」][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore -->來討論如何寫出這種行為的函式。

### 透過特徵界限修正 `largest` 函式

現在你既然已經知道如何使用泛型型別參數來指定你想使用的行為，就讓我們回到範例 10-5 來使用泛型型別參數來修正 `largest` 函式的定義吧！上次我們試著執行此程式時，我們獲得這樣的錯誤：

```text
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

在 `largest` 我們想要用大於（`>`）運算子比較兩個型別的為 `T` 的數值。由於該運算子是從標準函式庫中的特徵 `std::cmp::PartialOrd` 的預設方法所定義的，我們希望在 `T` 中加上 `PartialOrd` 的特徵界限，讓函式可以比較任意型別的切片。我們不需要將 `PartialOrd` 引入作用域因為它由 prelude 提供。請變更 `largest` 的簽名如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/src/main.rs:here}}
```

這次編譯程式碼時，我們會得到不同的錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/output.txt}}
```

此錯誤的關鍵在 `cannot move out of type [T], a non-copy slice`。在我們非泛型版本的函式 `largest` 中，我們只有嘗試尋找 `i32` 或 `char` 的最大值。如同第四章[「只在堆疊上的資料：拷貝（Copy）」][stack-only-data-copy]<!-- ignore -->段落所提到的，像 `i32` 和 `char` 這樣的型別是已知大小可以存在堆疊上，所以它們有實作 `Copy` 特徵。但當我們建立泛型函式 `largest` 時，`list` 參數就有可能拿到沒有實作 `Copy` 特徵的型別。隨後導致我們無法將 `list[0]` 移出給變數 `largest`，最後產生錯誤。

要限制此程式碼只允許有實作 `Copy` 特徵的型別，我們可以再 `T` 的特徵界限中加上 `Copy`！範例 10-15 展示了泛型函式 `largest` 完整的程式碼，只要我們傳遞給函式的切片數值型別有實作 `PartialOrd` **和** `Copy` 特徵的話（像是 `i32` 和 `char`），就能編譯成功。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/main.rs}}
```

<span class="caption">範例 10-15：一個適用於任何實作 `PartialOrd` 與 `Copy` 特徵的泛型的 `largest` 函式</span>

如果我們不想要限制函式 `largest` 只接受實作 `Copy` 特徵的型別，我們可以在 `T` 中改指定 `Clone` 而非 `Copy`。這樣當我們想要 `largest` 取得所有權，我們就可以克隆切片的數值。使用 `clone` 函式代表我們對於像是 `String` 這樣擁有堆積資料的型別，可能會產生更多堆積分配。而如果我們處理的資料很龐大的話，堆積分配的速度可能就會很慢。

另一種實作 `largest` 的方法是我們可以來回傳切片中 `T` 數值的引用。如果我們將回傳型別改成 `&T` 而非 `T`，也就是改變函式本體來回傳引用的話，我們就不需要 `Clone` 或 `Copy` 特徵界限，也能避免堆積分配。請試著自己實作這個解決辦法看看吧！

### 透過特徵界限來選擇性實作方法

在有使用泛型型別參數 `impl` 區塊中使用特徵界限，我們可以選擇性地對有實作特定特徵的型別來時錯方法。舉例來說，範例 10-16 的 `Pair<T>` 只有在其內部型別 `T` 有實作能夠做比較的 `PartialOrd` 特徵以及能夠顯示在螢幕的 `Display` 特徵的話，才會實作 `cmp_display` 方法。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/lib.rs}}
```

<span class="caption">範例 10-16：依據特徵界限來選擇性地在泛型型別實作方法</span>

我們還可以對有實作其他特徵的型別選擇性地來實作特徵。對滿足特徵界限的型別實作特徵會稱之為**毯子實作（blanket implementations）**，這被廣泛地用在 Rust 標準函式庫中。舉例來說，標準函式庫會對任何有實作 `Display` 特徵的型別實作 `ToString`。標準函式庫中的 `impl` 區塊會有類似這樣的程式碼：

```rust,ignore
impl<T: Display> ToString for T {
    // --省略--
}
```

因為標準函式庫有此毯子實作，我們可以在任何有實作 `Display` 特徵的型別呼叫 `ToString` 特徵的 `to_string` 方法。舉例來說，我們可以像這樣將整數轉變成對應的 `String` 數值，因為整數有實作 `Display`：

```rust
let s = 3.to_string();
```

毯子實作在特徵技術文件的「Implementors」段落有做說明。

特徵與特徵界限讓我們能使用泛型型別參數來減少重複的程式碼的同時，告訴編譯器該泛型型別該擁有何種行為。編譯器可以利用特徵界限資訊來檢查程式碼提供的實際型別有沒有符合特定行為。在動態語言中，我們要是呼叫一個該型別沒有的方法的話，我們會在執行時才發生錯誤。但是 Rust 將此錯誤移到執行期間，讓我們必須在程式能夠執行之前確保有修正此問題。除此之外，我們還不用寫在執行時檢查此行為的程式碼，因為我們已經在編譯時就檢查了。這麼做我們可以在不失去泛型彈性的情況下，提升效能。

另一種我們已經看過的泛型為**生命週期（lifetimes）**。不同於確保一個型別有沒有我們要的行為，生命週期確保我們在需要引用的時候，它們都是有效的。讓我們來看看生命週期是怎麼做到的。

[stack-only-data-copy]:
ch04-01-what-is-ownership.html#只在堆疊上的資料拷貝copy
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch10-02-traits.md)
> - updated: 2020-09-15
