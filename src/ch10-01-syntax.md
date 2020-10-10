## 泛型資料型別

我們可以使用泛型（generics）來建立項目的定義，像是函式簽名或結構體，讓我們在之後可以使用在不同的實際資料型別。讓我們先看看如何使用泛型定義函式、枚舉與方法。然後我們會在來看泛型對程式碼的效能影響如何。

### 在函式中定義

當要使用泛型定義函數時，我們通常會將泛型置於函式簽名中指定參數與回傳值資料型別的位置。這樣做能讓我們的程式碼更具彈性並向呼叫者提供更多功能，同時還能防止重複程式碼。

接續我們 `largest` 函式的例子，範例 10-4 展示了兩個都在切片上尋找最大值的函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">範例 10-4：兩個名稱與其簽名中的型別都不同的函式</span>

`largest_i32` 函式和我們在範例 10-3 提取的函式一樣都是尋找切片中最大的 `i32`。而 `largest_char` 函式則尋找切片中最大的 `char`。函式本體都擁有相同的程式碼，讓我以讓我們來開始用泛型型別參數來消除重複的部分，轉變成只有一個函式吧。

要在我們新定義的函式中參數化型別的話，我們需要為參數型別命名，就和我們在函式中的參數數值所做的一樣。你可以用任何標識符來命名型別參數名稱。但我們習慣上會用 `T`，因為 Rust 的參數名稱都盡量很短，常常只會有一個字母，而且 Rust 對於型別命名的慣用規則是駝峰式大小寫（CamelCase）。所以 `T` 作為「type」的簡稱是大多數 Rust 程式設計師的選擇。

當我們在函式本體使用參數時，我們必須在簽名中宣告參數名稱，編譯器才能之當該名稱代表什麼。同樣地，當我們要在函式簽名中使用型別參數名稱，我們必須在使用前宣告該型別參數名稱。要定義泛型 `largest` 函式的話，我們在函式名稱與參數列表之間加上尖括號，其內就是型別名稱的宣告，如以下所示：

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

我們可以這樣理解定義：函式 `largest` 有泛型型別 `T`，此函式有一個參數叫做 `list`，它的型別為數值 `T` 的切片。`largest` 函式會回傳與型別 `T` 相同型別的值。

範例 10-5 顯示了使用泛型資料型別於函式簽名組合出的 `largest` 函式。此範例還展示了我們如何依序用 `i32` 和 `char` 的切片呼叫函式。注意此程式碼尚未能編譯，不過我們會在本章之後修改它。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">範例 10-5：使用函式型別參數定義的 `largest` 函式，但現在還不能編譯</span>

如果我們現在就編譯程式碼的話，我們會得到此錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

註釋中提到了 `std::cmp::PartialOrd` 這個*特徵（trait）*。我們會在下個段落來討論特徵。現在此錯誤告訴我們 `largest` 本體無法適用於所有可能的 `T` 型別，因為我們想要在本體中比較型別 `T` 的數值，我們只能在能夠排序的型別中做比較。要能夠比較的話，標準函式庫有提供 `std::cmp::PartialOrd` 特徵讓你可以針對你的型別來實作（請查閱附錄 C 來瞭解更多此特徵的細節）。你會在[「特徵作為參數」][traits-as-parameters]<!-- ignore -->的段落學習到如何指定特定泛型型別擁有特定特徵。不過先讓先我們探索其他泛型型別參數使用的方式。

### 在結構體中定義

我們一樣能以 `<>` 語法來對結構體中一或多個欄位使用泛型型別參數。範例 10-6 顯示了如何定義 `Point<T>` 結構體並讓 `x` 與 `y` 可以是任意型別數值。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">範例 10-6：`Point<T>` 結構體的 `x` 與 `y` 會有型別 `T` 的數值</span>

在結構體定義使用泛型的語法與函式定義類似。首先，我們在結構體名稱後方加上尖括號，並在其內宣告型別參數名稱。接著我們能在原本指定實際資料型別的地方，使用泛型型別來定義結構體。

注意到我們使用了一個泛型型別來定義 `Point<T>`，此定義代表 `Point<T>` 是某型別 `T` 下之通用的，而且欄位 `x` 與 `y` 擁有*相同*型別，無論最終是何種型別。如果我們用不同的型別數值來建立 `Point<T>` 實例，我們的程式碼會無法編譯，如範例 10-7 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">範例 10-7：欄位 `x` 與 `y` 必須是相同型別，因為它們擁有相同的泛型資料型別 `T`</span>

在此例中，當我們賦值 5 給 `x` 時，我們讓編譯器知道 `Point<T>` 實例中的泛型型別 `T` 會是整數。然後我們將 4.0 賦值給 `y`，這應該要和 `x` 有相同型別，所以我們會獲得以下錯誤：

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

要將結構體 `Point` 的 `x` 與 `y` 定義成擁有不同型別確仍然是泛型的話，我們可以使用多個泛型型別參數。舉例來說，在範例 10-8 我們改變了 `Point` 的定義為擁有兩個泛型型別 `T` 與 `U`，`x` 擁有型別 `T` 而 `y` 擁有型別 `U`。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">範例 10-8：`Point<T, U>` 擁有兩個泛型惜別，所以 `x` 和 `y` 可以有不同的型別數值</span>

現在這些所有的 `Point` 實例都是允許的了！你要在定義中使用多少泛型型別參數都沒問題，但用太多的話會讓你的程式碼難以閱讀。當你的程式碼需要使用大量泛型的話，通常代表你的程式碼需要重新組織成更小的元件。

### 在枚舉中定義

如同結構體一樣，我們可以定義枚舉讓它們的變體擁有泛型資料型別。讓我們看看我們在第六章標準函式庫提供的 `Option<T>` 枚舉：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

此定義現在對你來說應該就說的通了。如同你所看到的 `Option<T>` 枚舉有個泛型型別參數 `T` 以及兩個變體：`Some` 擁有型別 `T` 的數值；而 `None` 則是不具任何數值的變體。使用 `Option<T>` 枚舉我們可以表達出一個可能擁有的數值這樣的抽象概念。而且因為 `Option<T>` 是泛型，不管可能的數值型別為何，我們都能使用此抽象。

枚舉也能有數個泛型型別。我們在第九章所使用枚舉 `Result` 的定義就是個例子：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 枚舉有兩個泛型型別 `T` 和 `E` 且有兩個變體：`Ok` 擁有型別 `T` 的數值；而 `Err` 擁有型別 `E` 的數值。這樣的定義讓我們很方便能表達 `Result` 枚舉可能擁有一個成功的數值（返回型別 `T` 的數值）或失敗的數值（回傳型別為 `E` 的錯誤值）。事實上這就是我們在範例 9-3 開啟檔案的方式，當我們成功開啟檔案時的 `T` 就會是型別 `std::fs::File`，然後當開啟檔案會發生問題時 `E` 就會是型別 `std::io::Error`。

當你發現你的程式碼有許多結構體或枚舉都只有儲存的值有所不同時，你可以使用泛型行別來避免重複。

### 在方法中定義

我們可以對結構體或枚舉定義方法（如第五章所述）並也可以使用泛型型別來定義。範例 10-9 展示了我們在範例 10-6 定義的結構體 `Point<T>` 並實作了一個叫做 `x` 的方法。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">範例 10-9：在 `Point<T>` 結構體實作一個方法叫叫做 `x`，其會回傳 `x` 欄位中型別為 `T` 的引用</span>

我們在這 `Point<T>` 定義了一個方法叫做 `x` 並回傳欄位 `x` 的資料引用。

注意到我們需要在 `impl` 宣告 `T`，這樣才代表我們指的是在型別 `Point<T>` 實作方法。在 `impl` 之後宣告泛型型別 `T`，Rust 可以識別出 `Point` 尖括號內的型別為泛型型別而非實際型別。

舉例來說，我們可以只針對 `Point<f32>` 的實例來實作方法，而非適用於任何泛型型別的 `Point<T>` 實例。在範例 10-10 我們使用了實例型別 `f32` 而沒有在 `impl` 宣告任何型別。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">範例 10-10：一個只適用於擁有泛型 `T` 結構體其中的特定實際型別的 `impl` 區塊</span>

此程式碼代表 `Point<f32>` 會有個方法叫做 `distance_from_origin` 但其他 `Point<T>` 只要 `T` 不是型別 `f32` 的實例都不會定義此方法。此方法測量我們的點距離座標 (0.0, 0.0) 有多遠並使用只有浮點數型別能使用的數學運算。

在結構體定義中的泛型型別參數不總會是和結構體方法簽名中的會是相同型別。舉例來說，範例 10-11 在範例 10-8 的 `Point<T, U>` 定義了一個方法 `mixup`。該方法會取得另一個 `Point` 作為參數，不過其可能會與我們呼叫的 `mixup` 方法中`self` `Point` 的型別有所相異。此方法使用 `self` 的 `Point`（型別為 `T`）的 `x` 值與由參數傳進來的 `Point`（型別為 `W`）的 `y` 值。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">範例 10-11：結構體定義中使用不同的泛型型別的方法</span>

在 `main` 中，我們定義了一個 `Point`，其 `x` 型別為 `i32`（數值為 `5`），`y` 型別為 `f64`（數值為 `10.4`）。變數 `p2` 是個 `Point` 結構體，`x` 為字串切片（數值為 `"Hello"`），`y` 為 `char`（數值為 `c`）。在 `p1` 呼叫 `mixup` 並加上引數 `p2` 的話會給我們 `p3`，它的 `x` 會有型別 `i32`，因為 `x` 來自 `p1`。而且變數 `p3` 還會有型別為 `char` 的 `y`，因為 `y` 來自 `p2`。`println!` 巨集的呼叫就會顯示 `p3.x = 5, p3.y = c`。

此例是是為了展示一些泛型參數是透過 `impl` 宣告而有些則是透過方法定義來得。在此，泛型參數 `T` 和 `U` 是宣告在 `impl` 之後，因為它們與結構體定義有關聯。而在 `fn mixup` 之後宣告的泛型參數只和該方法有關。

### 使用泛型的程式碼效能

你可能會好奇當你使用泛型型別參數會不會有執行時的消耗。好消息是 Rust 實作泛型的方式讓你使用泛型的的程式碼跑得不會比使用實際型別還來的慢。

Rust 在編譯時對使用泛型的程式碼進行單態化（monomorphization）。*單態化*是個讓泛型程式碼轉換成特定程式碼的過程，在編譯時填入實際的型別。

在此過程中，編譯器會做與我們在範例 10-5 建立泛型函式相反的事：編譯器檢查所有泛型程式碼被呼叫的地方，並依據泛型程式碼被呼叫的情況產生實際型別的程式碼。

讓我們看看這在標準函式庫的枚舉 `Option<T>` 中是怎麼做到的：

```rust
let integer = Some(5);
let float = Some(5.0);
```

當 Rust 編譯此程式碼時中，他會進行單態化。在此過程中，會讀取 `Option<T>` 實例中使用的數值並識別出兩種 `Option<T>`：一種是 `i32` 而另一種是 `f64`。接著它就會將 `Option<T>` 的泛型定義展開為 `Option_i32` 和 `Option_f64`，以此替換函式定義為特定型別。

單態化的版本看起來會像這樣，泛型 `Option<T>` 會被替換成編譯器定義的特定定義：

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

因為 Rust 會編譯泛型程式碼成個別實例的特定型別，我們使用泛型就不會造成任何執行時消耗。當程式執行時，它就會和我們親自寫重複定義的版本一樣。單態化的過程讓 Rust 的泛型在執行時十分有效率。

[traits-as-parameters]: ch10-02-traits.html#特徵作為參數

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch10-01-syntax.md)
> - updated: 2020-09-14
