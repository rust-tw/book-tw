## 允許不同型別數值的特徵物件

在第八章中，我們提及向量其中一項限制是它儲存的元素只能有一種型別。我們在範例 8-10 提出一個替代方案，那就是我們定義 `SpreadsheetCell` 枚舉且其變體能存有整數、浮點數與文字。這讓我們可以對每個元素儲存不同的型別，且向量仍能代表元素的集合。當我們的可變換的項目有固定的型別集合，而且我們在編譯程式碼時就知道的話，這的確是完美的解決方案。

然而，有時我們會希望函式庫的使用者能夠在特定的情形下擴展型別的集合。為了展示我們如何達成，我們來建立個圖形使用者介面（graphical user interface, GUI）工具範例來遍歷一個項目列表，呼叫其內每個項目的 `draw` 方法將其顯示在螢幕上，這是 GUI 工具常見的技巧。我們會建立個函式庫 crate 叫做 `gui`，這會包含 GUI 函式庫的結構體。此 crate 可能會包含一些人們會使用到的型別，像是 `Button` 或 `TextField`。除此之外，`gui` 使用者也能夠建立他們自己的型別來顯示出來。舉例來說，有些開發者可以加上 `Image` 而其他人可能會加上 `SelectBox`。

我們在此例中不會實作出整個 GUI 函式庫，但會展示各個元件是怎麼組合起來的。在寫函式庫時，我們無法知道並定義開發者想建立出來的所有型別。但我們知道 `gui` 需要追蹤許多不同型別的數值，且它需要能夠對這些不同的型別數值呼叫 `draw` 方法。它不需要知道當我們呼叫 `draw` 方法時實際發生了什麼事，只需要知道該數值有我們可以呼叫的方法。

在有繼承的語言中，我們可能會定義一個類型（class）叫做 `Component` 且其有個方法叫做 `draw`。其他的類型像是 `Button`、`Image` 和 `SelectBox` 等等，可以繼承 `Component` 以取得 `draw` 方法。它們可以覆寫 `draw` 方法來定義它們自己的自訂行為，但是整個框架能將所有型別視為像是 `Component` 實例來對待，並對它們呼叫 `draw`。但由於 Rust 並沒有繼承，我們需要其他方式來組織 `gui` 函式庫，好讓使用者可以透過新的型別來擴展它。

### 定義共同行為的特徵

要定義我們希望 `gui` 能擁有的行為，我們定義一個特徵叫做 `Draw` 並有個方法叫做 `draw`。然後我們可以定義一個接收**特徵物件**（trait object）的向量。一個特徵物件會指向有實作指定特徵的型別以及一個在執行時尋找該型別方法的尋找表（look up table）。要建立特徵物件，我們指定一些指標，像是引用 `&` 或者智慧指標 `Box<T>`，然後加上 `dyn` 關鍵字與指定的相關特徵。（我們會在第十九章的[「動態大小型別與 `Sized` 特徵」][dynamically-sized]<!-- ignore -->段落討論特徵物件必須使用指標的原因）我們可以對泛型或實際型別使用特徵物件。當我們使用特徵物件時，Rust 的型別系統會確保在編譯時該段落使用到的任何數值都有實作特徵物件的特徵。於是我們就不必在編譯時知道所有可能的型別。

我們提到在 Rust 中，我們避免將結構體和枚舉稱為「物件」，來與其他語言的物件做區別。在結構體或枚舉中，結構你欄位中的資料與 `impl` 區塊的行為是分開來的。在其他語言中，資料與行為會組合成一個概念，也就是所謂的物件。然而特徵物件才比較像是其他語言中的物件，因為這才會將資料與行為組合起來。但特徵物件與傳統物件不同的地方在於，我們無法向特徵物件新增資料。特徵物件不像其他語言的物件那麼通用，它們是特別用於對共同行為產生的抽象概念。

範例 17-3 定義了一個特徵叫做 `Draw` 以及一個方法叫做 `draw`：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<span class="caption">範例 17-3：`Draw` 特徵的定義</span>

此語法和我們在第十章介紹過的特徵定義方式相同。接下來才是新語法用到的地方，範例 17-4 定義了一個結構體叫做 `Screen` 並持有個向量叫做 `components`。此向量的型別為 `Box<dyn Draw>`，這是一個特徵物件，這代表 `Box` 內的任何型別都得有實作 `Draw` 特徵。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

<span class="caption">範例 17-4：定義結構體 `Screen` 且有個 `components` 欄位來持有一個實作 `Draw` 特徵的特徵物件向量</span>

在 `Screen` 結構體中，我們定義了一個方法叫做 `run` 來對其 `components` 呼叫 `draw` 方法，如範例 17-5 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<span class="caption">範例 17-5：`Screen` 的方法 `run` 會呼叫每個 `component` 的 `draw` 方法</span>

這與定義一個結構體並使用附有特徵界限的泛型型別參數的方式不相同。泛型型別參數一次只能替換成一個實際型別，特徵物件則是在執行時允許數個實際型別能填入特徵物件中。舉例來說，我們可以使用泛型型別與特徵界限來定義 `Screen`，如範例 17-6 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<span class="caption">範例 17-6：`Screen` 結構體的另種實作方式，它的方法 `run` 則使用泛型與特徵界限</span>

這樣我們會限制 `Screen` 實例必須擁有一串全是 `Button` 型別或全是 `TextField` 型別的列表。如果你只會有同型別的集合，使用泛型與特徵界限的確是比較合適的，因為其定義就會在編譯時單態化為使用實際型別。

另一方面，透過使用特徵物件的方法，`Screen` 實例就能有個同時包含 `Box<Button>` 與 `Box<TextField>` 的 `Vec<T>`。 讓我們看看這如何辦到的，然後我們會討論其對執行時效能的影響。

### 實作特徵

現在我們來加上一些有實作 `Draw` 特徵的型別。我們會提供 `Button` 型別。再次重申 GUI 函式庫的實際實作超出了本書的範疇，所以 `draw` 的本體不會有任何有意義的實作。為了想像該實作會像是什麼，`Button` 型別可能會有欄位 `width`、`height` 與 `label`，如範例 17-7 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<span class="caption">範例 17-7：結構體 `Button` 實作了 `Draw` 特徵</span>

在 `Button` 中的 `width`、`height` 與 `label` 欄位會與其他元件不同，像是 `TextField` 可能就會有前面所有的欄位在加上 `placeholder` 欄位。每個我們想在螢幕上顯示的型別都會實作 `Draw` 特徵，但在 `draw` 方法會使用不同程式碼來定義如何印出該特定型別，像是這裡的 `Button` 型別（不包含實際 GUI 程式碼，因為這超出本章範疇）。舉例來說，`Button` 可能會有額外的 `impl` 區塊來包含使用者點擊按鈕時該如何反應的方法。這種方法就不適用於 `TextField`。

如果有人想用我們的函式庫來實作個 `SelectBox` 結構體並擁有 `width`、`height` 與 `options` 欄位的話，他們也可以對 `SelectBox` 實作 `Draw` 特徵，如範例 17-8 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<span class="caption">範例 17-8：別的 crate 使用 `gui` 來對 `SelectBox` 結構體實作 `Draw` 特徵</span>

我們的函式庫使用者現在可以在他們的 `main` 建立個 `Screen` 實例。在 `Screen` 實例中，他們可以透過將 `SelectBox` 和 `Button` 放入 `Box<T>` 來成為特徵物件並加入元件中。他們接著就可以對 `Screen` 實例呼叫 `run` 方法，這會呼叫每個元件的 `draw` 方法。如範例 17-9 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<span class="caption">範例 17-9：使用特徵物件來儲存實作相同特徵的不同型別數值</span>

我們在寫函式庫時，我們並不知道有人會想要新增 `SelectBox` 型別，但我們的 `Screen` 實作能夠運用新的型別並顯示出來，因為 `SelectBox` 有實作 `Draw` 特徵，這代表它就有實作 `draw` 方法。

這種只在意數值回應的訊息而非數值實際型別的概念，類似於動態型別語言中**鴨子型別**（duck typing）的概念。如果它走起來像隻鴨子、叫起來像隻鴨子，那它必定是隻鴨子！在範例 17-5 中 `Screen` 的 `run` 實作不需要知道每個元件的實際型別為何。它不會檢查一個元件是 `Button` 還是 `SelectBox` 實例，它只會呼叫元件的 `draw` 方法。透過指定 `Box<dyn Draw>` 來作為 `components`向量中的數值型別，我們定義 `Screen` 需要我們能夠呼叫 `draw` 方法的數值。

我們使用特徵物件與 Rust 型別系統能寫出類似鴨子型別的程式碼，這樣的優勢在於我們在執行時永遠不必檢查一個數值有沒有實作特定方法，或擔心我們會不會呼叫了一個沒有實作該方法的數值而產生錯誤。如果數值沒有實作特徵物件要求的特徵的話，Rust 不會編譯通過我們的程式碼。

舉例來說，範例 17-10 展示了要是我們嘗試使用 `String` 作為元件來建立 `Screen` 的話會發生什麼事：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

<span class="caption">範例 17-10：嘗試使用沒有實作特徵物件的特徵的型別</span>

我們會因為 `String` 沒有實作 `Draw` 特徵而得到錯誤：

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

此錯誤讓我們知道要麼我們傳遞了不希望傳給 `Screen` 的型別所以應該要傳遞其他型別，要麼我們應該要對 `String` 實作 `Draw`，這樣 `Screen` 才能對其呼叫 `draw`。

### 特徵物件執行動態調度

回想一下第十章的[「使用泛型的程式碼效能」][performance-of-code-using-generics]<!-- ignore -->段落我們討論過，當我們對泛型使用閉包時，編譯器會執行單態化（monomorphization）的過程。編譯器會在我們對每個用泛型型別參數取代的實際型別產生非泛型的函式與方法實作。單態化產生程式碼的動作會稱為「靜態調度（static dispatch）」，這代表編譯器在編譯時知道我們呼叫的方法為何。與其相反的則是**動態調度（dynamic dispatch）**，這種方式時編譯器在編譯時無法知道你呼叫的方法為何。在動態調度的情況下，編譯器會生成在執行時能夠確定會呼叫何種方法的程式碼。

當我們使用特徵物件時，Rust 必須使用動態調度。編譯器無法知道使用特徵物件的程式碼會使用到的所有型別為何，所以它會不知道該呼叫哪個型別的哪個實作方法。取而代之的是，Rust 在執行時會使用特徵物件內部的指標來知道該呼叫哪個方法。這樣尋找的動作會產生靜態調度所沒有的執行時開銷。動態調度也讓編譯器無法選擇內聯（inline）方法的程式碼，這樣會因而阻止一些優化。不過我們的確對範例 17-5 的程式碼增加了額外的彈性，並能夠支援範例 17-9，所以這是個權衡取捨。

### 特徵物件要求物件安全

特徵物件只能使用**物件安全**（object-safe）的特徵。Rust 會有一些複雜的規則來檢測其屬性以確保特徵物件安全，不過實際上，我們只需要在意兩條規則。如果特徵定義的所有方法遵守以下屬性的話，該特徵就是物件安全的：

* 回傳值不是 `Self`。
* 沒有泛型型別參數。

`Self` 關鍵字是我們所實作特徵或方法的型別的別名。特徵物件必須是物件安全的，因為一旦你使用特徵物件後，Rust 就無法知道實作該特徵的型別為何。如果特徵方法回傳實際 `Self` 型別，但特徵物件忘記 `Self` 的確切型別的話，該方法不可能有辦法使用原本的實際型別。同理對於泛型型別參數來說，當特徵被使用到時，其就會填入實際的型別參數，實際型別變成了實作特徵的型別的一部分。當型別被使用的特徵物件遺忘時，就無從得知該填素何種泛型型別參數。

標準函式庫中其中一個不是物件安全的特徵範例是 `Clone` 特徵。`Clone` 特徵中的 `clone` 方法簽名長得像這樣：

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

`String` 型別有實作 `Clone` 特徵，而當我們呼叫 `String` 實例的 `clone` 方法時，我們會取得 `String` 的實例。同樣地，如果我們呼叫 `Vec<T>` 的 `clone`，我們就會得到 `Vec<T>` 的實例。`clone` 的簽名需要知道 `Self` 的實際型別為何，因為這是它的回傳型別。

當你嘗試違反特徵物件的物件安全規則時，編譯器會提醒警告你。舉例來說，假設我們嘗試實作範例 17-4 的 `Screen` 結構體為改儲存實作 `Clone` 特徵的型別而非 `Draw` 特徵，如以下所示：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/no-listing-01-trait-object-of-clone/src/lib.rs}}
```

我們就會得到此錯誤：

```console
{{#include ../listings/ch17-oop/no-listing-01-trait-object-of-clone/output.txt}}
```

此錯誤表示你不能這樣在特徵物件使用此特徵。如果你對物件安全的細節有興趣的話，歡迎查閱 [Rust RFC 255] 或[參考手冊][object-safety-reference]的物件安全段落。

[Rust RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md

[performance-of-code-using-generics]:
ch10-01-syntax.html#使用泛型的程式碼效能
[dynamically-sized]: ch19-04-advanced-types.html#動態大小型別與-sized-特徵
[object-safety-reference]: https://doc.rust-lang.org/stable/reference/items/traits.html#object-safety
