## 引用循環會導致記憶體泄漏

Rust 的記憶體安全保障讓意外產生永遠不會清除的記憶體的情況（通稱爲*記憶體泄漏／memory leak*）難以發生，但這並不是絕對不可能。和在編譯期間不允許資料競爭的情況不同，Rust 並不保證能完全避免記憶體泄漏，代表記憶體泄漏 Rust 中被視爲是記憶體安全的。我們可以透過使用 `Rc<T>` 和 `RefCell<T>` 觀察到 Rust 仍會產生記憶體泄漏，產生引用循環是有可能的，這會導致記憶體泄漏，因爲循環中的引用計數永遠不會變成 0，所以數值永遠不會被釋放。

### 產生引用循環

讓我們看看引用循環是怎麼發生的，以及如何避免它。我們從範例 15-25 的 `List` 枚舉定義與一個 `tail` 方法開始：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

<span class="caption">範例 15-25：一個 cons list 定義並持有 `RefCell<T>`，所以我們可以修改 `Cons` 變體引用的值</span>

我們用的是範例 15-5 中 `List` 的另一種定義寫法。`Cons` 變體的第二個元素現在是 `RefCell<Rc<List>>`，代表不同於範例 15-24 那樣能修改 `i32` 數值，我們想要能修改 `Cons` 變體指向的 `List` 數值。我們也加了一個 `tail` 方法讓我們如果有 `Cons` 變體的話，能方便取得第二個項目。

在範例 15-26 我們要加入 `main` 函式並使用範例 15-25 的定義。此程式碼建立了列表 `a` 與指向列表 `a` 的列表 `b`。然後它修改了列表 `a` 來指向 `b`，因而產生循環引用。在程序過程中 `println!` 陳述式會顯示不同位置時的引用計數。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

<span class="caption">範例 15-26：透過兩個彼此指向對方的 `List` 數值來產生引用循環</span>

我們在變數 `a` 建立了一個 `Rc<List>` 實例的 `List` 數值並持有 `5, Nil` 初始列表的 。我們然後在變數  `b` 建立另一個 `Rc<List>` 實例的 `List` 數值並持有數值 10 與指向的列表 `a`。

我們將 `a` 修改爲指向 `b` 而非 `Nil` 來產生循環。我們透過使用 `tail` 方法來取得 `a` 的 `RefCell<Rc<List>>` 引用，並放入變數 `link` 中。然後我們對 `RefCell<Rc<List>>` 使用 `borrow_mut` 方法來改變 `Rc<List>` 的值，從數值 `Nil` 改成 `b` 的 `Rc<List>`。

當我們執行此程式並維持將最後一行的 `println!` 註解掉的話，我們會得到以下輸出：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

在我們變更列表 `a` 來指向 `b` 後，`a` 和 `b` 的 `Rc<List>` 實例引用計數都是 2。在 `main` 結束後，Rust 會先嘗試釋放 `b`，讓 `b` 的 `Rc<List>` 實例計數減一。

然而，因爲 `a` 仍然引用 `b` 的 `Rc<List>`，該 `Rc<List>` 的計數會是 1 而非 0，所以 `Rc<List>` 在堆積上的記憶體不會被釋放。記憶體會永遠維持在那並保持計數爲 1。爲了視覺化引用循環，我們用圖示 15-4 表示。

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">圖示 15-4：列表 `a` 與 `b` 彼此指向對方的引用循環</span>

如果你解除最後一個 `println!` 的註解並執行程式的話，Rust 會嘗試印出此循環，因爲 `a` 會指向 `b` 會指向 `a` 以此循環下去，直到堆疊溢位（stack overflow）。

在此例中，當我們建立完循環引用，程式就結束了。此循環造成的影響並不嚴重。不過要是有個更複雜的程式分配了大量的記憶體而產生循環，並維持很常一段時間的話，程式會用到比原本預期還多的記憶體，並可能壓垮系統，導致它將記憶體用光。

要產生循環引用並不是件容易的事，但也不是絕對不可能。如果你有包含 `Rc<T>` 數值的 `RefCell<T>` 數值，或是有類似具內部可變性與引用計數巢狀組合的話，你必須確保不會產生循環引用，你無法依靠 Rust 來檢查它們。產生循環引用是程式中的邏輯錯誤，你需要使用自動化測試、程式碼審查以及其他軟體開發技巧來最小化問題。

另一個避免引用循環的解決辦法是重新組織你的資料結構，確定哪些引用要有所有權，哪些引用不用。這樣一來，循環會由一些有所有權的關係與沒有所有權的關係所組成，而只有所有權關係能影響數值是否能被釋放。在範例 15-25 中。我們永遠會希望 `Cons` 變體擁有它們的列表，讓重新組織資料結構變得有可能。讓我們看看一個由父節點與子節點的圖形結構，來看看無所有權的關係何時適合用來避免循環引用。

### 避免引用循環：將 `Rc<T>` 轉換成 `Weak<T>`

目前，我們解釋過呼叫 `Rc::clone` 會增加 `Rc<T>` 實例的 `strong_count`，而 `Rc<T>` 只會在 `strong_count` 爲 0 時被清除。你也可以對 `Rc<T>` 實例呼叫 `Rc::downgrade` 並傳入 `Rc<T>` 的引用來建立*弱引用（weak reference）*。當你呼叫 `Rc::downgrade` 時，你會得到一個型別爲 `Weak<T>` 的智慧指標。不同於對 `Rc<T>` 實例的 `strong_count` 增加 1，呼叫 `Rc::downgrade` 會對 `weak_count` 增加 1。`Rc<T>` 型別使用 `weak_count` 來追蹤有多少 `Weak<T>` 的引用存在，這類似於 `strong_count`。不同的地方在於 `weak_count` 不需要歸零才能將 `Rc<T>` 清除。

強引用是你分享 `Rc<T>` 實例的方式。弱引用不會表達所有權關係。它們不會造成引用循環，因爲弱引用的循環會在其強引用計數歸零時解除。

由於 `Weak<T>` 的引用數值可能會被釋放，要對 `Weak<T>` 指向的數值做任何事情時，你都必須確保該數值還存在。你可以透過對 `Weak<T>` 實例呼叫 `upgrade` 方法，這會回傳 `Option<Rc<T>>`。如果 `Rc<T>` 數值還沒被釋放的話，你就會得到 `Some`；而如果 `Rc<T>` 數值已經被釋放的話，就會得到 `None`。因爲 `upgrade` 回傳 `Option<Rc<T>>`，Rust 會確保 `Some` 與 `None` 的分支都有處理好，所以不會取得無效指標。

爲了做示範，與其使用知道下一項的列表的例子，我們會建立一個樹狀結構，每一個項目會知道它們的子項目*以及*它們的父項目。

#### 建立樹狀資料結構：帶有子節點的 `Node`

首先我們建立一個帶有節點的樹，每個節點知道它們的子節點。我們會定義一個結構體 `Node` 來存有它自己的 `i32` 數值以及其子數值 `Node` 的引用：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

我們想要 `Node` 擁有自己的子節點，而且我們想要透過變數分享所有權，讓我們可以在樹中取得每個 `Node`。爲此我們定義 `Vec<T>` 項目作爲型別 `Rc<Node>` 的數值。我們還想要能夠修改哪些節點才是該項目的子節點，所以我們將 `children` 中的 `Vec<Rc<Node>>` 加進 `RefCell<T>`。

接著，我們使用我們定義的結構體來建立一個 `Node` 實例叫做 `leaf`，其數值爲 3 且沒有子節點；我們再建立另一個實例叫做 `branch`，其數值爲 5 且有個子節點 `leaf`。如範例 15-27 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

<span class="caption">範例 15-27：建立一個沒有子節點的 `leaf` 節點與一個有 `leaf` 作爲子節點的 `branch` 節點</span>

我們克隆 `leaf` 的 `Rc<Node>` 並存入 `branch`，代表 `leaf` 的 `Node` 現在有兩個擁有者：`leaf` 和 `branch`。我們可以透過 `branch.children` 從 `branch` 取得 `leaf`，但是從 `leaf` 無法取得 `branch`。原因是因爲 `leaf` 沒有 `branch` 的引用且不知道它們之間是有關聯的。我們想要 `leaf` 能知道 `branch` 是它的父節點。這就是我們接下來要做的事。

#### 新增從子節點到父節點的引用

要讓子節點意識到它的父節點，我們需要在我們的 `Node` 結構體定義中加個 `parent` 欄位。問題在於 `parent` 應該要是什麼型別。我們知道它不能包含 `Rc<T>`，因爲那就會造成引用循環，`leaf.parent` 就會指向 `branch` 且 `branch.children` 就會指向 `leaf`，導致同名的 `strong_count` 數值無法歸零。

讓我們換種方式思考此關係，父節點必須擁有它的子節點，如果父節點釋放的話，它的子節點也應該要被釋放。但子節點不應該擁有它的父節點，如果我們釋放子節點的話，父節點應該要還存在。這就是弱引用的使用時機！

所以與其使用 `Rc<T>`，我們使用 `Weak<T>` 來建立 `parent` 的型別，更明確的話就是 `RefCell<Weak<Node>>`。現在我們的 `Node` 結構體定義看起來會像這樣：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

節點能夠引用其父節點但不會擁有它。在範例 15-28 中我們更新了 `main` 來使用新的定義，讓 `leaf` 節點有辦法引用它的父節點 `branch`：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

<span class="caption">範例 15-28：`leaf` 節點有其父節點 `branch` 的弱引用</span>

建立 `leaf` 節點與建立範例 15-27 的 `leaf` 節點類似，只是要多加個 `parent` 欄位：`leaf` 一開始沒有任何父節點，所以我們建立一個空的 `Weak<Node>` 引用實例。

此時當我們透過 `upgrade` 方法嘗試取得 `leaf` 的父節點引用的話，我們會取得 `None` 數值。我們能在輸出結果的第一個 `println!` 陳述式看到：

```text
leaf 的父節點 None
```

當我們建立 `branch` 節點，它的 `parent` 欄位也會有個新的 `Weak<Node>` 引用，因爲 `branch` 沒有父節點。我們仍然有 `leaf` 作爲 `branch` 其中一個子節點。一旦我們有了 `branch` 的 `Node` 實例，我們可以修改 `leaf` 使其擁有父節點的 `Weak<Node>` 引用。我們對 `leaf` 中 `parent` 欄位的 `RefCell<Weak<Node>>` 使用 `borrow_mut` 方法，然後我們使用 `Rc::downgrade` 函式來從 `branch` 的 `Rc<Node>` 建立一個 `branch` 的 `Weak<Node>` 引用。

當我們再次印出 `leaf` 的父節點，這次我們就會取得 `Some` 變體其內就是 `branch`，現在 `leaf` 可以取得它的父節點了！當我們印出 `leaf`，我們也能避免產生像範例 15-26 那樣最終導致堆疊溢位（stack overflow）的循環，`Weak<Node>` 會印成 `(Weak)`：

```text
leaf 的父節點 Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

沒有無限的輸出代表此程式碼沒有產生引用循環。我們也能透過呼叫 `Rc::strong_count` 與 `Rc::weak_count` 的數值看出。

#### 視覺化 `strong_count` 與 `weak_count` 的變化

讓我們看看 `Rc<Node>` 實例中 `strong_count` 與 `weak_count` 的數值如何變化，我們建立一個新的內部作用域，並將 `branch` 的產生移入作用域中。這樣我們就能看到 `branch` 建立與離開作用域而釋放時發生了什麼事。如範例 15-29 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

<span class="caption">範例 15-29：在內部作用域建立 `branch` 並觀察強與弱引用計數</span>

在 `leaf` 建立後，它的 `Rc<Node>` 有強計數爲 1 與弱計數爲 0。在內部作用域中，我們建立了 `branch` 並與 `leaf` 做連結，此時當我們印出計數時，`branch` 的 `Rc<Node>` 會有強計數爲 1 與弱計數爲 1（因爲 `leaf.parent` 透過 `Weak<Node>` 指向 `branch`）。當我們印出 `leaf` 的計數時，我們會看到它會有強計數爲 2，因爲 `branch` 現在有個 `leaf` 的 `Rc<Node>` 克隆儲存在 `branch.children`，但弱計數仍爲 0。

當內部作用域結束時，`branch` 會離開作用域且 `Rc<Node>` 的強計數會歸零，所以它的 `Node` 就會被釋放。`leaf.parent` 的弱計數 1 與 `Node` 是否被釋放無關，所以我們沒有產生任何記憶體泄漏！

如果我們嘗試在作用域結束後取得 `leaf` 的父節點，我們會再次獲得 `None`。在程式的最後，`leaf` 的 `Rc<Node>` 強計數爲 1 且弱計數爲 0，因爲變數 `leaf` 現在是 `Rc<Node>` 唯一的引用。

所有管理計數與數值釋放都已經實作在 `Rc<T>` 與 `Weak<T>`，它們都有 `Drop` 特的實作。在 `Node` 的定義中指定子節點對父節點的關係應爲 `Weak<T>` 引用，讓你能夠將父節點與子節點彼此關聯，且不必擔心產生引用循環與記憶體泄漏。

## 總結

本章節涵蓋了如何使用智慧指標來得到一些不同於 Rust 預設引用所帶來的保障以及取捨。`Box<T>` 型別有已知大小並能將資料分配到堆積上。`Rc<T>` 型別會追蹤堆積上資料的引用數量，讓該資料能有數個擁有者。`RefCell<T>` 型別具有內部可變性，提供一個外部不可變的型別，但有方法可以改變內部數值，其會在執行時強制檢測借用規則，而非編譯時。

我們也討論了 `Deref` 與 `Drop` 特徵，這些對智慧指標提供了許多功能。我們探討了引用循環可能會導致記憶體泄漏以及如何使用 `Weak<T>` 避免它們。

如果本章節引起你的興趣，讓你想要實作你自己的智慧指標的話，歡迎查閱[「The Rustonomicon」][nomicon]來學習更多實用資訊。

接下來，我們將討論 Rust 的並行性。你還會再學到一些新的智慧指標。

[nomicon]: ../nomicon/index.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch15-06-reference-cycles.md)
> - updated: 2020-09-20
