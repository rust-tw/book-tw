## 特徵：定義共同行為

**特徵**（trait）會定義特定型別與其他型別共享的功能。我們可以使用特徵定義來抽象出共同行為。我們可以使用**特徵界限**（trait bounds）來指定泛型型別為擁有特定行為的任意型別。

> 注意：特徵類似於其他語言常稱作**介面**（interfaces）的功能，但還是有些差異。

### 定義特徵

一個型別的行為包含我們對該型別可以呼叫的方法。如果我們可以對不同型別呼叫相同的方法，這些型別就能定義共同行為了。特徵定義是一個將方法簽名統整起來，來達成一些目的而定義一系列行為的方法。

舉例來說，如果我們有數個結構體各自擁有不同種類與不同數量的文字：結構體 `NewsArticle` 儲存特定地點的新聞故事，然後 `Tweet` 則有最多 280 字元的內容，且有個欄位來判斷是全新的推文、轉推或其他推文的回覆。

我們想要建立一個多媒體聚集器函式庫 crate 叫 `aggregator` 來顯示可能存在 `NewsArticle` 或 `Tweet` 實例的資料總結。要達成此目的的話，我們需要每個型別的總結，且我們會呼叫該實例的 `summarize` 方法來索取總結。範例 10-12 顯示了表達此行為的 `Summary` 特徵定義。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">範例 10-12：`Summary` 特徵包含 `summarize` 方法所定義的行為</span>

我們在此使用 `trait` 關鍵字定義一個特徵，其名稱為 `Summary`。我們也將特徵宣告成 `pub` 所以其他會依賴此函式庫的 crate 也能用到此特徵，我們之後會再看到其他範例。在大括號中，我們宣告方法簽名來描述有實作此特徵的型別行為，在此例就是 `fn summarize(&self) -> String`。

在方法簽名之後，我們並沒有加上大括號提供實作細節，而是使用分號。每個有實作此特徵的型別必須提供其自訂行為的方法本體。編譯器會強制要求任何有 `Summary` 特徵的型別都要有定義相同簽名的 `summarize` 方法。

特徵本體中可以有多個方法，每行會有一個方法簽名並都以分號做結尾。

### 為型別實作特徵

現在我們已經用 `Summary` 特徵定義了所需的方法簽名。我們可以在我們多媒體聚集器的型別中實作它。範例 10-13 顯示了 `NewsArticle` 結構體實作 `Summary` 特徵的方式，其使用頭條、作者、位置來建立 `summerize` 的回傳值。至於結構體 `Tweet`，我們使用使用者名稱加上整個推文的文字來定義 `summarize`，因為推文的內容長度已經被限制在 280 個字元以內了。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">範例 10-13：在型別 `NewsArticle` 與 `Tweet` 實作 `Summary` 特徵</span>

為一個型別實作一個特徵類似於實作一般的方法。不同的地方在於在 `impl` 之後我們加上的是想要實作的特徵，然後在用 `for` 關鍵字加上我們想要實作特徵的型別名稱。在 `impl` 的區塊內我們置入該特徵所定義的方法簽名，我們使用大括號並填入方法本體來為對特定型別實作出特徵方法的指定行為。

現在，我們就能像呼叫正常方法一樣，來呼叫 `NewsArticle` 和 `Tweet` 實例的方法，如以下所示：
現在函式庫已經對 `NewsArticle` 和 `Tweet` 實作 `Summary` 特徵了，crate 的使用者能像我們平常呼叫方法那樣，對 `NewsArticle` 和 `Tweet` 的實例呼叫特徵方法。唯一的不同是使用者必須將特徵也加入作用域中。以下的範例展示執行檔 crate 如何使用我們的 `aggregator` 函式庫 crate：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

此程式碼會印出「1 則新推文：horse_ebooks: of course, as you probably already know, people」。

其他依賴 `aggregator` 函式庫的 crate 也能將 `Summary` 特徵引入作用域並對他們自己的型別實作 `Summary` 特徵。不過實作特徵時有一個限制，那就是我們只能在該特徵或該型別位於我們的 crate 時，才能對型別實作特徵。舉例來說，我們可以對自訂型別像是 `Tweet` 來實作標準函式庫的 `Display` 特徵來為我們 crate `aggregator` 增加更多功能。因為 `Tweet` 位於我們的 `aggregator` crate 裡面。我們也可以在我們的 crate `aggregator` 內對 `Vec<T>` 實作 `Summary`。因為特徵 `Summary` 也位於我們的 `aggregator` crate 裡面。

但是我們無法對外部型別實作外部特徵。舉例來說我們無法在我們的 `aggregator` crate 裡面對 `Vec<T>` 實作 `Display` 特徵。因為 `Display` 與 `Vec<T>` 都定義在標準函式庫中，並沒有在我們 `aggregator` crate 裡面。此限制叫做「連貫性（coherence）」是程式屬性的一部分。更具體來說我們會稱作「孤兒原則（orphan rule）」，因為上一代（parent）型別不存在。此原則能確保其他人的程式碼不會破壞你的程式碼，反之亦然。沒有此原則的話，兩個 crate 可以都對相同型別實作相同特徵，然後 Rust 就會不知道該用哪個實作。

### 預設實作

有時候對特徵內的一些或所有方法定義預設行為是很實用的，而不必要求每個型別都實作所有方法。然後當我們對特定型別實作特徵時，我們可以保留或覆蓋每個方法的預設行為。

在範例 10-14 我們在 `Summary` 特徵內指定  `summarize` 方法的預設字串，而不必像範例 10-12 只定義了方法簽名。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">範例 10-14：`Summary` 特徵定義了 `summarize` 方法的預設實作</span>

要使用預設實作來總結 `NewsArticle` 的話，我們可以指定一個空的 `impl` 區塊，像是 `impl Summary for NewsArticle {}`。

我們沒有直接對 `NewsArticle` 定義 `summarize` 方法，因為我們使用的是預設實作並聲明對 `NewsArticle` 實作 `Summary` 特徵。所以最後我們仍然能在 `NewsArticle` 實例中呼叫 `summarize`，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

此程式碼會印出 `有新文章發佈！(閱讀更多...)`。

建立預設實作不會影響範例 10-13 中 `Tweet` 實作的 `Summary`。因為要取代預設實作的語法，與當沒有預設實作時實作特徵方法的語法是一樣的。

預設實作也能呼叫同特徵中的其他方法，就算那些方法沒有預設實作。這樣一來，特徵就可以提供一堆實用的功能，並要求實作者只需處理一小部分就好。舉例來說，我們可以定義 `Summary` 特徵，使其擁有一個必須要實作的`summarize_author` 方法，以及另一個擁有預設實作會呼叫 `summarize_author` 的方法：

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

要使用這個版本的 `Summary`，我們只需要在對型別實作特徵時定義 `summarize_author` 就好：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

在我們定義 `summarize_author` 之後，我們可以在結構體 `Tweet` 的實例呼叫 `summarize`，然後 `summarize` 的預設實作會呼叫我們提供的 `summarize_author`。因為我們已經定義了`summarize_author`，且 `Summary` 特徵有提供 `summarize` 方法的預設實作，所以我們不必再寫任何程式碼。

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

此程式碼會印出 `1 則新推文：(從 @horse_ebooks 閱讀更多...)`。

注意要是對相同方法覆寫實作的話，就無法呼叫預設實作。

### 特徵作為參數

現在你知道如何定義與實作特徵，我們可以來探討如何使用特徵來定義函式來接受多種不同的型別。我們會使用範例 10-13 中 `NewsArticle` 與 `Tweet` 實作的 `Summary` 特徵，來定義一個函式 `notify` 使用它自己的參數 `item` 來呼叫 `summarize` 方法，所以此參數的型別預期有實作 `Summary` 特徵。
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

`impl Trait` 語法比較方便，且在簡單的案例中可以讓程式碼比較簡潔；而特徵界限語法則適合用於其他比較複雜的案例。舉例來說我們可以有兩個有實作 `Summary` 的參數，使用 `impl Trait` 語法看起來會像這樣：

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
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

此函式簽名就沒有這麼複雜了，函式名稱、參數列表與回傳型別能靠得比較近，就像沒有一堆特徵界限的函式一樣。

### 回傳有實作特徵的型別

我們也能在回傳的位置使用 `impl Trait` 語法來回傳某個有實作特徵的型別數值，如以下所示：

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

將 `impl Summary` 作為回傳型別的同時，我們在函式 `returns_summarizable` 指定回傳有實作 `Summary` 特徵的型別而不必指出實際型別。在此例中，`returns_summarizable` 回傳 `Tweet`，但呼叫此函式的程式碼不需要知道。

回傳一個只有指定所需實作特徵的型別在閉包（closures）與疊代器（iterators）中非常有用，我們會在第十三章介紹它們。閉包與疊代器能建立只有編譯器知道的型別，或是太長而難以指定的型別。`impl Trait` 語法允許你不用寫出很長的型別，而是只要指定函數會回傳有實作 `Iterator` 特徵的型別就好。

然而如果你使用 `impl Trait` 的話，你就只能回傳單一型別。舉例來說此程式碼指定回傳型別為 `impl Summary` ，但是寫說可能會回傳 `NewsArticle` 或 `Tweet` 的話就會無法執行：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

寫說可能回傳 `NewsArticle` 或 `Tweet` 的話是不被允許的，因為 `impl Trait` 語法會限制在編譯器中最終決定的型別。我們會在第十七章的[「允許不同型別數值的特徵物件」][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore -->來討論如何寫出這種行為的函式。

### 透過特徵界限來選擇性實作方法

在有使用泛型型別參數 `impl` 區塊中使用特徵界限，我們可以選擇性地對有實作特定特徵的型別來實作方法。舉例來說，範例 10-15 的 `Pair<T>` 對所有 `T` 實作了 `new` 函式來回傳新的 `Pair<T>` 實例（回想一下第五章的[「定義方法」][methods]<!-- ignore -->段落，`Self` 是 `impl` 區塊內的型別別名，在此例就是 `Pair<T>`）。但在下一個 `impl` 區塊中，只有在其內部型別 `T` 有實作能夠做比較的 `PartialOrd` 特徵**以及**能夠顯示在螢幕的 `Display` 特徵的話，才會實作 `cmp_display` 方法。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

<span class="caption">範例 10-15：依據特徵界限來選擇性地在泛型型別實作方法</span>

我們還可以對有實作其他特徵的型別選擇性地來實作特徵。對滿足特徵界限的型別實作特徵會稱之為**全面實作（blanket implementations）**，這被廣泛地用在 Rust 標準函式庫中。舉例來說，標準函式庫會對任何有實作 `Display` 特徵的型別實作 `ToString`。標準函式庫中的 `impl` 區塊會有類似這樣的程式碼：

```rust,ignore
impl<T: Display> ToString for T {
    // --省略--
}
```

因為標準函式庫有此全面實作，我們可以在任何有實作 `Display` 特徵的型別呼叫 `ToString` 特徵的 `to_string` 方法。舉例來說，我們可以像這樣將整數轉變成對應的 `String` 數值，因為整數有實作 `Display`：

```rust
let s = 3.to_string();
```

全面實作在特徵技術文件的「Implementors」段落有做說明。

特徵與特徵界限讓我們能使用泛型型別參數來減少重複的程式碼的同時，告訴編譯器該泛型型別該擁有何種行為。編譯器可以利用特徵界限資訊來檢查程式碼提供的實際型別有沒有符合特定行為。在動態語言中，我們要是呼叫一個該型別沒有的方法的話，我們會在執行時才發生錯誤。但是 Rust 將此錯誤移到編譯期間，讓我們必須在程式能夠執行之前確保有修正此問題。除此之外，我們還不用寫在執行時檢查此行為的程式碼，因為我們已經在編譯時就檢查了。這麼做我們可以在不失去泛型彈性的情況下，提升效能。

[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html
[methods]: ch05-03-method-syntax.html#定義方法
