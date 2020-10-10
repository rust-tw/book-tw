## 引用與借用

我們在範例 4-5 使用元組的問題在於，我們必須回傳 `String` 給呼叫的函式，我們才能繼續在呼叫 `calculate_length` 之後繼續使用 `String`，因為 `String` 會被傳入 `calculate_length`。

以下是我們定義並使用 `calculate_length` 時，在參數改用引用物件而非取得所有權的程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

首先你會注意到原先變數宣告與函式回傳值會用到元組的地方都被更改了。再來注意到我們傳遞的是 `&s1` 給 `calculate_length`，然後在定義時我們是取 `&String` 而非 `String`。這些「&」符號就是*引用（references）*，它們允許你不必獲取所有權來引用它。以下用圖示 4-5 示意。 

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">圖示 4-5：顯示 `&String s` 指向 `String s1` 的示意圖</span>

> 注意：使用 `&` 引用的反向動作是 *解引用（dereferencing）*，使用的是解引用運算符號 `*`。我們會在第八章看到一些解引用的範例並在第 15 章詳細解釋解引用。

讓我們進一步看看函式的呼叫：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&s1` 語法讓我們可以建立一個指向 `s1` 數值的引用，但不會擁有它。因為它並沒有所有權，它所指向的資料在引用離開作用域時並不會被丟棄。

同樣地，函式簽名也是用 `&` 說明參數 `s` 是個引用。讓我們加一些註解在範例上：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

變數 `s` 有效的作用域和任何函式參數的作用域一樣，但當引用離開作用域時，我們不會丟棄任何它指向的資料，因為我們沒有所有權。當函式使用引用作為參數而非實際數值時，我們不需要回傳數值來還所有權，因為我們不曾擁有過。

我們會稱呼函式用引用作為參數叫做*借用（borrowing）*。就像現實世界一樣，如果有人擁有每項東西，他可以借用給你。當你使用完後，你就還給他。

所以要是我們嘗試修改我們借用的東西會如何呢？請試試範例 4-6 的程式碼。直接劇透你：它執行不了的！

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">範例 4-6：嘗試修改借用的值</span>

以下是錯誤訊息：

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

如同變數預設是不可變，引用也是一樣的。我們不被允許修改我們引用的值。

### 可變引用

我們可以修正這項錯誤，只要在範例 4-6 做一點小修改就好：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

首先我們將 `s` 加上了 `mut`，然後我們用 `&mut s` 建立了一個可變引用，然後以 `some_string: &mut String` 來接收這個可變引用。

但是可變引用有個很大的限制：在特定作用域中的一個特定資料只能有一個可變引用。所以以下程式碼就會失敗：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

以下是錯誤資訊：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

這項限制允許了可變行為，但是同時也受到一定程度的約束。這通常是新 Rustaceans 遭受挫折的地方，因為多數語言都會任你去改變其值。

這項限制的好處是 Rust 可以在編譯時期就防止資料競爭（data races）。*資料競爭*和競爭條件（race condition）類似，它會由以下三種行為發生：

* 同時有兩個以上的指標存取同個資料。
* 至少有一個指標在寫入資料。
* 沒有針對資料的同步存取機制。

資料競爭會造成未定義行為（undefined behavior），而且在執行時你通常是很難診斷並修正的。Rust 能夠阻止這樣的問題發生，因為它不會讓有資料競爭的程式碼編譯通過！

如往常一樣，我們可以用大括號來建立一個新的作用域來允許多個可變引用，只要不是同時擁有就好：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

類似的規則也存在於可變引用和不可變引用的組合中，以下程式碼就會產生錯誤：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

以下是錯誤訊息：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

哇！看來我們*也不可以*在擁有不可變引用的同時擁有可變引用。擁有不可變引用的使用者可不希望有人暗地裡突然改變了值！不過數個不可變引用是沒問題的，因為所有在讀取資料的人都無法影響其他人閱讀資料。

請注意引用的作用域始於它被宣告的地方，一直到它最後一次引用被使用為止。舉例來說以下程式就可以編譯，因為不可變引用最後一次的使用在可變引用宣告之前：

```rust,edition2018
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

不可變引用 `r1` 和 `r2` 的作用域在 `println!` 之後結束。這是它們最後一次使用到的地方，也就是在宣告可變引用 `r3` 之前。它們的作用域沒有重疊，所以程式碼是允許的。

雖然借用錯誤有時是令人沮喪的，但請記得這是 Rust 編譯器希望提前（在編譯時而非執行時）指出潛在程式錯誤並告訴你問題的源頭在哪。這樣你就不必親自追蹤到你為何你的資料不是你所想的那樣。

### 迷途引用

在有指標的語言中，通常都很容易不小心產生*迷途指標（dangling pointer）*。當資源已經被釋放但指標卻還留著，這樣的指標指向的地方很可能就已經被別人所有了。相反地，在 Rust 中編譯器會保證引用絕不會是迷途引用：如果你有某些資料的引用，編譯器會確保資料不會在引用結束前離開作用域。

讓我們來嘗試產生迷途指標，看看 Rust 怎麼產生編譯期錯誤：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

以下是錯誤訊息：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

此錯誤訊息包含了一個我們還沒介紹的功能：生命週期（lifetimes）。我們會在第十章詳細討論生命週期。就算我們先不管生命週期的部分，錯誤訊息仍然告訴了我們程式出錯的關鍵點：

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

讓我們進一步看看我們的 `dangle` 程式碼每一步發生了什麼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

因為 `s` 是在 `dangle` 內產生的，當 `dangle` 程式碼結束時，`s` 會被釋放。但我們卻嘗試回傳引用。此引用會指向一個已經無效的 `String`。這看起來不太優！Rust 不允許我們這麼做。

解決的辦法是直接回傳 `String` 就好：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

這樣就沒問題了。所有權轉移了出去，沒有任何值被釋放。

### 引用規則

讓我們來回顧我們討論到的引用規則：

* 在任何時候，我們要麼*只能有*一個可變引用，要麼可以有*任意數量*的不可變引用。
* 引用必須永遠有效。

接下來我們要來看看一個不太一樣的引用型別：切片（slices）。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch04-02-references-and-borrowing.md)
> - updated: 2020-09-08