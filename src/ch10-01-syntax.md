## 泛型資料型別

我們使用泛型（generics）來建立項目的定義，像是函式簽名或結構體，讓我們在之後可以使用在不同的實際資料型別。讓我們先看看如何使用泛型定義函式、枚舉與方法。然後我們會在來看泛型對程式碼的效能影響如何。

### 在函式中定義

當要使用泛型定義函數時，我們通常會將泛型置於函式簽名中指定參數與回傳值資料型別的位置。這樣做能讓我們的程式碼更具彈性並向呼叫者提供更多功能，同時還能防止重複程式碼。

接續我們 `largest` 函式的例子，範例 10-4 展示了兩個都在切片上尋找最大值的函式。我們要使用泛型將它們融合成一個函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">範例 10-4：兩個名稱與其簽名中的型別都不同的函式</span>

`largest_i32` 函式和我們在範例 10-3 提取的函式一樣都是尋找切片中最大的 `i32`。而 `largest_char` 函式則尋找切片中最大的 `char`。函式本體都擁有相同的程式碼，讓我們來開始用泛型型別參數來消除重複的部分，轉變成只有一個函式吧。

要在新定義的函式中參數化型別的話，我們需要為參數型別命名，就和我們在函式中的參數數值所做的一樣。你可以用任何標識符來命名型別參數名稱。但我們習慣上會用 `T`，因為 Rust 的型別參數名稱都盡量很短，常常只會有一個字母，而且 Rust 對於型別命名的慣用規則是駝峰式大小寫（CamelCase）。所以 `T` 作為「type」的簡稱是大多數 Rust 程式設計師的選擇。

當我們在函式本體使用參數時，我們必須在簽名中宣告參數名稱，編譯器才能知道該名稱代表什麼。同樣地，當我們要在函式簽名中使用型別參數名稱，我們必須在使用前宣告該型別參數名稱。要定義泛型 `largest` 函式的話，我們在函式名稱與參數列表之間加上尖括號，其內就是型別名稱的宣告，如以下所示：

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

我們可以這樣理解定義：函式 `largest` 有泛型型別 `T`，此函式有一個參數叫做 `list`，它的型別為數值 `T` 的切片。`largest` 函式會回傳與型別 `T` 相同型別的參考數值。

範例 10-5 顯示了使用泛型資料型別於函式簽名組合出的 `largest` 函式。此範例還展示了我們如何依序用 `i32` 和 `char` 的切片呼叫函式。注意此程式碼尚未能編譯，不過我們會在本章之後修改它。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">範例 10-5：使用泛型型別參數的 `largest` 函式，但現在還不能編譯</span>

如果我們現在就編譯程式碼的話，我們會得到此錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

提示文字中提到了 `std::cmp::PartialOrd` 這個**特徵（trait）**。我們會在下個段落來討論特徵。現在只需要知道 `largest` 本體無法適用於所有可能的 `T` 型別，因為我們想要在本體中比較型別 `T` 的數值，我們只能在能夠排序的型別中做比較。要能夠比較的話，標準函式庫有提供 `std::cmp::PartialOrd` 特徵讓你可以針對你的型別來實作（請查閱附錄 C 來瞭解更多此特徵的細節）。照著提示文字的建議，我們限制 `T` 只對有實作 `PartialOrd` 的型別有效。這樣此範例就能編譯，因為標準函式庫有對 `i32` 與 `char` 實作 `PartialOrd`。

### 在結構體中定義

我們一樣能以 `<>` 語法來對結構體中一或多個欄位使用泛型型別參數。範例 10-6 展示了定義 `Point<T>` 結構體並讓 `x` 與 `y` 可以是任意型別數值。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">範例 10-6：`Point<T>` 結構體的 `x` 與 `y` 會有型別 `T` 的數值</span>

在結構體定義使用泛型的語法與函式定義類似。首先，我們在結構體名稱後方加上尖括號，並在其內宣告型別參數名稱。接著我們能在原本指定實際資料型別的地方，使用泛型型別來定義結構體。

注意到我們使用了一個泛型型別來定義 `Point<T>`，此定義代表 `Point<T>` 是某型別 `T` 下之通用的，而且欄位 `x` 與 `y` 擁有**相同**型別，無論最終是何種型別。如果我們用不同的型別數值來建立 `Point<T>` 實例，我們的程式碼會無法編譯，如範例 10-7 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">範例 10-7：欄位 `x` 與 `y` 必須是相同型別，因為它們擁有相同的泛型資料型別 `T`</span>

在此例中，當我們賦值 5 給 `x` 時，我們讓編譯器知道 `Point<T>` 實例中的泛型型別 `T` 會是整數。然後我們將 4.0 賦值給 `y`，這應該要和 `x` 有相同型別，所以我們會獲得以下錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

要將結構體 `Point` 的 `x` 與 `y` 定義成擁有不同型別卻仍然是泛型的話，我們可以使用多個泛型型別參數。舉例來說，在範例 10-8 我們改變了 `Point` 的定義為擁有兩個泛型型別 `T` 與 `U`，`x` 擁有型別 `T` 而 `y` 擁有型別 `U`。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">範例 10-8：`Point<T, U>` 擁有兩個泛型型別，所以 `x` 和 `y` 可以有不同的型別數值</span>

現在這些所有的 `Point` 實例都是允許的了！你要在定義中使用多少泛型型別參數都沒問題，但用太多的話會讓你的程式碼難以閱讀。如果你發現你的程式碼需要使用大量泛型的話，這通常代表你的程式碼需要重新組織成更小的元件。

### 在枚舉中定義

如同結構體一樣，我們可以定義枚舉讓它們的變體擁有泛型資料型別。讓我們看看我們在第六章標準函式庫提供的 `Option<T>` 枚舉：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

此定義現在對你來說應該就說得通了。如同你所看到的 `Option<T>` 枚舉有個泛型型別參數 `T` 以及兩個變體：`Some` 擁有型別 `T` 的數值；而 `None` 則是不具任何數值的變體。使用 `Option<T>` 枚舉我們可以表達出一個可能擁有的數值這樣的抽象概念。而且因為 `Option<T>` 是泛型，不管可能的數值型別為何，我們都能使用此抽象。

枚舉也能有數個泛型型別。我們在第九章所使用枚舉 `Result` 的定義就是個例子：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 枚舉有兩個泛型型別 `T` 和 `E` 且有兩個變體：`Ok` 擁有型別 `T` 的數值；而 `Err` 擁有型別 `E` 的數值。這樣的定義讓我們很方便能表達 `Result` 枚舉可能擁有一個成功的數值（回傳型別 `T` 的數值）或失敗的數值（回傳型別為 `E` 的錯誤值）。事實上這就是我們在範例 9-3 開啟檔案的方式，當我們成功開啟檔案時的 `T` 就會是型別 `std::fs::File`，然後當開啟檔案會發生問題時 `E` 就會是型別 `std::io::Error`。

當你發現你的程式碼有許多結構體或枚舉都只有儲存的值有所不同時，你可以使用泛型型別來避免重複。

### 在方法中定義

我們可以對結構體或枚舉定義方法（如第五章所述）並也可以使用泛型型別來定義。範例 10-9 展示了我們在範例 10-6 定義的結構體 `Point<T>` 並實作了一個叫做 `x` 的方法。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">範例 10-9：在 `Point<T>` 結構體實作一個方法叫做 `x`，其會回傳 `x` 欄位中型別為 `T` 的參考</span>

我們在這 `Point<T>` 定義了一個方法叫做 `x` 並回傳欄位 `x` 的資料參考。

注意到我們需要在 `impl` 宣告 `T`，才有 `T` 可以用來標明我們在替型別 `Point<T>` 實作其方法。在 `impl` 之後宣告泛型型別 `T`，Rust 可以識別出 `Point` 尖括號內的型別為泛型型別而非實際型別。我們其實可以選用不同的泛型參數名稱，而不用和結構體定義的泛型參數一樣，不過通常使用相同名稱還是比較常見。無論該泛型型別最終會是何種實際型別，任何方法在有宣告泛型型別的 `impl` 內，都會被定義成適用於各種型別實例。 

當我們在定義方法時，我們也可以對泛型型別加上些限制。舉例來說，我們可以只針對 `Point<f32>` 的實例來實作方法，而非適用於任何泛型型別的 `Point<T>` 實例。在範例 10-10 我們使用了實例型別 `f32` 而沒有在 `impl` 宣告任何型別。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">範例 10-10：一個只適用於擁有泛型 `T` 結構體其中的特定實際型別的 `impl` 區塊</span>

此程式碼代表 `Point<f32>` 會有個方法 `distance_from_origin`，其他 `Point<T>` 只要 `T` 不是型別 `f32` 的實例都不會定義此方法。此方法測量我們的點距離座標 (0.0, 0.0) 有多遠並使用只有浮點數型別能使用的數學運算。

在結構體定義中的泛型型別參數不會總是和結構體方法簽名中的相同。舉例來說，範例 10-11 在 `Point` 結構體中使用泛型型別 `X1` 和 `Y1`，但在 `mixup` 方法中就使用 `X2` `Y2` 以便清楚辨別。該方法用 `self` `Point` 的 `x` 值（型別為 `X1`）與參數傳進來的 `Point` 的 `y` 值（型別為 `Y2`）來建立新的 `Point` 實例。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">範例 10-11：結構體定義中使用不同的泛型型別的方法</span>

在 `main` 中，我們定義了一個 `Point`，其 `x` 型別為 `i32`（數值為 `5`），`y` 型別為 `f64`（數值為 `10.4`）。變數 `p2` 是個 `Point` 結構體，`x` 為字串切片（數值為 `"Hello"`），`y` 為 `char`（數值為 `c`）。在 `p1` 呼叫 `mixup` 並加上引數 `p2` 的話會給我們 `p3`，它的 `x` 會有型別 `i32`，因為 `x` 來自 `p1`。而且變數 `p3` 還會有型別為 `char` 的 `y`，因為 `y` 來自 `p2`。`println!` 巨集的呼叫就會顯示 `p3.x = 5, p3.y = c`。

此例是是為了展示一些泛型參數是透過 `impl` 宣告而有些則是透過方法定義來取得。泛型參數 `X1` 和 `Y1` 是宣告在 `impl` 之後，因為它們與結構體定義有關聯。泛型參數 `X2` 和 `Y2` 則是宣告在 `fn mixup` 之後，因為它們只與方法定義有關聯。

### 使用泛型的程式碼效能

你可能會好奇當你使用泛型型別參數會不會有執行時的消耗。好消息是使用泛型型別不會比使用實際型別還來的慢。

Rust 在編譯時對使用泛型的程式碼進行單型化（monomorphization）。**單型化**是個讓泛型程式碼轉換成特定程式碼的過程，在編譯時填入實際的型別。在此過程中，編譯器會做與我們在範例 10-5 建立泛型函式相反的事：編譯器檢查所有泛型程式碼被呼叫的地方，並依據泛型程式碼被呼叫的情況產生實際型別的程式碼。

讓我們看看這在標準函式庫的泛型枚舉 `Option<T>` 中是怎麼做到的：

```rust
let integer = Some(5);
let float = Some(5.0);
```

當 Rust 編譯此程式碼時中，他會進行單型化。在此過程中，會讀取 `Option<T>` 實例中使用的數值並識別出兩種 `Option<T>`：一種是 `i32` 而另一種是 `f64`。接著它就會將 `Option<T>` 的泛型定義展開為兩種定義 `i32` 與 `f64`，以此替換函式定義為特定型別。

單型化的版本看起來會像這樣（編譯器實際使用的名稱會和我們這邊示範的不同）：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

泛型 `Option<T>` 會被替換成編譯器定義的特定定義。因為 Rust 會編譯泛型程式碼成個別實例的特定型別，我們使用泛型就不會造成任何執行時消耗。當程式執行時，它就會和我們親自寫重複定義的版本一樣。單型化的過程讓 Rust 的泛型在執行時十分有效率。

[traits-as-parameters]: ch10-02-traits.html#特徵作為參數
