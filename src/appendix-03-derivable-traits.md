## 附錄 C：可推導的特徵

在本書中的許多地方方，我們都有遇到 `derive` 屬性，這能用在結構體或枚舉定義中。`derive` 屬性會透過 `derive` 語法來對被標記的型別產生對應的預設特徵實作。

在此附錄中，我們提供了所有在標準函式庫中你可以透過 `derive` 來使用的特徵。每個段落會包含：

* 該特徵會推導出哪些運算子與方法
* `derive` 提供的特徵實作會做什麼事情
* 該特徵實作對型別有何影響
* 你能夠或不能夠實作特徵的條件
* 需要特徵來做運算的範例

如果你想要不同於 `derive` 屬性提供的行爲，請查閱每個特徵的[標準函式庫技術文件](../std/index.html)<!-- ignore -->來瞭解如何手動實作它們。

其他在標準函式庫中定義的特徵則無法使用 `derive` 來對你的型別實作。這些特徵通常不太常有理想的預設行爲，所以會由你來決定最合理的方式來實作它們。

其中一個無法推導的特徵範例就是 `Display`，這用來顯示格式化資訊給終端使用者。這應該永遠由你來決定顯示型別給終端使用者的最佳方式。型別的哪些部分應該給使用者看到？哪些部分他們會覺得是有關聯的？什麼樣的資料格式對他們最相關？Rust 編譯器並不具這樣的眼光能判斷，所以它無法爲你提供適合的預設行爲。

此附錄提供的可推導的特徵列表並不是就是所有能用的特徵：函式庫也可以爲他們自己的特徵實作 `derive`，所以你可以使用 `derive` 的特徵是沒有極限的。實作 `derive` 會需要用到過程式巨集（procedural macro），這在第十九章的[「巨集」][macros]<!-- ignore -->段落有提到。

### 用於開發時輸出的 `Debug`

`Debug` 特徵用於啟用除錯格式資訊的格式化字串，讓你可以在 `{}` 佔位符加上 `:?` 來顯示。

`Debug` 特徵讓你可以印出型別實例的除錯資訊，好讓你以及其他程式設計師在使用你的型別時，能在程式執行的特定時間點觀察實例。

舉例來說，要使用 `assert_eq!` 巨集的話就必須要有 `Debug` 特徵。如果相等判定失敗的話，此巨集會印出作爲引數的實例數值，讓程式設計師可以看到爲何兩個實例不相等。

### 用於比較相等的 `PartialEq` 與 `Eq`

`PartialEq` 特徵讓你可以比較型別實例來檢查是否相等，並可因此使用 `==` 與 `!=` 運算子。

推導 `PartialEq` 會實作 `eq` 方法。當推導結構體的 `PartialEq` 時，兩個實例之間必須*所有*欄位都相等才算相等，所以要是有任意欄位不相等實例就不算相等。而在枚舉推導時，每個變體會與自己相等，且與其他變體不相等。

舉例來說，使用 `assert_eq!` 巨集的話就必須要有 `PartialEq` 特徵，因爲這要用來比較兩個型別實例是否相等。

`Eq` 特徵沒有任何方法，它用來表示指定型別的每個數值都與自己本身相等。`Eq` 只能用於也有實作 `PartialEq` 的型別，然而並非所有實作 `PartialEq` 的型別都能實作 `Eq`。其中一個例子就是浮點數型別，浮點數的實作就指明兩個非數（not-a-number, `NaN`）實例數值彼此並不相等。

而 `Eq` 會用到的地方則有像是 `HashMap<K, V>` 中的鍵，這樣 `HashMap<K, V>` 才能知道兩個鍵之間是否相等。

### 用於比較順序的 `PartialOrd` 與 `Ord`

`PartialOrd` 特徵讓你比較型別實例排序的順序。有實作 `PartialOrd` 的型別能夠使用 `<`、`>`、`<=` 與 `>=` 運算子。你只能在有實作 `PartialEq` 的型別實作 `PartialOrd` 特徵。

推導 `PartialOrd` 會實作 `partial_cmp` 方法，這會回傳一個 `Option<Ordering>`，當給予的數值無法產生任何順序的話就會是 `None`。其中一個儘管該型別大多數數值都能比較時，但仍有機會無法產生順序的範例就是非數（not-a-number, `NaN`）浮點數數值。對任意浮點數與 `NaN` 浮點數數值呼叫 `partial_cmp` 的話就會回傳 `None`。

當在結構體推導時，`PartialOrd` compare會比較兩個實例，並從結構體定義的欄位順序來依序比較每個欄位的數值。而在枚舉推導時，在枚舉定義中較早宣告的枚舉變體會比之後的變體還小。

舉例來說，`rand` crate 中的 `gen_range` 方法就必須要有 `PartialOrd` 特徵，該方法會在指定的最小值與最大值範圍內產生隨機數值。

`Ord` 特徵能讓你知道指定型別的任意兩個數值存在著順序。`Ord` 特徵實作了 `cmp` 方法，這會回傳 `Ordering` 而不只是 `Option<Ordering>`，因爲其永遠會有有效的順序。你只能在有實作 `PartialOrd` 與 `Eq`（而 `Eq` 需要 `PartialEq`）的特徵實作 `Ord` 特徵。在結構體與枚舉推導時，`cmp` 的行爲會與 `PartialOrd` 推導的 `partial_cmp` 行爲一致。

其中一個需要 `Ord` 的範例就是當 `BTreeSet<T>` 要儲存數值的時候，這是一個依據數值排序順序儲存資料的資料結構。

### 用於複製數值的 `Clone` 與 `Copy`

`Clone` 特徵讓你能顯式建立一個數值的深拷貝，而且在複製的過程中可能會包含執行其他程式碼並拷貝堆積的資料。你可以在第四章的[「變數與資料互動的方式：克隆（Clone）」][ways-variables-and-data-interact-clone]<!-- ignore -->段落瞭解更多關於 `Clone` 的資訊。

推導 `Clone` 會實作 `clone` 方法，這在整個型別實作時，會呼叫每個型別部分的 `clone`。這意味著該型別的所有欄位或數值都必須有實作 `Clone` 才能推導 `Clone`。

會需要用到 `Clone` 的其中一個例子是對 slice 呼叫 `to_vec` 方法。Slice 不擁有其所包含的型別實例，但是 `to_vec` 回傳的 vector 會需要擁有其實例，所以 `to_vec` 會對每個項目呼叫 `clone`。因此，儲存在 slice 內的型別必須實作 `Clone`。

`Copy` 特徵讓你能只拷貝堆疊上的資料來複製數值，而且不需要額外的程式碼。你可以在第四章的[「只在堆疊上的資料：拷貝（Copy）」][stack-only-data-copy]<!-- ignore -->段落瞭解更多關於 `Copy` 的資訊。

`Copy` 特徵沒有定義任何方法，以避免開發者超載這些方法並違反不執行任何程式碼的假設。這樣所有的程式設計師才都能預定拷貝數值是很迅速的。

你可以對任何內部所有部分有實作 `Copy` 的型別推導 `Copy`。此外你只能對有實作 `Clone` 的型別實作 `Copy`，因爲有實作 `Copy` 的型別都會附有 `Clone` 的實作，其所做的事會與 `Copy` 一樣。

`Copy` 通常不是必要的，實作 `Copy` 的型別會能進行優化，代表你不需要呼叫 `clone` 並讓程式碼更簡潔。

所有 `Copy` 能辦到的事你也能用 `Clone` 來達成，但是程式碼會變得比較慢或是需要使用 `clone`。

### 用於映射數值至固定大小數值的 `Hash`

`Hash` 特徵讓你能取得任意大小的型別實例，並使用在雜湊函式映射（map）到固定大小的數值實例。推導 `Hash` 會實作 `hash` 方法。推導出的 `hash` 方法實作會組合該型別每個部分的 `hash` 呼叫結果，這代表所有的欄位或數值也必須實作 `Hash` 才能推導 `Hash`。

會需要 `Hash` 的其中一個範例是在 `HashMap<K, V>` 儲存鍵，這樣才能有效率地儲存資料。

### 用於預設數值的 `Default`

`Default` 特徵讓你能建立一個型別的預設數值。推導 `Default` 會實作 `default` 函式。推導出的 `default` 函式實作會呼叫該型別每個部分的 `default` 函式，這代表該型別的所有欄位或數值都得實作 `Default` 才能推導 `Default`。

`Default::default` 函式常用於結合結構體更新語法，如果我們在第五章的[「使用結構體更新語法從其他結構體建立實例」][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore -->段落所提及的。你可以自訂結構體中的一些欄位，然後使用 `..Default::default()` 將剩餘的欄位設爲預設數值。

舉例來說，當你在 `Option<T>` 實例中使用 `unwrap_or_default` 方法就會需要 `Default` 特徵。如果 `Option<T>` 爲 `None`，`unwrap_or_default` 方法就會回傳儲存在 `Option<T>` 的 `T` 的 `Default::default` 結果。

[creating-instances-from-other-instances-with-struct-update-syntax]:
ch05-01-defining-structs.html#使用結構體更新語法從其他結構體建立實例
[stack-only-data-copy]:
ch04-01-what-is-ownership.html#只在堆疊上的資料拷貝copy
[ways-variables-and-data-interact-clone]:
ch04-01-what-is-ownership.html#變數與資料互動的方式克隆clone
[macros]: ch19-06-macros.html#巨集

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/appendix-03-derivable-traits.md)
> - updated: 2020-09-29
