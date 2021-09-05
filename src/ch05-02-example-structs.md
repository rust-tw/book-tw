## 使用結構體的程式範例

為了瞭解我們何時會想要使用結構體，讓我們來寫一支計算長方形面積的程式。我們會先從單一變數開始，再慢慢重構成使用結構體。

讓我們用 Cargo 建立一個新的專案 *rectangles* ，它將接收長方形的長度與寬度，然後計算出長方形的面積。範例 5-8 展示了在我們專案底下 *src/main.rs* 用其中一種方式寫出來的小程式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">範例 5-8：使用變數 width 和 height 計算長方形面積</span>

現在使用 `cargo run` 執行程式：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

雖然範例 5-8 可以執行並呼叫 `area` 函式計算出長方形的面積，但我們可以做得更好。寬度與長度是互相關聯的，因為它們在一起剛好定義了一個長方形。

此程式碼的問題在 `area` 的函式簽名就能看出來：

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

`area` 函式應該要計算長方形的面積，但是我們寫的函式有兩個參數。參數之間是有關聯的，但是它在我們的程式中沒有表現出來。要是能將寬度與長度組合起來的話，會更容易閱讀與管理。我們可以使用我們在第三章提到的[「元組型別」][the-tuple-type]<!-- ignore -->。

### 使用元組重構

範例 5-9 展示了我們的程式用元組的另一種寫法。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">範例 5-9：使用元組指定長方形的寬度與長度</span>

一方面來說，此程式的確比較好。元組讓我們增加了一些結構，而我們現在只需要傳遞一個引數。但另一方面來說，此版本的閱讀性反而更差。元組無法命名它的元素，所以我們在計算時反而更難讀懂，我們傳得只是元組的索引。

我們在計算面積時，哪個值是寬度還是長度的確不重要。但如果我們要顯示出來的話，這就很重要了！我們會需要記住元組索引 `0` 是 `width` 然後元組索引 `1` 是 `height`。如果有其他人要維護這段程式碼的話，他就也得知道並記住這件事才行。但事實上是我們很常忘記這樣數值的意義並導致錯誤發生，因為我們無法從程式碼推導出資料的意義。

### 使用結構體重構：賦予更多意義

我們可以用結構體來為資料命名以賦予其意義。我們可以將元組轉換成一個有整體名稱且內部資料也都有名稱的資料型別，如範例 5-10 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">範例 5-10：定義 `Rectangle` 結構體</span>

我們在此定義了一個結構體叫做 `Rectangle`。在大括號內，我們定義了 `width` 與 `height` 的欄位，兩者型別皆為 `u32`。然後在 `main` 中，我們建立了一個寬度為 30 長度為 50 的 `Rectangle` 實例。

現在我們的 `area` 函式使需要一個參數 `rectangle`，其型別為 `Rectangle` 結構體實例的不可變借用。如同第四章提到的，我們希望借用結構體而非取走其所有權。這樣一來，`main` 能保留它的所有權並讓 `rect1` 繼續使用，這也是為何我們要在要呼叫函式的簽名中使用 `&`。

`area` 函式能夠存取 `Rectangle` 中的 `width` 與 `height` 欄位。我們的 `area` 函式簽名由可以表達出我們想要做的事情了：使用 `width` 與 `height` 欄位來計算 `Rectangle` 的面積。這能表達出寬度與長度之間的關係，並且給了它們容易讀懂的名稱，而不是像元組那樣用索引 `0` 和 `1`。這樣清楚多了。

### 使用推導特徵實現更多功能

現在要是能夠在我們除錯程式時能夠印出 `Rectangle` 的實例並看到它所有的欄位數值就更好了。範例 5-11 嘗試使用我們之前章節提到的 [`println!` 巨集][println]<!-- ignore -->，但是卻無法執行。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">範例 5-11：嘗試印出 `Rectangle` 實例</span>

當我們編譯此程式碼時，我們會得到以下錯誤訊息：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` 巨集預設可以做各式各樣的格式化，大括號告訴 `println!` 要使用 `Display` 特徵的格式化方式：其輸出結果是用來給最終使用者使用的。我們目前遇過的基本型別預設都會實作 `Display`，因為它們也只有一種顯示方式（像是 `1`）能夠給使用者。但是對結構體來說 `println!` 要怎麼格式化輸出結果就會有點不明確了，因為顯示的方式就很有多種。是要加上頓號嗎？是要印出大括號嗎？所有的欄位都要顯示出來嗎？基於這些不確定因素，Rust 不會去猜我們要的是什麼，所以結構體預設並沒有 `Display` 的實作。

如果我們繼續閱讀錯誤訊息，我們會得到一些有幫助的資訊：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

讓我們來試試看吧！`println!` 巨集的呼叫方式現在看起來應該會像這樣 `println!("rect1 is {:?}", rect1);`。在 `println!` 內加上 `:?` 這樣的標記指的是我們想要使用 `Debug` 特徵來作為輸出格式方式。`Debug` 特徵讓我們能印出對開發者有幫助的資訊，好讓我們在除錯程式時可以看到它的數值。

但是要是編譯這樣的程式的話，哎呀！我們卻還是會得到錯誤：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

不過同樣地，編譯器又給了我們有用的資訊：

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust **的確**有印出除錯資訊的功能，但是我們要針對我們的結構體顯式實作出來才會有對應的功能。為此我們可以在結構體前加上 `#[derive(Debug)]`，如範例 5-12 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">範例 5-12：加上推導（derive） `Debug` 特徵的標記並印出 `Rectangle` 實例的格式化資訊</span>

現在當我們執行程式，我們不會再得到錯誤了，而且我們可以看到格式化後的輸出結果：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

漂亮！雖然這不是非常好看的輸出格式，但是它的確顯示了實例中所有的欄位數值，這對我們除錯時會非常有用。不過如果我們的結構體非常龐大的話，我們會希望輸出格式可以比較好閱讀。為此我們可以在 `println!` 的字串使用 `{:#?}` 而非 `{:?}`。當我們使用 `{:#?}` 風格的話，輸出結果會長得像這樣：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

另一種使用 `Debug` 格式印出數值的方式是使用 [`dbg!` 巨集][dbg] <!-- ignore -->。`dbg!` 巨集會拿走一個表達式的所有權，印出該 `dbg!` 巨集在程式碼中呼叫的檔案與行數，以及該表達式的數值結果，最後回傳該數值的所有權。呼叫 `dbg!` 巨集會顯示到標準錯誤終端串流（`stderr`），而不像 `println!` 是印到標準輸出終端串流（`stdout`）。我們會在第十二章的[「將錯誤訊息寫入標準錯誤而非標準輸出」][err]<!-- ignore -->段落進一步討論 `stderr` 與 `stdout`。以下的範例我們想印出賦值給 `width` 的數值，以及整個 `rect1` 結構體的數值： 

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

我們在表達式 `30 * scale` 加上 `dbg!`，因爲 `dbg!` 會回傳表達式的數值所有權， `width` 將能取得和如果我們不加上 `dbg!` 時相同的數值。而我們不希望 `dbg!` 取走 `rect1` 的所有權，所以我們在下一個 `dbg!` 的呼叫使用引用。以下是此範例得到的輸出結果：

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

我們可以看見第一個輸出結果來自 *src/main.rs* 第十行，也就是我們除錯表達式 `30 * scale` 的地方，其結果數值爲 60 （整數實作的 `Debug` 格式只會印出它們的數值）。而在 *src/main.rs* 第十四行所呼叫的 `dbg!` 則輸出 `&rect1` 的數值，也就是 `Rectangle` 的結構體。此輸出就會使用 `Rectangle` 實作的 `Debug` 漂亮格式。當你需要嘗試理解程式碼怎麼運作時，`dbg!` 巨集可以變得相當實用！

除了 `Debug` 特徵之外，Rust 還提供了一些特徵能讓我們透過 `derive` 來使用並爲我們的自訂型別擴增實用的行爲。這些特徵與它們的行爲有列在[附錄 C][app-c]<!--ignore -->。我們會在第十章介紹如何實作這些特徵的自訂行爲，以及如何建立你自己的特徵。

我們的函式 `area` 最後就非常清楚明白了，它只會計算長方形的面積。這樣的行為要是能夠緊貼著我們的 `Rectangle` 結構體，因為這樣一來它就不會相容於其他型別。讓我們看看我們如何繼續重構我們的程式碼，接下來我們可以將函式 `area` 轉換為 `Rectangle` 型別的**方法（method）**。

[the-tuple-type]: ch03-02-data-types.html#元組型別
[app-c]: appendix-03-derivable-traits.md
[println]: https://doc.rust-lang.org/std/macro.println.html
[dbg]: https://doc.rust-lang.org/std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
