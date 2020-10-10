## 透過字串儲存 UTF-8 編碼的文字

我們已經在第四章提到字串（String），但現在我們要更加深入探討。Rustaceans 初心者常常會卡在三個環節：Rust 傾向於回報可能的錯誤、字串的資料結構比開發者所熟悉的還要複雜，以及 UTF-8。這些要素讓來自其他程式語言背景的開發者會遇到一些困難。

我們會在集合章節討論字串的原因是，字串本身就是位元組的集合，且位元組作為文字呈現時，它會提供一些實用的方法。在此段落我們將和其他集合型別一樣討論 `String` 的操作，像是建立、更新與讀取。我們還會討論到 `String` 與其他集合不一樣的地方，像是 `String` 的索引就比其他集合還複雜，因為它會依據人們對於 `String` 資料型別的理解而有所不同。

### 什麼是字串？

首先我們要好好定義*字串（String）*這個術語。Rust 在核心語言中只有一個字串型別，那就是字串切片 `str`，它通常是以借用的形式存在 `&str`。在第四章中我們提到*字串切片*是一個針對存在某處的  UTF-8 編碼資料的引用。舉例來說，字串字面值（String literals）就儲存在程式的二進制檔案中，因此就是字串切片。

`String` 型別是 Rust 標準函式庫所提供的型別，並不是核心語言內建的型別，它是可增長的、可變的、可擁有所有權的 UTF-8 編碼字串型別。當 Rustaceans 提及 Rust 中的「字串」時，他們通常指的是 `String` 以及字串切片 `&str` 型別，而不只是其中一種型別。雖然此段落大部分都在討論 `String`，這兩個型別都時常用在 Rust 的標準函式庫中，且 `String` 與字串切片都是 UTF-8 編碼的。

Rust 的標準函式庫還包含了其他種類的字串型別，像是 `OsString`、`OsStr`、`CString` 以及 `CStr`。函式庫 crates 更可以提供儲存字串資料的更多選項。你應該會注意到這些型別的結尾都是 `String` 和 `Str`，它們分別代表擁有所有權與借用的變體。就像你之前看到的 `String` 和 `str` 型別一樣。這些字串型別可以儲存不同編碼的文字或者以不同的記憶體形式呈現。我們不會在本章節討論這些字串型別，要是你想知道如何或何時使用它們的話，你可以查閱它們的 API 技術文件。

### 建立新的字串

許多 `Vec<T>` 可使用的方法在 `String` 也都能用，像是用 `new` 函式建立新的字串，如範例 8-11 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">範例 8-11：建立新的空 `String`</span>

此行會建立新的字串叫做 `s`，我們之後可以再寫入資料。不過通常我們會希望建立字串的同時能夠初始化資料。為此我們可以使用 `to_string` 方法，任何有實作 `Display` 特徵的型別都可以使用此方法，就像字串字面值的使用方式一樣。範例 8-12 就展示了兩種例子。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<span class="caption">範例 8-12：從字串字面值使用 `to_string` 方法來建立 `String`</span>

此程式碼建立了一個字串內容為 `初始內容`。

我們也可以用函式 `String::from` 從字串字面值建立 `String`。範例 8-13 的程式碼和使用 `to_string` 的範例 8-12 效果一樣。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<span class="caption">範例 8-13：使用函式 `String::from` 從字串字面值建立 `String`</span>

因為字串用在許多地方，我們可以使用許多不同的通用字串 API 供我們選擇。有些看起來似乎是多餘的，但是它們都有一席之地的！在上面的範例中 `String::from` 和 `to_string` 都在做相同的事，所以你的選擇在於喜好風格上。

另外記得字串是 UTF-8 編碼的，所以我們可以包含任何正確編碼的資料，如範例 8-14 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<span class="caption">範例 8-14：用字串儲存各種語言打招呼的文字</span>

以上全是合理的 `String` 數值。

### 更新字串

就和 `Vec<T>` 一樣，如果你插入更多資料的話，`String` 可以增長大小並變更其內容。除此之外你也可以使用 `+` 運算子或 `format!` 巨集來串接 `String` 數值。

#### 使用 `push_str` 和 `push` 追加字串

我們可以使用 `push_str` 方法來追加一個字串切片使字串增長，如範例 8-15 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<span class="caption">範例 8-15：使用 `push_str` 方法向 `String` 追加字串切片</span>

在這兩行之後，`s` 會包含 `foobar`。`push_str` 方法取得的是字串切片因為我們並不需要取得參數的所有權。舉例來說範例 8-16 就說明了如果 `s2` 在追加其內容給 `s1` 之後卻不能使用的話，就很不妙了。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<span class="caption">範例 8-16：在內容追加給 `String` 後繼續使用字串切片</span>

如果 `push_str` 方法會取得 `s2` 的所有權，我們就無法在最後一行印出其數值了。幸好這段程式碼是可以執行的！

而 `push` 方法會取得一個字元作為參數並加到 `String` 上。範例 8-17 顯示了一個使用 `push` 方法將字母 *l* 加到 `String` 的程式碼。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">範例 8-17：使用 `push` 將一個字元加到 `String`</span>

此程式碼的結果就是 `s` 會包含 `lol`。

#### 使用 `+` 運算子或 `format!` 巨集串接字串

你通常會想要組合兩個字串在一起，其中一種方式是用 `+` 運算子。如範例 8-18 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">範例 8-18：使用 `+` 運算子組合兩個 `String` 數值成一個新的 `String` 數值</span>

程式碼最後的字串 `s3` 就會獲得 `Hello, world!`。`s1` 之所以在相加後不再有效，以及 `s2` 是使用引用的原因，都和我們使用 `+` 運算子時呼叫的方法簽名有關。`+` 運算子使用的是 `add` 方法，其簽名會長得像這樣：

```rust,ignore
fn add(self, s: &str) -> String {
```

這不全是標準函式庫中實際的簽名，在標準函式庫中 `add` 是用泛型（generics）定義。我們在此看到的是使用實際型別指明泛型的 `add` 簽名。我們會在第十章討論到泛型。此簽名給了一些我們需要瞭解 `+` 運算子的一些線索。

首先 `s2` 有 `&` 代表我們是將第二個字串的*引用*與第一個字串相加，因為函式 `add` 中的參數 `s` 說明我們只能將 `&str` 與 `String` 相加，我們無法將兩個 `String` 數值相加。但等等 `&s2` 是 `&String` 才對，並非 `add` 第二個參數所指定的 `&str`。為何範例 8-18 可以編譯呢？

我們可以在 `add` 的呼叫中使用 `&s2` 的原因是因為編譯器可以*強制（coerce）* `&String` 引數轉換成 `&str`。當我們我們呼叫 `add` 方法時，Rust *強制解引用（deref coercion）* 讓 `&s2` 變成 `&s2[..]`。我們會在第十五章深入探討強制解引用。因為 `add` 不會取得 `s` 參數的所有權，`s2` 在此運算後仍然是個有效的 `String`。

再來，我們可以看到 `add` 的簽名會取得 `self` 的所有權，因為 `self` *沒有* `&`。這代表範例 8-18 的 `s1` 會移動到 `add` 的呼叫內，在之後就不再有效。所以雖然 `let s3 = s1 + &s2;` 看起來像是它拷貝了兩個字串的值並產生了一個新的，但此陳述式實際上是取得 `s1` 的所有權、追加一份 `s2` 的複製內容、然後回傳最終結果的所有權。換句話說，雖然它看起來像是產生了很多拷貝，但實際上並不是。此實作反而比較有效率。

如果我們需要串接數個字串的話，`+` 運算子的行為看起來就顯得有點笨重了：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

此時 `s` 會是 `tic-tac-toe`。有這麼多的 `+` 和 `"` 字元，我們很難看清楚發生什麼事。如果要完成更複雜的字串組合的話，我們可以使用 `format!` 巨集：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

此程式碼一樣能設置 `s` 為to `tic-tac-toe`。`format!` 巨集運作的方式和 `println!` 一樣，但不會將輸出結果顯示在螢幕上，它做的是回傳內容的 `String`。使用 `format!` 的程式碼版本看起來比較好讀懂，而且不會取走任何參數的所有權。

### 索引字串

在其他許多程式語言中，使用索引引用字串來取得獨立字元是有效且常見的操作。然而在 Rust 中如果你嘗試對 `String` 使用索引語法的話，你會得到錯誤。請看看範例 8-19 這段無效的程式碼。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">範例 8-19：嘗試在字串使用索引語法</span>

此程式會有以下錯誤結果：

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

錯誤訊息與提示告訴了我們 Rust 字串並不支援索引。但為何不支援呢？要回答此問題，我們需要先討論 Rust 如何儲存字串進記憶體的。

#### 內部呈現

`String` 基本上就是 `Vec<u8>` 的封裝。讓我們看看範例 8-14 中一些正確編碼為 UTF-8 字串的例子，像是這一個：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

在此例中 `len` 會是 4，也就是向量儲存的字串「Hola」長度為 4 個位元組。每個字母在用 UFT-8 編碼時長度均為 1 個位元組。那接下來這段呢？（請注意字串的開頭是西里爾字母 Ze 的大寫，而不是阿拉伯數字 3）

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

你可能會以為這字串的長度為 12，然而 Rust 給的答案卻是 24。這是將「Здравствуйте」用 UTF-8 編碼後的位元組長度，因為該字串的每個 Unicode 純量都佔據兩個位元組。因此字串位元組的索引不會永遠都能對應到有效的 Unicode 純量數值。我們用以下無效的 Rust 程式碼進一步說明：

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

`answer` 的數值會是多少呢？會是第一個字母 `З` 嗎？當經過 UTF-8 編碼時，`З` 的第一個位元組會是 `208` 然後第二個是 `151`。所以 `answer` 實際上會拿到 `208`，但 `208` 本身又不是個有效字元。回傳 `208` 可能不會是使用者想要的，他們希望的應該是此字串的第一個字母，但這是 Rust 在位元組索引 0 唯一能回傳的資料。就算字串都只包含拉丁字母，使用者通常也不會希望看到位元組數值作為回傳值。如果 `&"hello"[0]` 是有效程式碼且會回傳位元組數值的話，它會回傳的是 `104` 並非 `h`。為了預防回傳意外數值進而導致無法立刻察覺的錯誤，Rust 不會成功編譯這段程式碼，並在開發過程前期就杜絕誤會發生。

#### 位元組、純量數值與形素群集！我的天啊！

UTF-8 還有一個重點是在 Rust 中我們實際上可以有三種觀點來理解字串：位元組、純量數值（scalar values）以及形素群集（grapheme clusters，最接近人們常說的「*字母*」）。

如果我們觀察用天成體寫的印度語「नमस्ते」，它存在向量中的 `u8` 數值就會長這樣：

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

這 18 個位元組是電腦最終儲存的資料？如果我們用 Unicode 純量數值觀察的話，也就是 Rust 的 `char` 型別，這些位元組會組成像這樣：

```text
['न', 'म', 'स', '्', 'त', 'े']
```

這邊有六個 `char` 數值，但第四個和第六個卻不是字母，它們是單獨存在不具任何意義的變音符號。最後如果我們以形素群集的角度來看的話，我們就會得到一般人所說的構成此印度語的四個字母：
```text
["न", "म", "स्", "ते"]
```

Rust 提拱多種不同的方式來解釋電腦中儲存的原始字串資料，讓每個程式無論是何種人類語言的資料，都可以選擇它們需要的呈現方式。

Rust 還有一個不允許索引 `String` 來取得字元的原因是因為，索引運算必須永遠預期是花費常數時間（O(1)）。但在 `String` 上無法提供這樣的效能保證，因為 Rust 會需要從索引的開頭遍歷每個內容才能決定多少有效字元存在。

### 字串切片

索引字串通常不是個好點子，因為字符串索引要回傳的型別是不明確的，是要一個位元組數值、一個字元、一個形素群集還是一個字串切片呢。因此如果你真的想要使用索引建立字串切片的話，Rust 會要你更明確些。要明確指定你的索引與你想要的字串切片，與其在 `[]` 只使用一個數字來索引，你可以在 `[]` 指定一個範圍來建立包含特定位元組的字串切片：

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

`s` 在此會是 `&str` 並包含字串前 4 個位元組。稍早我們提過這些字元各佔 2 個位元組，所以這裡的 `s` 就是 `Зд`。

那如果我們只用 `&hello[0..1]` 呢？答案是 Rust 會和在向量中取得無效索引一樣在執行時恐慌：

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

你在使用範圍來建立字串切片時要格外小心，因為這樣做有可能會使你的程式崩潰。

### 遍歷字串的方法

幸運的是你有其他方法來取得字串的元素。

如果你需要對每個獨立的 Unicode 純量型別做運算的話，最好的方式是使用 `chars` 方法。對「नमस्ते」呼叫 `chars` 會將六個擁有 `char` 型別的數值拆開並回傳，這樣一來你就可以遍歷每個元素：

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

此程式碼會顯示以下輸出：

```text
न
म
स
्
त
े
```

而 `bytes` 方法會回傳每個原始位元組，可能會在某些場合適合你：

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

此程式碼會印出此 `String` 的 18 個位元組：

```text
224
164
// --省略--
165
135
```

請確定你已經瞭解有效的 Unicode 純量數值可能不止佔 1 個位元組。

而要從字串取得形素群集的話就非常複雜了，所以標準函式庫並未提供這項功能。如果你需要的話，[crates.io](https://crates.io/) 上會有提供這項功能的 crate。

### 字串並不簡單

總結來說，字串是很複雜的。不同的程式語言會選擇不同的決定來呈現給程式設計師。Rust 選擇正確處理 `String` 的方式作為所有 Rust 程式的預設行為，這也代表開發者在處理 UTF-8 資料時需要多加考量。這樣的取捨的確對比其他程式語言來說，增加了不少字串的複雜程度，但是這能讓你在開發週期免於處理非 ASCII 字元相關的錯誤。

讓我們接下去開一個較簡單地集合吧：雜湊映射（hash maps）！

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch08-02-strings.md)
> - updated: 2020-09-11
