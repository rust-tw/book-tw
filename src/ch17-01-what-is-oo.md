## 物件導向語言的特色

對於一個被視爲物件導向的語言該有哪些功能，在程式設計語言社群中並沒有達成共識。Rust 受到許多程式設計理念影響，這當然包括 OOP。舉例來說，我們在第十三章探討了源自於函式語言的特性。同樣地，OOP 語言有一些特定常見特色，諸如物件、封裝（encapsulation）與繼承（inheritance）。讓我們看看這些特色分別是什麼意思以及 Rust 有沒有支援。

### 物件包含資料與行爲

由 Erich Gamma、Richard Helm、Ralph Johnson 與 John Vlissides（Addison-Wesley Professional, 1994) 所寫的書 *Design Patterns: Elements of Reusable Object-Oriented Software* 俗稱爲 *The Gang of Four*，這是本物件導向設計模式的目錄。它是這樣定義 OOP 的：

> 物件導向程式由物件所組成。*物件*會包裝資料以及運作在資料上的行爲。此行爲常稱爲*方法（methods）*或*操作（operations）*。

在此定義下，Rust 是物件導向的，結構體與枚舉擁有資料，而 `impl` 區塊對結構體與枚舉提供方法。就算有方法的結構體與枚舉不會被稱爲*物件*，依據 Gang of Four 對物件的定義，它們還是有提供相同的功能。

### 隱藏實作細節的封裝

另外一個常和 OOP 相關的概念就是*封裝（encapsulation）*，這指的是物件的實作細節不會讓使用物件的程式碼取得。因此要與該物件互動的方式是透過它的公開 API，使用物件的程式碼不該有辦法觸及物件內部並直接變更資料的行爲。這讓程式設計師能變更並重構物件內部，無需擔心要變更使用物件的程式碼。

我們在第七章討論過如何控制封裝，我們可以使用 `pub` 關鍵字來決定程式中的哪些模組、型別、函式與方法要公開出來，且預設一切都是私有的。舉例來說，我們可以定義個結構體 `AveragedCollection` 並有個欄位包含一個 `i32` 數值的向量。此結構體還有個欄位包含向量數值的平均值，讓我們不必在每次呼叫時都得重新計算平均值。換句話說，`AveragedCollection` 會爲我們快取計算出的平均值。範例 17-1 展示了結構體 `AveragedCollection` 的定義：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-01/src/lib.rs}}
```

<span class="caption">範例 17-1：結構體 `AveragedCollection` 有個整數列表與集合中的項目平均值
collection</span>

此結構體有 `pub` 標記所以其他程式碼可以使用它，但結構體內部的欄位是私有的。這在此例中是很重要的，因爲我們希望在有數值加入或移出列表時，平均值也能更新。我們會實作結構體的 `add`、`remove` 與 `average` 方法來達成，如範例 17-2 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-02/src/lib.rs:here}}
```

<span class="caption">範例 17-2：對 `AveragedCollection` 實作公開方法 `add`、`remove` 與 `average`</span>

公開的方法 `add`、`remove` 與 `average` 是存取或修改 `AveragedCollection` 實例資料的唯一方法。當有個項目透過 `add` 方法加入或透過 `remove` 方法移出 `list` 中時，每個方法會同時呼叫 `update_average` 方法來更新 `average` 欄位。

我們讓 `list` 與 `average` 欄位維持私有，所以外部的程式碼不可能直接新增或移除 `list` 欄位的項目。不然的話，`average` 欄位可能就無法與變更的 `list` 同步了。`average` 方法會回傳 `average` 欄位的數值，讓外部程式碼能夠讀取 `average` 但不會修改它。

由於我們封裝了 `AveragedCollection` 結構體的實作細節，我們可以在未來輕鬆變更像是資料結構等內部細節。舉例來說，我們可以用 `HashSet<i32>` 來替換 `list` 欄位的 `Vec<i32>`。只要 `add`、`remove` 與 `average` 的公開方法簽名維持一樣，使用到 `AveragedCollection` 的程式碼就不需要改變。如果我們讓 `list` 公開的話，情況可能就不相同了，`HashSet<i32>` 與 `Vec<i32>` 有不同的方法來新增和移除項目，所以外部的程式碼如果會直接修改 `list` 的話，可能會需要做些改變。

如果封裝是物件導向的必備條件的話，Rust 也符合此條件。對程式碼中不同部分使用 `pub` 可以封裝實作細節。

### 作爲型別系統與程式碼共享來繼承

*繼承（Inheritance）* 是指一個物件可以繼承其他物件定義的機制，使其可以獲取繼承物件的資料與行爲，不必再定義一次。

如果一個語言一定要有繼承才算物件導向語言的話，那麼 Rust 就不是。在定義結構體時我們無法繼承父結構體欄位的方法實作。然而如果你在程式設計時常常用到繼承的話，依據你想使用繼承的原因，Rust 還是有提供其他方案。

你選擇繼承通常會有兩個主要原因。第一個是想能重複使用程式碼，你可以定義一個型別的特定行爲，然後繼承讓你可以在不同的型別重複使用該實作。爲此你可以使用預設的特徵方法實作來分享 Rust 程式碼，你在範例 10-14 就有看到我們在 `Summary` 特徵加上的預設 `summarize` 方法實作。任何有實作 `Summary` 特徵的型別都不必加上更多程式碼就能有 `summarize` 可以呼叫。這就類似於父類型（class）實作的方法可以在繼承的子類型擁有該方法實作。我們也可以在實作 `Summary` 特徵時，覆寫 `summarize` 方法的預設實作，這就類似於子類型覆寫父類型的方法實作。

另一個想使用繼承的原因與型別系統有關，讓子類型可以視爲父類型來使用。這也稱爲*多型（polymorphism）*，代表要是數個物件有共享特定特性的話，你可以在執行時彼此替換使用。

> ### 多型
>
> 對許多人來說，多型就是繼承的代名詞。不過這其實是個更通用的概念，用來指程式碼可適用於多種型別資料。而對繼承來說，這些型別通常都是子類型。
>
> Rust 則是使用泛型來抽象化不同可能的型別，並以特徵界限來加強約束這些型別必須提供的內容。這有時會稱爲*限定的參數多型（bounded parametric polymorphism）*。

近年來像繼承這種程式設計的解決方案在許多程式設計語言中都漸漸失寵了，因爲這經常有分享不必要程式碼的風險。子類型不應該永遠分享其父類型的所有特性，但繼承會這樣做。這會讓程式的設計較不具有彈性。這還可能產生不具意義或導致錯誤的子類型方法呼叫，因爲該方法不適用於子類型。除此之外，有些語言只會允許子類型繼承一個類型，進一步限制了程式設計的彈性。

基於這些原因，Rust 採取了不同的方案，使用特徵物件（trait objects）而非繼承。讓我們看看 Rust 的特徵物件如何達成多型。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch17-01-what-is-oo.md)
> - updated: 2020-09-23
