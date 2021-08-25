## 使用 `Box<T>` 指向堆積上的資料

最直白的智慧指標是 *box* 其型別為 `Box<T>`。Box 允許你儲存資料到堆積上，而不是堆疊。留在堆疊上的會是指向堆積資料的指標。你可以回顧第四章瞭解堆疊與堆積的差別。

Box 沒有額外的效能開銷，就只是將它們的資料儲存在堆積上而非堆疊而已。不過相對地它們也沒有多少額外功能。你大概會在這些場合用到它們：

* 當你有個型別無法在編譯時期確定大小，而你又想在需要知道確切大小的情況下使用該型別的數值
* 當你有個龐大的資料，而你想要轉移所有權並確保資料不會被拷貝。
* 當你想要擁有某個值，但你只在意該型別有實作特定的特徵，而不再是何種特定型別

我們會在[「透過 Box 建立遞迴型別」](#透過-box-建立遞迴型別)<!-- ignore -->段落解說第一種情形。而在第二種情形，轉移龐大的資料的所有權可能會很花費時間，因為在堆疊上的話會拷貝所有資料。要改善此情形，我們可以用 box 將龐大的資料儲存在堆積上。這樣就只有少量的指標資料在堆疊上被拷貝，而其引用的資料仍然保留在堆積上的同個位置。第三種情況被稱之為**特徵物件（trait object）**，第十七章會花整個[「允許不同型別數值的特徵物件」][trait-objects]<!-- ignore -->段落來討論此議題。所以你在此學到的到第十七章會再次用上！

### 使用 `Box<T>` 儲存資料到堆積上

在我們討論 `Box<T>` 的使用場合前，我們會先介紹語法以及如何對 `Box<T>` 內儲存的數值進行互動。

範例 15-1 顯示如何使用 box 在堆積上儲存一個 `i32` 數值：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

<span class="caption">範例 15-1：使用 box 在堆積上儲存一個 `i32` 數值</span>

我們定義了變數 `b` 其數值為 `Box` 分配在堆積上指向的數值 `5`。程式在此例會印出 `b = 5`，在此例中我們可以用在堆疊上相同的方式取得 box 的資料。就像任何有所有權的數值一樣，當 box 離開作用域時會釋放記憶體，在此例就是當 `b` 抵達 `main` 結尾的時候。釋放記憶體作用於 box（儲存在堆疊上）以及其所指向的資料（儲存在堆積上）。

將單一數值放在堆積上的確沒什麼用處，所以你不會對這種類型經常使用 box。在大多數情況下將像 `i32` 這種單一數值預設儲存在堆疊的確比較適合。

### 透過 Box 建立遞迴型別

在編譯時期，Rust 需要知道一個型別佔用的空間有多少。其中一種無法在編譯期間知道大小的型別就是**遞迴型別（recursive type）**，其值的一部分可以是相同型別的另一個值。由於這種巢狀數值理論上可以無限循環下去，Rust 無法知道一個遞迴型別的數值需要多大的空間。然而 box 則有已知大小，所以將 box 填入遞迴型別定義中，你就可以有遞迴型別了。

讓我們來探索 *cons list*，這是個在函式程式語言中常見的資料型別，很適合作為遞迴型別的範例。我們要定義的 cons list 型別除了遞迴的部分以外都很直白，因此這個例子的概念在往後你遇到更複雜的遞迴型別時會很實用。

#### 更多關於 Cons List 的資訊

*cons list* 是個起源於 Lisp 程式設計語言與其方言的資料結構。在 Lisp 中，`cons` 函式（「construct function」的縮寫）會從兩個引數建構一個新的配對，而這通常是一個數值與另一個配對，而這些配對就包含了列表中的配對。

cons 函式的概念在往後成了常見的函式語言術語：「將 *x* cons 到 *y*」通常代表的是建立一個新的容器實例，將元素 *x* 置於此容器的開頭，而後方則是連接到容器 *y*。

每個 cons list 的項目都包含兩個元素：目前項目的數值與下一個項目。列表中的最後一個項目只會包含一個數值叫做 `Nil`，並不會再連接下一個項目。cons list 透過遞迴呼叫 `cons` 函式來產生。表示遞迴終止條件的名稱為 `Nil`。注意這和第六章提到的「null」或「nil」的概念不全然相同，這些代表的是無效或空缺的數值。

雖然函式程式設計語言很常使用 cons lists，但在 Rust 中 cons lists 卻不是常見的資料結構。大多數當你在 Rust 需要項目列表時，`Vec<T>` 會是比較好的選擇。而其他時候夠複雜的遞迴資料型別**確實**在各種特殊情形會很實用，不過先從 cons list 開始的話，我們可以專注探討 box 如何讓我們定義遞迴資料型別。

範例 15-2 包含了 cons list 的枚舉定義。注意到此程式碼還不能編譯過，因為 `List` 型別並沒有以已知大小，我們接下來會繼續說明。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

<span class="caption">範例 15-2：第一次嘗試定義一個枚舉來代表有 `i32` 數值的 cons list 資料結構</span>

> 注意：我們定義的 cons list 只有 `i32` 數值是為了範例考量。我們當然可以使用第十章討論過的泛型來定義它，讓 cons list 定義的型別可以儲存任何型別數值。

使用 `List` 型別來儲存 `1, 2, 3` 列表的話會如範例 15-3 的程式碼所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

<span class="caption">範例 15-3：使用 `List` 枚舉儲存列表 `1, 2, 3`</span>

第一個 `Cons` 值會得到 `1` 與另一個 `List` 數值。此 `List` 數值是另一個 `Cons` 數值且持有 `2` 與另一個 `List` 數值。此 `List` 數值是另一個 `Cons` 數值且擁有 `3` 與一個 `List` 數值，其就是最後的 `Nil`，這是傳遞列表結尾訊號的非遞迴變體。

如果我們嘗試編譯範例 15-3 的程式碼，我們會得到範例 15-4 的錯誤：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

<span class="caption">範例 15-4：嘗試定義遞迴枚舉所得到的錯誤</span>

錯誤顯示此型別的「大小為無限」，原因是因為我們定義的 `List` 有個變體是遞迴：它直接存有另一個相同類型的數值。所以 Rust 無法判別出它需要多少空間才能儲存一個 `List` 的數值。讓我進一步研究為何我們會得到這樣的錯誤，首先來看 Rust 如何決定要分配多少空間來儲存非遞迴型別。

#### 計算非遞迴型別的大小

回想一下第六章中，當我們在討論枚舉定義時，我們在範例 6-2 定義的 `Message` 枚舉：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

要決定一個 `Message` 數值需要分配多少空間，Rust 會遍歷每個變體來看哪個變體需要最大的空間。Rust 會看到 `Message::Quit` 不佔任何空間、`Message::Move` 需要能夠儲存兩個 `i32` 的空間，以此類推。因為只有一個變體會被使用，一個 `Message` 數值所需的最大空間就是其最大變體的大小。

將此對應到當 Rust 嘗試檢查像是範例 15-2 的 `List` 枚舉來決定遞迴型別需要多少空間時，究竟會發生什麼事。編譯器先從查看 `Cons` 的變體開始，其存有一個 `i32` 型別與一個 `List` 型別。因此 `Cons` 需要的空間大小為 `i32` 的大小加上 `List` 的大小。為了要瞭解 `List` 型別需要的多少記憶體，編譯器在進一步看它的變體，也是從 `Cons` 變體開始。`Cons` 變體存有一個型別 `i32` 與一個型別 `List`，而這樣的過程就無限處理下去，如圖示 15-1 所示。

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">圖示 15-1：無限個 `List` 包含著無限個 `Cons` 變體</span>

#### 使用 `Box<T>` 取得已知大小的遞迴型別

Rust 無法判別出遞迴定義型別要分配多少空間，所以編譯器給了範例 15-4 的錯誤，但是此錯誤有提供實用的建議：

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ^^^^    ^
```

在此建議中，「indirection」代表與其直接儲存數值，我們可以變更資料結構，間接儲存指向數值的指標。

因為 `Box<T>` 是個指標，Rust 永遠知道 `Box<T>` 需要多少空間：指標的大小不會隨著指向的資料數量而改變。這代表我們可以將 `Box<T>` 存入 `Cons` 變體而非直接儲存另一個 `List` 數值。`Box<T>` 會指向另一個存在於堆積上的 `List` 數值而不是存在 `Cons` 變體中。概念上我們仍然有建立一個**持有**其他列表的列表，但此實作更像是將項目接著另一個項目排列，而非包含另一個在內。

我們可以改變範例 15-2 的 `List` 枚舉定義以及範例 15-3 `List` 的使用方式，將其寫入範例 15-5，這次就能夠編譯過了：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

<span class="caption">範例 15-5：使用 `Box<T>` 定義的 `List` 就有已知大小</span>

`Cons` 變體需要的大小為 `i32` 加上儲存 box 指標的空間。`Nil` 變體沒有儲存任何數值，所以它需要的空間比 `Cons` 變體少。現在我們知道任何 `List` 數值會佔的空間都是一個 `i32` 加上 box 指標的大小。透過使用 box，我們打破了無限遞迴，所以編譯器可以知道儲存一個 `List` 數值所需要的大小。圖示 15-2 顯示了 `Cons` 變體看起來的樣子。

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

<span class="caption">圖示 15-2：不再是無限大小的 `List`，因為其 `Cons` 存的是 `Box`</span>

Boxes 只提供了間接儲存與堆積分配，它們沒有其他任何特殊功能，比如我們等下就會看到的其他智慧指標型別。它們也沒有任何因這些特殊功能產生的額外效能開銷，所以它們很適合用於像是 cons list 這種我們只需要間接儲存的場合。我們在第十七章還會再介紹到更多 box 的使用情境。

`Box<T>` 型別是智慧指標是因為它有實作 `Deref` 特徵，讓 `Box<T>` 的數值可以被視為引用所使用。當 `Box<T>` 數值離開作用域時，該 box 指向的堆積資料也會被清除，因為其有 `Drop` 特徵實作。讓我們來探討這兩種特徵的細節吧。這兩種特徵對於本章將會討論的其他智慧指標型別所提供的功能，將會更加重要。

[trait-objects]: ch17-02-trait-objects.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch15-01-box.md)
> - updated: 2020-09-19
