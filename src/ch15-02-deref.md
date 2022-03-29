## 透過 `Deref` 特徵將智慧指標視為一般引用

實作 `Deref` 特徵讓你可以自訂**解引用運算子（dereference operator）** `*` 的行為（這不是相乘或全域運算子）。透過這種方式實作 `Deref` 的智慧指標可以被視為正常引用來對待，這樣操作引用的程式碼也能用在智慧指標中。

讓我們先看解引用運算子如何在正常引用中使用。然後我們會嘗試定義一個行為類似 `Box<T>` 的自定型別，並看看為何解引用運算子無法像引用那樣用在我們新定義的型別。我們將會探討如何實作 `Deref` 特徵使智慧指標能像類似引用的方式運作。接著我們會看看 Rust 的**強制解引用**（deref coercion）功能並瞭解它如何處理引用與智慧指標。

> 注意：我們即將定義的 `MyBox<T>` 型別與真正的 `Box<T>` 有一項很大的差別，就是我們的版本不會將其資料儲存在堆積上。我們在此例會專注在 `Deref` 上，所以資料實際上儲存在何處，並沒有比指標相關行為來得重要。

### 使用解引用運算子追蹤指標的數值

一般的引用是一種指標，其中一種理解指標的方式是看成一個會指向存於某處數值的箭頭。在範例 15-6 中我們建立了數值 `i32` 的引用，接著使用解引用運算子來追蹤引用的資料：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

<span class="caption">範例 15-6：使用解引用運算子來追蹤數值 `i32` 的引用</span>

變數 `x` 存有 `i32` 數值 `5`。我們將 `y` 設置為 `x` 的引用。我們可以判定 `x` 等於 `5`。不過要是我們想要判定 `y` 數值的話，我們需要使用 `*y` 來追蹤引用指向的數值（也就是**解引用**）。一旦我們解引用 `y`，我們就能取得 `y` 指向的整數數值並拿來與 `5` 做比較。

如果我們嘗試寫說 `assert_eq!(5, y);` 的話，我們會得到此編譯錯誤：

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

比較一個數字與一個數字的引用是不允許的，因為它們是不同的型別。我們必須使用解引用運算子來追蹤其指向的數值。

### 像引用般使用 `Box<T>`

我們將範例 15-6 的引用改用 `Box<T>` 重寫。解引用運算子的使用方式如範例 15-7 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

<span class="caption">範例 15-7：對 `Box<i32>` 使用解引用運算子</span>

範例 15-7 與範例 15-6 唯一的差別在於這裡我們設置 `y` 為一個指向 `x` 的拷貝數值的 box 實例，而不是指向 `x` 數值的引用。在最後的判定中，我們可以對 box 的指標使用解引用運算子，跟我們對當 `y` 還是引用時所做的動作一樣。接下來，我們要來探討 `Box<T>` 有何特別之處，讓我們可以對自己定義的 box 型別也可以使用解引用運算子。

### 定義我們自己的智慧指標

讓我們定義一個與標準函式庫所提供的 `Box<T>` 型別類似的智慧指標，並看看智慧指標預設行為與引用有何不同。然後我們就會來看能夠使用解引用運算子的方式。

`Box<T>` 本質上就是定義成只有一個元素的元組結構體，所以範例 15-8 用相同的方式來定義 `MyBox<T>`。我們也定義了 `new` 函式來對應於 `Box<T>` 的 `new` 函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

<span class="caption">範例 15-8：定義 `MyBox<T>` 型別</span>

我們定義了一個結構體叫做 `MyBox` 並宣告一個泛型參數 `T`，因為我們希望我們的型別能存有任何型別的數值。`MyBox` 是個只有一個元素型別為 `T` 的元組結構體。`MyBox::new` 函式接受一個參數型別為 `T` 並回傳存有該數值的 `MyBox` 實例。

讓我們將範例 15-7 的 `main` 函式加到範例 15-8 並改成使用我們定義的 `MyBox<T>` 型別而不是原本的 `Box<T>`。範例 15-9 的程式碼無法編譯，因為 Rust 不知道如何解引用`MyBox`。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

<span class="caption">範例 15-9：嘗試像使用 `Box<T>` 和引用一樣的方式來使用 `MyBox<T>`</span>

以下是編譯結果出現的錯誤：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

我們的 `MyBox<T>` 型別無法解引用因為我們還沒有對我們的型別實作該能力。要透過 `*` 運算子來解引用的話，我們要實作 `Deref` 特徵。

### 透過實作 `Deref` 特徵來將一個型別能像引用般對待

如同第十章講過的，要實作一個特徵的話，我們需要提供該特徵要求的方法實作。標準函式庫所提供的 `Deref` 特徵要求我們實作一個方法叫做 `deref`，這會借用 `self` 並回傳內部資料的引用。範例 15-10 包含了對 `MyBox` 定義加上的 `Deref` 實作：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

<span class="caption">範例 15-10：對 `MyBox<T>` 實作 `Deref`</span>

`type Target = T;` 語法定義了一個供 `Deref` 特徵使用的關聯型別。關聯型別與宣告泛型參數會有一點差別，但是你現在先不用擔心它們，我們會在第十九章深入探討。

我們對 `deref` 的方法本體加上 `&self.0`，`deref` 就可以回傳一個引用讓我們可以使用 `*` 運算子取得數值。範例 15-9 的 `main` 函式現在對 `MyBox<T>` 數值的 `*` 呼叫就可以編譯了，而且判定也會通過！

沒有 `Deref` 特徵的話，編譯器只能解引用 `&` 的引用。`deref` 方法讓編譯器能夠從任何有實作 `Deref` 的型別呼叫 `deref` 方法取得 `&` 引用，而它就可以進一步解引用獲取數值。

當我們在範例 15-9 中輸入 `*y` 時，Rust 背後實際上是執行此程式碼：

```rust,ignore
*(y.deref())
```

Rust 將 `*` 運算子替換為方法 `deref` 的呼叫再進行普通的解引用，所以我們不必煩惱何時該或不該呼叫 `deref` 方法。此 Rust 特性讓我們可以對無論是引用或是有實作 `Deref` 的型別都能寫出一致的程式碼。

`deref` 方法會回傳一個數值引用，以及括號外要再加上普通解引用的原因，都是因為所有權系統。如果 `deref` 方法直接回傳數值而非引用數值的話，該數值就會移出 `self`。我們不希望在此例或是大多數使用解引用運算子的場合下，取走 `MyBox<T>` 內部數值的所有權。

注意到每次我們在程式碼中使用 `*` 時，`*` 運算子被替換成 `deref` 方法呼叫，然後再呼叫 `*` 剛好一次。因為 `*` 運算子不會被無限遞迴替換，我們能剛好取得型別 `i32` 並符合範例 15-9 `assert_eq!` 中與 `5` 的判定。

### 函式與方法的隱式強制解引用

**強制解引用*（Deref coercion）是一個 Rust 針對函式或方法的引數的便利設計。強制解引用只適用於有實作 `Deref` 特徵的型別。強制解引用會將一個型別轉換成另一個型別的引用。舉例來說，強制解引用可以轉換 `&String` 成 `&str`，因為 `String` 有實作 `Deref` 特徵並能用它來回傳 `&str`。當我們將某個特定型別數值的引用作為引數傳入一個函式或方法，但該函式或方法所定義的參數卻不相符時，強制解引用就會自動發生，並進行一系列的 `deref` 方法呼叫，將我們提供的型別轉換成參數所需的型別。

Rust 會加入強制解引用的原因是因為程式設計師在寫函式與方法呼叫時，就不必加上許多顯式引用 `&` 與解引用 `*`。強制解引用還讓我們可以寫出能同時用於引用或智慧指標的程式碼。

為了展示強制解引用，讓我們使用範例 15-8 定義的 `MyBox<T>` 型別以及範例 15-10 所加上的 `Deref` 實作。範例 15-11 中定義的函式使用字串切片作為參數：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

<span class="caption">範例 15-11：`hello` 函式且有參數 `name` 其型別為 `&str`</span>

我們可以使用字串切片作為引數來呼叫函式 `hello`，比方說 `hello("Rust");`。強制解引用讓我們可以透過 `MyBox<String>` 型別數值的引用來呼叫 `hello`，如範例 15-12 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

<span class="caption">範例 15-12：利用強制解引用透過 `MyBox<String>` 數值的引用來呼叫 `hello`</span>

我們在此使用 `&m` 作為引數來呼叫函式 `hello`，這是 `MyBox<String>` 數值的引用。因為我們在範例 15-10 有對 `MyBox<T>` 實作 `Deref` 特徵，Rust 可以呼叫 `deref` 將 `&MyBox<String>` 變成 `&String`。標準函式庫對 `String` 也有實作 `Deref` 並會回傳字串切片，這可以在 `Deref` 的 API 技術文件中看到。所以 Rust 會在呼叫 `deref` 一次來將 `&String` 變成 `&str`，這樣就符合函式 `hello` 的定義了。

如果 Rust 沒有實作強制解引用的話，我們就得用範例 15-13 的方式才能辦到範例 15-12 使用型別 `&MyBox<String>` 的數值來呼叫 `hello` 的動作。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

<span class="caption">範例 15-13：如果 Rust 沒有強制解引用，我們就得這樣寫程式碼</span>

`(*m)` 會將 `MyBox<String>` 解引用成 `String`，然後 `&` 和 `[..]` 會從 `String` 中取得等於整個字串的字串切片，這就符合 `hello` 的簽名。沒有強制解引用的程式碼就難以閱讀、寫入或是理解，因為有太多的符號參雜其中。強制解引用能讓 Rust 自動幫我們做這些轉換。

當某型別有定義 `Deref` 特徵時，Rust 會分析該型別並重複使用 `Deref::deref` 直到能取得與參數型別相符的引用。`Deref::deref` 需要呼叫的次數會在編譯時期插入，所以使用強制解引用沒有任何的執行時開銷！

### 強制解引用如何處理可變性

類似於你使用 `Deref` 特徵來覆蓋不可變引用的 `*` 運算子的方式，你也可以使用 `DerefMut` 特徵來覆蓋可變引用的 `*` 運算子。

當 Rust 發現型別與特徵實作符合以下三種情況時，它就會進行強制解引用：

* 從 `&T` 到 `&U` 且 `T: Deref<Target=U>`
* 從 `&mut T` 到 `&mut U` 且 `T: DerefMut<Target=U>`
* 從 `&mut T` 到 `&U` 且 `T: Deref<Target=U>`

前兩個除了可變性之外是相同的。第一個情況表示如果你有個 `&T` 且 `T` 有實作 `Deref` 到某個型別 `U`，你就可以直接得到 `&U`。第二種情況指的則是對可變引用的強制解引用。

第三種情況比較棘手：Rust 也能強制將可變引用轉為一個不可變引用。但反過來是**不可行**的：不可變引用永遠不可能強制解引用成可變引用。由於借用規則，如果你有個可變引用，該可變引用必須是該資料的唯一引用（不然程式無法編譯）。轉換可變引用成不可變引用不會破壞借用規則。轉換不可變引用成可變引用的話，就需要此不可變引用是該資料的唯一引用，但借用規則無法做擔保。因此 Rust 無法將不可變引用轉換成可變引用。
