## 進階特徵

我們在第十章[「特徵：定義共享行爲」][特徵-定義共享行爲]一節首次提及特徵（trait），但和生命週期（lifetime）一樣，對其進階細節並無著墨。現在你已熟稔 Rust ，了解箇中真諦的時機已至。

### 利用關聯型別在特徵定義中指定佔位符型別

*關聯型別（associated types）* 連結了一個型別佔位符（placeholder）與一個特徵，可以將這些佔位符型別使用在這些特徵所定義的方法簽名上。對特定實作來說，特徵的實作者必須指明在這個型別位置上會用到的具體型別。如此一來，我們可以定義一個特色，使用了某個型別，但直到特徵被實作之前，都不需知道實際上的型別。

多數在本章提及的進階特色都較少使用，而關聯型別則是介於其中：他們比書中其他內容來得少用，但比本章介紹的其他特色來得更常見。

一個具有關聯型別的特徵之範例是標準函式庫提供的 `Iterator` 特徵。這例子中的關聯型別叫做 `Item`，表示一型別實作 `Iterator` 特徵時，會被迭代的那些值的型別。範例 19-12 展示了在第十三章[「`Iterator` 特徵與 `next` 方法」][iterator-特徵與-next-方法]一節提及的 `Iterator` 特徵定義：

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-12/src/lib.rs}}
```

<span class="caption">範例 19-12：`Iterator` 特徵自帶一個關聯型別</span>

`Item` 型別是個佔位符型別，`next` 方法的定義顯示它會回傳型別為 `Option<Self::Item>` 之值。`Iterator` 特徵的實作者會指定 `Item` 的具體型別，而 `next` 方法則會回傳一個包含該具體型別的值的一個 `Option`。

關聯型別可能看起來和泛型的概念非常相似，而後者允許定義函式而不需指定該函式可以處理何種型別。那為什麼我們還需要關聯型別？

讓我們透過第十三章的例子，來檢視以下兩者概念上的差異，這例子的 `Counter` 結構體實作了 `Iterator` 特徵。範例 13-21 中，我們指定的 `Item` 的型別為 `u32`

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-13-21-reproduced/src/lib.rs:ch19}}
```

語法似乎和泛型很像，所以為什麼我們不使用泛型定義 `Iterator` 特徵，如範例 19-13 所示？

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-13/src/lib.rs}}
```

<span class="caption">範例 19-13: 假設使用泛型來定義 `Iterator` 特徵</span>

差別在於使用泛型時，如範例 19-13 所示，由於我們可以實作出 `Iterator<String> for Counter` 或以任意多個其他泛型型別來替 `Counter` 實作 `Iterator`，因此，必須在每個實作都標註該型別。換言之，當一特徵擁有泛型參數，一型別即可透過改變泛型型別參數（generic type parameter）的具體型別，進而實作該特徵多次。於是，當我們使用 `next` 方法時，必須提供型別標註，指名要用哪個 `Iterator` 的實作。

有了關聯型別，同個型別就不能實作同個特徵多次，所以我們不需要標註型別。範例 19-12 中的定義用上了關聯型別，因為只能擁有一個 `impl Iterator for Counter`，於是只能替 `Item` 選擇唯一一個型別。在任何地方呼叫 `Counter` 的 `next` 方法就不必再明確指定我們想要 `u32` 疊代器了。

### 預設泛型型別參數與運算子重載

我們可以透過泛型型別參數（generic type parameter）指定該泛型型別預設的具體型別。在預設型別可運作的情形下，這可省去實作者需要指定具體型別的勞動。替泛型型別指定預設型別的語法是在宣告泛型型別是寫稱 `<PlaceholderType=ConcreteType>`。

運算子重載（operator overloading）就是一個使用這個技術的好例子。*運算子重載*是指特定情況下自訂運算子（如 `+`）的行為。

Rust 不允許建立你自己的運算子或重載任意的運算子，但你可以透過實作 `std::ops` 表列出的特徵與相關的運算子，來重載特定運算與相應特徵。在範例 19-14 我們重載了 `+` 運算子，讓兩個 `Point` 實例可相加。這個功能是透過對 `Point` 結構體實作 `Add` 特徵來達成：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-14/src/main.rs}}
```

<span class="caption">範例 19-14：藉由實作 `Add` 特徵，重載 `Point` 實例的 `+` 運算子</span>

`add` 方法將兩個 `Point` 實例的 `x` 值相加，兩個 `y` 值相加，並建立新的 `Point` 實例。`Add` 特徵有個關聯型別 `Outpout` 可以決定 `add` 方法回傳的型別。

這段程式碼的預設泛型型別寫在 `Add` 特徵中，定義如下：

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

這段程式碼大體上看起來很眼熟：具有一個方法與一個關聯型別的特徵。新朋友是 `Rhs=Self`，這部分叫做*預設型別參數（default type parameter）*。`Rhs` 泛型參數（「右運算元 right hand side」的縮寫）定義了 `add` 方法中 `rhs` 參數的型別。若我們未在實作 `Add` 特徵時指定 `Rhs` 的具體型別，這個 `Rhs` 的型別預設會是 `Self`，也就是我們正在實作 `Add` 的型別。

當我們對 `Point` 實作 `Add`，因為我們想要將兩個 `Point` 實例相加，所以用到預設的 `Rhs`。讓我們看一個實作 `Add` 的範例，如何不用預設值，轉而自訂 `Rhs`。

有兩個結構體，`Millimeters` 與 `Meters`，分別儲存不同單位的值。我們想將毫米透過 `Add` 做好正確單位轉換來加至公尺，這可透過對 `Millimeters` 實作 `Add` 並將 `Rhs` 設為 `Meters` 達成，如範例 19-15。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-15/src/lib.rs}}
```

<span class="caption">範例 19-15：藉由替 `Millimeters` 實作 `Add` 特徵，使 `Millimeters` 可與 `Meters` 相加Point</span>

欲相加 `Millimeters` 與 `Meters`，先將 `Rhs` 型別參數指定為 `impl Add<Meters>`，替代預設的 `Self`。

你會在下列兩種情況下使用預設型別參數：

* 擴充一個型別但不破壞既有程式碼
* 提供大多數使用者不會需要的特殊狀況之自訂空間

標準函式庫是第二種情況的範例：通常你會將兩個相同的型別相加，但 `Add` 特徵提供超乎預設的自訂能力。`Add` 特徵定義中的預設型別參數讓我們大多數時候不需要指定額外的參數。換句話說，不用再寫部分重複的樣板，讓該特徵更易用。

第一種情況和第二種類似，但概念相反：若你想替既有特徵加上新的型別參數，可以給它一個預設值，允許擴充該特徵的功能，而不破壞既有的程式實作。

### 消除歧義的完全限定語法：呼叫同名的方法

Rust 並沒有限制不同特徵之間不能有同名的方法，也沒有阻止你對同一個型別實作這兩個特徵。有可能實作一個型別，其擁有多個從多個特徵而來的同名方法的型別。

當呼叫這些同名方法，你必須告訴 Rust 你想呼叫誰。試想範例 19-16 的程式碼，我們定義了兩個特徵 `Pilot` 與 `Wizard`，兩者都有 `fly` 方法。當我們對一個已經擁有 `fly` 方法的 `Human` 型別分別實作這兩個特徵時，每個 `fly` 方法的行為皆不同。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```
<span class="caption">範例 19-16：Human 分別實作了兩個特徵的 `fly` 方法，且 `Human` 自己實作了一個 `fly` 方法</span>

當我們對一個 `Human` 實例呼叫 `fly`，編譯器預設會呼叫直接在該型別上實作的方法，如範例 19-17 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```

<span class="caption">範例 19-17：對 `Human` 實例呼叫 `fly`</span>

執行這段程式碼會印出 `*狂揮雙臂*`，表示 Rust 呼叫直接在 `Human` 上實作的 `fly` 方法。

欲呼叫在 `Pilot` 或 `Wizard` 特徵上的 `fly` 方法，我們要用更明確的語法指定我們想要的 `fly` 方法。範例 19-18 展示了這個語法。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```

<span class="caption">範例 19-18：指定想要呼叫哪個特徵的 `fly` 方法</span>

在你要呼叫的方法名前指定特徵名稱，可以讓 Rust 清楚得知我們要呼叫哪個實作 `fly`。我們也可以寫成 `Human::fly(&person)`，同義於在範例 19-18 的 `person.fly()`，只是為了消歧義而寫得長一點罷了。

執行這段程式碼會印出：

```console
{{#include ../listings/ch19-advanced-features/listing-19-18/output.txt}}
```

因為 `fly` 方法有個 `self` 參數，所以我們若有兩個*型別*都實作了同個特徵，Rust 可以透過 `self` 的型別理出該用哪個特徵的實作。

然而，當特徵上的關聯函式（associated function）沒有 `self` 參數時，當同個作用域下的兩個型別都實作同個特徵，除非使用「完全限定語法（fully qualified syntax）」，否則 Rust 無法推斷你指涉哪個型別。舉例來說，範例 19-19 的 `Animal` 特徵有個對 `Dog` 實作 `Animal` 所得的關聯函式 `baby_name`，同時也有直接在 `Dog` 上實作的關聯函式 `baby_name`。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-19/src/main.rs}}
```

<span class="caption">範例 19-19: 一個特徵和一個型別分別擁有同名關聯函式，並且該型別實作了該特徵</span>

這段給動物庇護所的程式碼想要將所有小狗崽命名為小不點，這個行為實作在 `Dog` 的 `baby_name` 關聯函式。這個 `Dog` 型別同時實作了 `Animal` 特徵，`Animal` 特徵則描述了所有動物都有的習性。我們會在實作了 `Animal` 特徵的 `Dog` 上，透過與 `Animal` 特徵關聯的 `baby_name` 函式中，表達幼犬被稱作小狗崽這一概念。

在 `main` 中我們呼叫 `Dog::baby_name` 函式，最終會直接呼叫 `Dog` 上的關聯函式。這段程式碼會印出：

```console
{{#include ../listings/ch19-advanced-features/listing-19-19/output.txt}}
```

這個輸出結果不符合我們預期。我們想呼叫的 `baby_name` 函式應該是我們在 `Dog` 上實作的 `Animal` 特徵，所以程式碼應該印出 `幼犬被稱作小狗狗`。我們在範例 19-18 所使用的指明特徵的技巧不適用於此，如果我們更改 `main` 成範例 19-20，會得到一個編譯錯誤：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```

<span class="caption">範例 19-20：嘗試呼叫 `Animal` 特徵上的 `baby_name` 函式，但 Rust 不知道該用哪個實作</span>

因為 `Animal::baby_name` 不是方法而是關聯函式，因此沒有 `self` 參數，Rust 無法推斷出我們想要哪個 `Animal::baby_name` 實作。我們會得到這個編譯錯誤：

```console
{{#include ../listings/ch19-advanced-features/listing-19-20/output.txt}}
```

欲消除歧義，告訴 Rust 我們想用實作了 `Animal` 的 `Dog`，必須使用完全限定語法。範例 19-21 展示了如何使用完全限定語法。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```

<span class="caption">範例 19-21：使用完全限定語法指定呼叫實作了 `Animal` 的 `Dog` 上的 `baby_name` 函式</span>

我們提供一個用角括號包住的型別詮釋（type annotation），這個詮釋透過將此函式呼叫的 `Dog` 型別視為 `Animal`，來指明我們想要呼叫有實作 `Animal` 特徵的 `Dog` 上的 `baby_name` 方法。這段程式碼現在會印出我們所要的：

```console
{{#include ../listings/ch19-advanced-features/listing-19-21/output.txt}}
```

普遍來說，完全限定語法定義如下：

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

對於關聯函式來說，不會有 `receiver`，只會有其他引數的列表。你可以在任何呼叫函式或方法之處使用完全限定語法。然而，你亦可在 Rust 能透過程式其他資訊推斷出的地方省略這個語法。只需要在有多個同名實作且需要協助 Rust 指定呼叫哪個實作時，才需要使用這囉嗦冗長的語法。

### 使用超特徵要求在一個特徵內有另一特徵的功能

有些時候，你會需要一個使用到其他特徵的功能的特徵。在這種情形下，相依的特徵也需要被實作，而你依賴的這個特徵就是你正在實作的特徵的「超特徵（supertrait）」。

假設我們想要建立一個 `OutlinePrint` 特徵，它有一個 `outline_print` 方法會印出一個被星號包圍的值。換句話說，給定一個實作 `Display` 而會產生 `(x, y)` 的 `Point` 結構體，當我們對 `x` 為 `1`，`y` 為 `3` 的 `Point` 實例呼叫 `outline_print`，它印出如下：

```text
**********
*        *
* (1, 3) *
*        *
**********
```

在這個 `outline_print` 實作中，我們想要使用到 `Display` 特徵的功能。因此，我們需要指明 `OutlinePrint` 特徵只會在型別同時實作 `Display` 且提供 `OutlinePrint` 所需功能時才會成功。這件事可以在特徵定義中做到，透過指明 `OutlinePrint: Display`。這項技巧很類似特徵上的特徵約束（trait bound）。範例 19-22 展示了 `OutlinePrint` 特徵的實作。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```

<span class="caption">範例 19-22: 實作要求 `Display` 功能的 `OutlinePrint` 特徵</span>

因為我們已指明 `OutlinePrint` 需要 `Display` 特徵，且只要有實作 `Display` 的型別都會自動實作 `to_string` 這個函式，所以我們可以使用 `to_string`。若我們嘗試使用 `to_string` 但並沒有在該特徵後加上冒號並指明 `Display`，會得到一個錯誤，告訴我們在當前作用域下的 `&Self` 型別找不到名為 `to_string` 函數。

我們嘗試看看在一個沒有實作 `Display` 的型別上實作 `OutlinePrint`（如 `Point` 結構體）會發生什麼事：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

於是得到 `Display` 為必須但沒實作的錯誤：

```console
{{#include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

我們可以透過對 `Point` 實作 `Display` 並滿足 `OutlinePrint` 要求的約束（constraint），如下：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

於是對 `Point` 實作 `OutlinePrint` 特徵就成功編譯，我們即可以對 `Point` 實例呼叫 `outline_print` 來顯示一個星號外框框住它。

### 使用新型別模式替外部型別實作外部特徵

在第十章[「爲型別實作特徵」][對型別實作特徵]一節中，我們提及孤兒規則（orphah rule），這個規則指出只要型別或特徵其一是在本地的 crate 中定義，就允許我們對該型別實作該特徵。使用*新型別模式（newtype pattern）*，即可繞過這項規則，此模式涉及建立一個元組結構體（tuple struct）型別（我們在[「使用無名稱欄位的元組結構體來建立不同型別」][元組結構體]說明了元組結構體）。元組結構體包含一個欄位，在我們想要實作該特徵的型別外作一層薄薄的封裝。這封裝型別對 crate 來說算作在本地定義，因此可以對該封裝實作該特徵。*新型別*是一個源自 Haskell 程式語言的術語。使用此模式不會有任何執行時效能的耗損，這個封裝型別會在編譯期刪略。

舉個例子，我們想要對 `Vec<T>` 實作 `Display`，但孤兒規則限制我們不能這樣做，因為 `Display` 特徵與 `Vec<T>` 都是在我們的 crate 之外定義。我們可以建立一個 `Wrapper` 結構體，帶有一個 `Vec<T>` 實例，接下來再對 `Wrapper` 實作 `Display` 並使用 `Vec<T>` 之值，如範例 19-23 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-23/src/main.rs}}
```

<span class="caption">範例 19-23：建立一個 `Wrapper` 型別封裝 `Vec<STring>` 以實作 `Display`</span>

因為 `Wrapper` 是一個元組結構體而 `Vec<T>` 是該元組在索引 0 上的項目，所以該 `Display` 的實作使用 `self.0` 存取內部的 `Vec<T>`。我們就可以在 `Wrapper` 上使用 `Display` 的功能了。

使用這個技術的缺點是 `Wrapper` 是個新型別，並無它封裝的值所擁有的方法。我們不得不在 `Wapper` 上實作所有 `Vec<T>` 的方法，委派這些方法給 `self.0`，讓我們可以將 `Wrapper` 作為 `Vec<T>` 一樣對待。如果我們想要新型別得到所有內部型別擁有的所有方法，一個解法是透過對 `Wrapper` 實作 `Deref` 特徵（在第十五章[「透過 `Deref` 特徵將智慧指標視為一般引用」][智慧指標取值]一節有相應討論）並回傳內部型別。如果我們不想要 `Wrapper` 擁有所有內部型別的方法，例如限制 `Wrapper` 型別之行為，就僅須實作那些我們想要的方法。

現在，你知道如何將新型別模式與特徵相關聯，縱使不涉及特徵，新型別模式仍非常實用。接下來我們將目光轉移到其他與 Rust 型別系統互動的方法吧。

[對型別實作特徵]: ch10-02-traits.html#爲型別實作特徵
[iterator-特徵與-next-方法]: ch13-02-iterators.html#iterator-特徵與-next-方法
[特徵-定義共享行爲]: ch10-02-traits.html#特徵-定義共享行爲
[智慧指標取值]: ch15-02-deref.html#透過-deref-特徵將智慧指標視為一般引用
[元組結構體]: ch05-01-defining-structs.html#使用無名稱欄位的元組結構體來建立不同型別

> - translators: [Weihang Lo <me@weihanglo.tw>]
> - commit: [1f8e2ec](https://github.com/rust-lang/book/blob/1f8e2ec392b1d261acebda3fa9d81ea3f18c7e40/src/ch19-03-advanced-traits.md)
> - updated: 2020-09-16
