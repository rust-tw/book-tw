## 資料型別

每個數值在 Rust 中都屬於某種**資料型別**，這告訴 Rust 何種資料被指定，好讓它能妥善處理資料。我們將討論兩種資料型別子集：純量（scalar）與複合（compound）。

請記住 Rust 是一門**靜態型別**語言，這代表它必須在編譯時知道所有變數的型別。編譯器通常能依據數值與我們使用的方式推導出我們想使用的型別。但有時候如果多種型別都有可能時，像是第二章的[「將猜測的數字與祕密數字做比較」][comparing-the-guess-to-the-secret-number]<!-- ignore -->用到的 `parse` 將 `String` 轉換成數字時，我們就需要像這樣加上型別詮釋：

```rust
let guess: u32 = "42".parse().expect("這不是數字！");
```

如果我們沒有加上型別詮釋的話，Rust 將會顯示以下錯誤訊息。這表示編譯器需要我們給予更多資訊才能夠知道我們想用何種型別：

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

你將會看到其他資料型別的各種型別詮釋。

### 純量型別

**純量**型別代表單一數值。Rust 有四種主要純量型別：整數、浮點數、布林以及字元。你應該在其他程式語言就看過它們了，讓我們來看看它們在 Rust 是怎麼使用的：

#### 整數型別

**整數**是沒有小數點的數字。我們在第二章用到了一個整數型別 `u32`，此型別表示其擁有的數值應該是一個佔 32 位元大小的非帶號整數（帶號整數的話則是用 `i` 起頭而非 `u`）。表格 3-1 展示了 Rust 中內建的整數型別。我們可以使用以下任何一種型別來宣告一個整數數值。

<span class="caption">表格 3-1：Rust 中的整數型別</span>

| 長度     | 帶號    | 非帶號   |
|----------|---------|----------|
| 8 位元   | `i8`    | `u8`     |
| 16 位元  | `i16`   | `u16`    |
| 32 位元  | `i32`   | `u32`    |
| 64 位元  | `i64`   | `u64`    |
| 128 位元 | `i128`  | `u128`   |
| 系統架構 | `isize` | `usize`  |

每個變體都可以是帶號或非帶號的，並且都有明確的大小。**帶號**與**非帶號**的區別是數字能不能有負數，換句話說就是數字能否帶有正負符號，如果沒有的話那就只會出現正整數而已。就像在紙上寫數字一樣：當我們需要考慮符號時，我們就會在數字前面加上正負號；但如果我們只在意正整數的話，那它可以不帶符號。帶號數字是以[二補數](https://zh.wikipedia.org/zh-tw/%E4%BA%8C%E8%A3%9C%E6%95%B8)<!-- ignore -->的方式儲存。

每一帶號變體可以儲存的數字範圍包含從 -(2<sup>n - 1</sup>) 到 2<sup>n - 1</sup> - 1 以內的數字，*n* 就是該變體佔用的位元大小。所以一個 `i8` 可以儲存的數字範圍就是從 -(2<sup>7</sup>) 到 2<sup>7</sup> - 1，也就是 -128 到 127。而非帶號可以儲存的數字範圍則是從 0 到 2<sup>n</sup> - 1，所以 `u8` 可以儲存的範圍是從 0 到 2<sup>8</sup> - 1，也就是 0 到 255。

另外，`isize` 與 `usize` 型別則是依據你程式運行的電腦架構來決定大小，所以上方表格才用「系統架構」來表示長度：如果你在 64 位元架構上的話就是 64 位元；如果你是 32 位元架構的話就是 32 位元。

你可以用表格 3-2 列的格式來寫出整數字面值（literals）。能適用於數種數字型別的數字字面值都允許在最後面加上型別，比如說用 `57u8` 來指定型別。數字字面值也可以加上底線 `_` 分隔方便閱讀，比如說 `1_000` 其實就和指定 `1000` 的數值一樣。

<span class="caption">表格 3-2：Rust 中的整數字面值</span>

| 數字字面值         | 範例          |
|--------------------|---------------|
| 十進制             | `98_222`      |
| 十六進制           | `0xff`        |
| 八進制             | `0o77`        |
| 二進制             | `0b1111_0000` |
| 位元組（僅限`u8`） | `b'A'`        |

所以你該用哪些整數型別呢？如果你不確定的話，Rust 預設的型別是很好的起始點：整數型別預設是 `i32`。而你會用到 `isize` 或 `usize` 的主要時機是作為某些集合的索引。

> ##### 整數溢位
>
> 假設你有個變數型別是 `u8` 可以儲存 0 到 255 的數值。如果你想要改變變數的值超出這個範圍的話，比方說像是 256，那麼就會發生**整數溢位**，這會產生兩種不同的結果。如果你是在除錯模式編譯的話，Rust 會包含整數溢位的檢查，造成你的程式在執行時**恐慌（panic）**。Rust 使用恐慌來表示程式因錯誤而結束，我們會在第九章的[「對無法復原的錯誤使用 `panic!`」][unrecoverable-errors-with-panic]<!-- ignore -->段落討論更多造成恐慌的細節。
>
> 當你是在發佈模式下用 `--release` 來編譯的話，Rust 則**不會**加上整數溢位的檢查而造成恐慌。相反地，如果發生整數溢位的話，Rust 會作出**二補數包裝**的動作。簡單來說，超出最大值的數值可以被**包裝**成該型別的最低數值。以 `u8` 為例的話，256 會變成 0、257 會變成 1，以此類推。程式不會恐慌，但是該變數可能會得到一個不是你原本預期的數值。通常依靠整數溢位的行為仍然會被視為邏輯錯誤。
>
> 要顯式處理可能的溢位的話，你可以使用以下標準函式庫中基本型別提供的一系列方法：
>
> - 將所有操作用 `wrapping_*` 方法包裝，像是 `wrapping_add`
> - 使用 `checked_*` 方法，如果有溢位的話其會回傳 `None` 數值
> - 使用 `overflowing_*` 方法，其會回傳數值與一個布林值來顯示是否有溢位發生
> - 屬於 `saturating_*` ，讓數值溢位時保持在最小或最大值

#### 浮點數型別

Rust 還有針對有小數點的**浮點數**提供兩種基本型別：`f32` 和 `f64`，分別佔有 32 位元與 64 位元的大小。而預設的型別為 `f64`，因為現代的電腦處理的速度幾乎和 `f32` 一樣卻還能擁有更高的精準度。所有的浮點數型別都是帶號的（signed）。

以下為展示浮點數的範例：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

浮點數是依照 IEEE-754 所定義的，`f32` 型別是單精度浮點數，而 `f64` 是倍精度浮點數。

#### 數值運算

Rust 支援你所有想得到的數值型別基本運算：加法、減法、乘法、除法和取餘。整數除法會取最接進的下界數值。以下程式碼展示出如何在 `let` 陳述式使用這些運算：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

每一個陳述式中的表達式都使用了一個數學運算符號並計算出一個數值出來，賦值給該變數。[附錄 B][appendix_b]<!-- ignore --> 有提供列表列出 Rust 所提供的所有運算子。

#### 布林型別

如同其他多數程式語言一樣，Rust 中的布林型別有兩個可能的值：`true` 和 `false`。布林值的大小為一個位元組。要在 Rust 中定義布林型別的話用 `bool`，如範例所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

布林值最常使用的方式之一是作為條件判斷，像是在 `if` 表達式中使用。我們將會在[「控制流程」][control-flow]<!-- ignore -->段落介紹如何在 Rust 使用 `if` 表達式。

#### 字元型別

Rust 的 `char` 型別是最基本的字母型別，以下程式碼顯示了使用它的方法：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

注意到 `char` 字面值是用單引號賦值，宣告字串字面值時才是用雙引號。Rust 的 `char` 型別大小為四個位元組並表示為一個 Unicode 純量數值，這代表它能擁有的字元比 ASCII 還來的多。舉凡標音字母（Accented letters）、中文、日文、韓文、表情符號以及零長度空格都是 Rust `char` 的有效字元。Unicode 純量數值的範圍包含從 `U+0000` 到 `U+D7FF` 以及 `U+E000` 到 `U+10FFFF`。但是一個「字元」並不是真正的 Unicode 概念，所以你對於什麼是一個「字元」的看法可能會和 Rust 的 `char` 不一樣。我們將會在第八章的[「透過字串儲存 UTF-8 編碼的文字」][strings]<!-- ignore -->來討論此議題。

### 複合型別

**複合型別**可以組合數個數值為一個型別，Rust 有兩個基本複合型別：元組（tuples）和陣列（arrays）。

#### 元組型別

元組是個將許多不同型別的數值合成一個複合型別的常見方法。元組擁有固定長度：一旦宣告好後，它們就無法增長或縮減。

我們建立一個元組的方法是寫一個用括號囊括起來的數值列表，每個值再用逗號分隔開來。元組的每一格都是一個獨立型別，不同數值不必是相同型別。以下範例我們也加上了型別詮釋，平時不一定要加上：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

此變數 `tup` 就是整個元組，因為一個元組就被視為單一複合元素。要拿到元組中的每個獨立數值的話，我們可以用模式配對（pattern matching）來解構一個元組的數值，如以下所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

此程式先是建立了一個元組然後賦值給 `tup`，接著它用模式配對和 `let` 將 `tup` 拆成三個個別的變數 `x`、`y` 和 `z`。這就叫做**解構（destructuring）**，因為它將單一元組拆成了三個部分。最後程式將 `y` 的值印出來，也就是 `6.4`。

我們也可以直接用句號（`.`）再加上數值的索引來取得元組內的元素。舉例來說：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

此程式建立了元組 `x`，然後用它們個別的索引產生新的變數。和多數程式語言一樣，元組的第一個索引是 0。

沒有任何數值的元組 `()` 會是個只有一種數值的特殊型別，其值也寫作 `()`。此型別稱爲 **單元型別** 而其數值稱爲 **單元數值**。表達式要是沒有回傳任何數值的話，它們就會隱式回傳單元型別。

#### 陣列型別

另一種取得數個數值集合的方法是使用**陣列**。和元組不一樣的是，陣列中的每個型別必須是一樣的。和其他語言的陣列不同，Rust 的陣列是固定長度的。

我們將數值寫在陣列中的括號內，每個數值再用逗號區隔開來：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

當你想要你的資料被分配在堆疊（stack）而不是堆積（heap）的話，使用陣列是很好的選擇（我們會在[第四章][stack-and-heap]<!-- ignore -->討論堆疊與堆積的內容）。或者當你想確定你永遠會取得固定長度的元素時也是。所以陣列不像向量（vector）型別那麼有彈性，向量是標準函式庫提供的集合型別，類似於陣列但**允許**變更長度大小。如果你不確定該用陣列或向量的話，通常你應該用向量就好。[第八章][vectors]<!-- ignore -->將會討論更多向量的細節。

不過如果你知道元素的多寡不會變的話，陣列就是個不錯的選擇。舉例來說，如果你想在程式中使用月份的話，你可能就會選擇用陣列宣告，因為永遠只會有 12 個月份：

```rust
let months = ["一月", "二月", "三月", "四月", "五月", "六月", "七月",
              "八月", "九月", "十月", "十一月", "十二月"];
```

要詮釋陣列型別的話，你可以在中括號寫出型別和元素個數，並用分號區隔開來，如以下所示：

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```


`i32` 在此是每個元素的型別，在分號後面的數字 `5` 指的是此陣列有五個元素。

如果你想建立的陣列中每個元素數值都一樣的話，你可以指定一個數值後加上分號，最後寫出元素個數。如以下所示：

```rust
let a = [3; 5];
```

陣列 `a` 會包含 `5` 個元素，然後每個元素的初始化數值均為 `3`。這樣寫與 `let a = [3, 3, 3, 3, 3];` 的寫法一樣，但比較簡潔。

##### 獲取陣列元素

一個陣列是被分配在堆疊上且已知固定大小的一整塊記憶體，你可以用索引來取得陣列的元素，比如：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

在此範例中，變數 `first` 會得到數值 `1`，因為這是陣列索引 `[0]` 的數值。變數 `second` 則會從陣列索引 `[1]` 得到數值 `2`。

##### 無效的陣列元素存取

讓我們看看如果我們存取陣列之後的元素會發生什麼事呢？假設你修改成以下範例，這是改寫自第二章猜謎遊戲要從使用者取得陣列索引的程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

此程式碼能編譯成功。如果你透過 `cargo run` 執行此程式碼並輸入 0、1、2、3 或 4 的話，程式將會印出陣列索引對應的數值。但如果你輸入超出陣列長度的數值，你會看到像是這樣的輸出結果：

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

此程式會在使用無效數值進行索引操作時產生**執行時**（runtime）錯誤。程式會退出並回傳錯誤訊息，且不會執行最後的 `println!`。當你嘗試使用索引存取元素時，Rust 會檢查你的索引是否小於陣列長度，如果索引大於或等於陣列長度的話，Rust 就會恐慌。這樣的檢查必須發生在執行時，尤其是在此例，因爲編譯器無法知道之後的使用者將會輸入哪些數值。

這是 Rust 記憶體安全原則給予的保障。在許多低階語言並不會提供這樣的檢查，所以當你提供不正確的索引時，無效的記憶體可能會被存取。Rust 會保護你免於這樣的錯誤風險，並立即離開程式，而不是允許記憶體存取並繼續。第九章將會討論更多有關 Rust 的錯誤處理方式。

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#將猜測的數字與祕密數字做比較
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: https://doc.rust-lang.org/std/num/struct.Wrapping.html
[appendix_b]: appendix-02-operators.md
