## 透過 `Drop` 特徵執行清除程式碼

第二個對智慧指標模式很重要的特徵是 `Drop`，這讓你能自訂數值離開作用域時的行爲。你可以對任何型別實作 `Drop` 特徵，然後你指定的程式碼就能用來釋放像是檔案或網路連線等資源。我們在智慧指標的章節介紹 `Drop` 的原因是因爲 `Drop` 特徵的功能幾乎永遠會在實作智慧指標時用到。舉例來說，當 `Box<T>` 離開作用域時，它會釋放該 box 在堆積上指向的記憶體空間。

在某些語言中，當程式設計師使用完智慧指標的實例後，每次都得呼叫釋放記憶體與資源的程式碼。如果他們忘記的話，系統可能就會過載並崩潰。在 Rust 中你可以對數值離開作用域時指定一些程式碼，然後編譯器就會自動插入此程式碼。所以你就不用每次在特定型別實例使用完時，在程式的每個地方都寫上清理程式碼。而且你還不會泄漏資源！

透過實作 `Drop` 特徵我們可以指定當數值離開作用域時要執行的程式碼。`Drop` 特徵會要求我們實作一個方法叫做 `drop`，這會取得 `self` 的可變引用。爲了觀察 Rust 何時會呼叫 `drop`，讓我們先用 `println!` 陳述式實作 `drop`。

範例 15-14 的結構體 `CustomSmartPointer` 只有一個功能那就是在實例離開作用域時印出 `Dropping CustomSmartPointer!`。此範例能夠展示 Rust 何時會執行 `drop` 函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

<span class="caption">範例 15-14：`CustomSmartPointer` 結構體實作了會放置清理程式碼的 `Drop` 特徵</span>

`Drop` 特徵包含在 prelude 中，所以我們不需要特地引入作用域。我們對 `CustomSmartPointer` 實作 `Drop` 特徵並提供會呼叫 `println!` 的 `drop` 方法實作。`drop` 的函式本體用來放置你想要在型別實例離開作用域時執行的邏輯。我們在此印出一些文字來展示 Rust 如何呼叫 `drop`。

在 `main` 中，我們建立了兩個 `CustomSmartPointer` 實例並印出 `CustomSmartPointers 建立完畢`。在 `main` 結尾，我們的 `CustomSmartPointer` 實例會離開作用域，然後 Rust 就會呼叫我們放在 `drop` 方法的程式碼，也就是印出我們的最終訊息。注意到我們不需要顯式呼叫 `drop` 方法。

當我們執行此程式時，我們會看到以下輸出：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

當我們的實例離開作用域時，Rust 會自動呼叫 `drop`，呼叫我們指定的程式碼。變數會以與建立時相反的順序被釋放，所以 `d` 會在 `c` 之前被釋放。此範例給了我們一個觀察 `drop` 如何執行的視覺化指引，通常你會指定該型別所需的清除程式碼，而不是印出訊息。

### 透過 `std::mem::drop` 提早釋放數值

不幸的是，我們無法值接了當地取消自動 `drop` 的功能。停用 `drop` 通常是不必要的，整個 `Drop` 的目的本來就是要能自動處理。不過有些時候你可能會想要提早清除數值。其中一個例子是使用智慧指標來管理鎖：你可能會想要強制呼叫 `drop` 方法來釋放鎖，好讓作用域中的其他程式碼可以取得該鎖。Rust 不會讓你手動呼叫 `Drop` 特徵的 `drop` 方法。不過如果你想要一個數值在離開作用域前就被釋放的話，你可以使用標準函式庫提供的 `std::mem::drop` 函式來呼叫。

如果我們嘗試修改範例 15-14 的 `main` 函式來手動呼叫 `Drop` 特徵的 `drop` 方法，如範例 15-15 所示，我們會得到編譯錯誤：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

<span class="caption">範例 15-15：嘗試呼叫 `Drop` 特徵的 `drop` 方法來手動提早清除</span>

當我們嘗試編譯此程式碼，我們會獲得以下錯誤：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

此錯誤訊息表示我們不允許顯式呼叫 `drop`。錯誤訊息使用了一個術語*解構子（destructor）*，這是通用程式設計術語中表達會清除實例的函式。*解構子*對應的術語就是*建構子（constructor）*，這會建立實例。Rust 中的 `drop` 函式就是一種特定的解構子。

Rust 不讓我們顯式呼叫 `drop`，因爲 Rust 還是會在 `main` 結束時自動呼叫 `drop`。這樣可能會導致*重複釋放（double free）* 的錯誤，因爲 Rust 可能會嘗試清除相同的數值兩次。

當數值離開作用域時我們無法停用自動插入的 `drop`，而且我們無法顯式呼叫 `drop` 方法，所以如果我必須強制讓一個數值提早清除的話，我們可以用 `std::mem::drop` 函式。

`std::mem::drop` 函式不同於 `Drop` 中的 `drop` 方法，我們將我們想要強制提早釋放的數值作爲引數傳遞並呼叫它。此函式也包含在 prelude，所以我們可以修改範例 15-15 的 `main` 來呼叫 `drop` 函式，如範例 15-16 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

<span class="caption">範例 15-16：在數值離開作用域前呼叫 `std::mem::drop` 來顯示釋放數值</span>

執行此程式會印出以下結果：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

```釋放 CustomSmartPointer 的資料 `某些資料`!``` 這段文字會在 `CustomSmartPointer 建立完畢。` 與 `CustomSmartPointer 在 main 結束前就被釋放了。` 文字之間印出，顯示  `drop` 方法會在那時釋放 `c`。

你可以在許多地方使用 `Drop` 特徵實作所指定的程式碼，讓清除實例變得方便又安全。舉例來說，你可以用它來建立你自己的記憶體分配器！透過 `Drop` 特徵與 Rust 的所有權系統，你不必去擔心要記得清理，因爲 Rust 會自動處理。

你也不必擔心會意外清理仍在使用的數值：所有權系統會確保所有引用永遠有效，並確保當數值不再需要使用時只會呼叫 `drop` 一次。

現在你看過 `Box<T>` 以及一些智慧指標的特性了，讓我們來看看一些其他定義在標準函式庫的智慧指標吧。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch15-03-drop.md)
> - updated: 2020-09-19
