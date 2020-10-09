## 使用疊代器來處理一系列的項目

疊代器（Iterator）模式讓你可以對 一個項目序列依序進行某些任務。一個疊代器負責遍歷序每個項目以及序列何時結束的邏輯。當你使用疊代器，你不需要自己實作這些邏輯。

在 Rust 中疊代器是*惰性（lazy）* 的，代表除非你呼叫方法來使用疊代器，不然它們不會有任何效果。舉例來說，範例 13-13 的程式碼會透過 `Vec<T>` 定義的方法 `iter` 從向量`v1` 建立一個疊代器來遍歷它的項目。此程式碼本身沒有啥實用之處。

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/main.rs:here}}
```

<span class="caption">範例 13-13：建立一個疊代器</span>

一旦我們建立了疊代器，我們可以有很多使用它的方式。在第三章的範例 3-5 中，我們在 `for` 迴圈中使用疊代器來對每個項目執行一些程式碼，雖然我們當時沒有詳細解釋 `iter` 是在做什麼。

範例 13-14 區隔了疊代器的建立與使用疊代器 `for` 迴圈。疊代器儲存在變數 `v1_iter`，且在此時沒有任何遍歷的動作發生。當使用 `v1_iter` 疊代器的 `for` 迴圈被呼叫時，疊代器中的每個元素才會在迴圈中每次疊代中使用，以此印出每個數值。

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

<span class="caption">範例 13-14：在 `for` 迴圈使用疊代器</span>

在標準函式庫沒有提供疊代器的語言中，你可能會用別種方式寫這個相同的函式，像是先從一個變數 0 作爲索引開始、使用該變數索引向量來獲取數值，然後在迴圈中增加變數的值直到它抵達向量的總長。

疊代器會爲你處理這些所有邏輯，減少重複且你可能會搞砸的程式碼。疊代器還能讓你靈活地將相同的邏輯用於不同的序列，而不只是像向量這種你能進行索引的資料結構。讓我們研究看看疊代器怎麼辦到的。

### `Iterator` 特徵與 `next` 方法

所有的疊代器都會實作定義在標準函式庫的 `Iterator` 特徵。特徵的定義如以下所示：

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 以下省略預設實作
}
```

注意到此定義使用了一些新的語法：`type Item` 與 `Self::Item`，這是此特徵定義的*關聯型別（associated type）*。我們會在第十九章進一步探討關聯型別。現在你只需要知道此程式碼表示要實作 `Iterator` 特徵的話，你還需要定義 `Item` 型別，而此 `Item` 型別會用在方法 `next` 的回傳型別中。換句話說，`Item` 型別會是從疊代器回傳的型別。

`Iterator` 型別只要求實作者定義一個方法：`next` 方法會用 `Some` 依序回傳疊代器中的每個項目，並在疊代器結束時回傳 `None`。

我們可以直接在疊代器呼叫 `next` 方法。範例 13-15 展示從向量建立的疊代器重複呼叫 `next` 每次會得到什麼數值。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/lib.rs:here}}
```

<span class="caption">範例 13-15：對疊代器呼叫 `next` 方法</span>

注意到 `v1_iter` 需要是可變的：在疊代器上呼叫 `next` 方法會改變疊代器內部用來紀錄序列位置的狀態。換句話說，此程式碼*消耗*或者說使用了疊代器。每次 `next` 的呼叫會從疊代器消耗一個項目。而我們不必在 `for` 迴圈指定 `v1_iter` 爲可變是因爲迴圈會取得 `v1_iter` 的所有權並在內部將其改爲可變。

另外還要注意的是我們從 `next` 呼叫取得的是向量中數值的不可變引用。`iter` 方法會從疊代器中產生不可變引用。如果我們想要一個取得 `v1` 所有權的疊代器，我們可以呼叫 `into_iter` 而非 `iter`。同樣地，如果我們想要遍歷可變引用，我們可以呼叫 `iter_mut` 而非 `iter`。

### 消耗疊代器的方法

標準函式庫提供的 `Iterator` 特徵有一些不同的預設實作方法，你可以查閱標準函式庫的 `Iterator` 特徵 API 技術文件來找到這些方法。其中有些方法就是在它們的定義呼叫 `next` 方法，這就是爲何當你實作 `Iterator` 特徵時需要提供 `next` 方法的實作。

會呼叫 `next` 的方法被稱之爲*消耗配接器（consuming adaptors）*，因爲呼叫它們會使用掉疊代器。其中一個例子就是方法 `sum`，這會取得疊代器的所有權並重複呼叫 `next` 來遍歷所有項目，因而消耗掉疊代器。隨著遍歷的過程中，他會將每個項目加到總計中，並在疊代完成時回傳總計數值。範例 13-16 展示了一個使用 `sum` 方法的測試：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs:here}}
```

<span class="caption">範例 13-16：呼叫 `sum` 方法來取得疊代器中所有項目的總計數值</span>

我們在呼叫 `sum` 之後就不允許使用 `v1_iter` 因爲 `sum` 取得了疊代器的所有權。

### 產生其他疊代器的方法

而其他定義在 `Iterator` 特徵的方法則叫做*疊代配接器（iterator adaptors）*，它們能讓你變更疊代器成其他種類的疊代器。你可以串接數個疊代配接器的呼叫來組織一系列複雜的動作並仍能保持閱讀性。不過因爲所有的疊代器都是惰性的，你需要呼叫一個消耗配接器方法來取得疊代配接器呼叫的結果。

範例 13-17 呼叫了疊代器的疊代配接器方法 `map`，這可以取得一個閉包來對每個項目進行處理以產生一個新的疊代器。閉包會將向量中的每個項目加 1 來產生新的疊代器。不過此程式碼會產生一個警告：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-17/src/main.rs:here}}
```

<span class="caption">範例 13-17：呼叫疊代配接器 `map` 來建立新的疊代器</span>

我們獲得的警告如以下所示：

```console
{{#include ../listings/ch13-functional-features/listing-13-17/output.txt}}
```

範例 13-17 的程式碼不會做任何事情，我們指定的閉包沒有被呼叫到半次。警告提醒了我們原因：疊代配接器是惰性的，我們必須在此消耗疊代器才行。

要修正並消耗此疊代器，我們將使用 `collect` 方法，這是我們在範例 12-1 搭配 `env::args` 使用的方法。此方法會消耗疊代器並收集結果數值至一個資料型別集合。

在範例 13-18 中，我們將遍歷 `map` 呼叫所產生的疊代器結果數值收集到一個向量中。此向量最後會包含原本向量每個項目都加 1 的數值。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<span class="caption">範例 13-18：呼叫方法 `map` 來建立新的疊代器並呼叫 `collect` 方法來消耗新的疊代器來產生向量</span>

因爲 `map` 接受一個閉包，我們可以對每個項目指定任何我們想做的動作。這是一個展示如何使用閉包來自訂行爲，同時又能重複使用 `Iterator` 特徵提供的遍歷行爲的絕佳例子。

### 使用閉包獲取它們的環境

現在我們介紹了疊代器，我們可以展示一個透過使用 `filter` 疊代配接器與閉包獲取它們環境的常見範例。疊代器中的 `filter` 方法會接受一個使用疊代器的每個項目並回傳布林值的閉包。如果閉包回傳 `true`，該數值就會被包含在 `filter` 產生的疊代器中；如果閉包回傳 `false`，該數值就保不會被包含在結果疊代器中。

在範例 13-19 中我們使用 `filter` 與一個從它的環境獲取變數 `shoe_size` 的閉包來遍歷一個有 `Shoe` 結構體實例的集合。它會回傳只有符合指定大小的鞋子：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs}}
```

<span class="caption">範例 13-19：使用 `filter` 方法與一個獲取 `shoe_size` 的閉包</span>

函式 `shoes_in_my_size` 會取得鞋子向量的所有權以及一個鞋子大小作爲參數。它會回傳只有符合指定大小的鞋子向量。

在 `shoes_in_my_size` 的本體中，我們呼叫 `into_iter` 來建立一個會取得向量所有權的疊代器。然後我們呼叫 `filter` 來將該疊代器轉換成只包含閉包回傳爲 `true` 的元素的新疊代器。

閉包會從環境獲取 `shoe_size` 參數並比較每個鞋子數值的大小，讓只有符合大小的鞋子保留下來。最後呼叫 `collect` 來收集疊代器回傳的數值進一個函式會回傳的向量。

此測試顯示了當我們呼叫 `shoes_in_my_size` 時，我們會得到我們指定相同大小的鞋子。

### 透過 `Iterator` 特徵建立我們自己的疊代器

我們已經顯示了你可以對向量呼叫 `iter`、`into_iter` 或 `iter_mut` 來建立疊代器。你也可以從標準函式庫的其他集合型別產生疊代器，像是雜湊映射等等。你也可以透過對你自己的型別實作 `Iterator` 特徵來建立任何你所希望的疊代器。如同之前提到的，你唯一需要提供的方法定義就是 `next` 方法。一旦你完成，你就可以使用 `Iterator` 特徵提供的所有預設實作方法！

作爲展示，讓我們建立一個只會從 1 數到 5 的疊代器。首先，我們要建立個擁有一些數值的結構體。然後我們對此結構體實作 `Iterator` 特徵將它變成一個疊代器，並在實作中使用其值。

範例 13-20 有個結構體 `Counter` 的定義以及能夠產生 `Counter` 實例的關聯函式 `new`：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs}}
```

<span class="caption">範例 13-20：定義結構體 `Counter` 與關聯函式 `new`，這能建立一個初始值 `count` 爲 0 的 `Counter` 結構體</span>

`Counter` 結構體只有一個欄位 `count`，此欄位擁有一個 `u32` 數值來追蹤我們遍歷 1 到 5 的當前位置。`count` 欄位是私有的，因爲我們希望 `Counter` 的實作會管理此數值。函式 `new` 強制建立新實例的行爲永遠會從 `count` 欄位爲 0 時開始。

接下來我們對我們的 `Counter` 型別實作 `Iterator` 特徵，定義 `next` 方法本體來指定疊代器的使用行爲，如範例 13-21 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-21/src/lib.rs:here}}
```

<span class="caption">範例 13-21：在我們的 `Counter` 結構體實作 `Iterator` 特徵</span>

我們將疊代器的關聯型別 `Item` 設爲 `u32`，代表疊代器將會回傳 `u32` 數值。一樣先別擔心關聯型別，我們會在第十九章討論到。

我們希望我們的疊代器對目前的狀態加 1，所以我們將 `count` 初始化爲 0，這樣它就會先回傳 1。如果 `count` 的值小於 5，`next` 就會增加 `count` 的值並用 `Some` 回傳目前數值。一旦 `count` 等於 5，我們的疊代器就會停止增加 `count` 並永遠回傳傳 `None`。

#### 使用 `Counter` 疊代器的 `next` 方法

一旦我們實作了 `Iterator` 特徵，我們就有一個疊代器了！範例 13-22 的測試展示我們可以對我們的 `Counter` 結構體直接呼叫 `next` 方法來使用疊代器的功能，就像我們在範例 13-15 對向量建立的疊代器使用的方式一樣。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<span class="caption">範例 13-22：測試 `next` 方法實作的功能</span>

此測試建立了一個新的 `Counter` 實例給變數 `counter` 並重複呼叫 `next`，驗證我們實作的疊代器是否行爲如我們預期的一樣：回傳數值 1 到 5。


#### 使用其他 `Iterator` 特徵方法

我們透過定義 `next` 方法來實作 `Iterator` 特徵，所以我們現在可使用在標準函式庫提供的 `Iterator` 特徵中所任何有預設實作的方法了，因爲它們都使用到了 `next` 的方法功能。

舉例來說，如果我們因爲某些原因想要取得一個 `Counter` 實例的數值與另一個 `Counter` 實例去掉第一個值的數值來做配對、對每個配對相乘、保留結果可以被 3 整除的值，最後將所有結果數值相加，我們可以這樣寫，如範例 13-23 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-23/src/lib.rs:here}}
```

<span class="caption">範例 13-23：對我們的 `Counter` 疊代器使用各式各樣的 `Iterator` 特徵方法</span>

注意到 `zip` 只會產生四個配對，理論上的 `(5, None)` 配對是不會產生出來的，因爲 `zip` 會在它的其中一個輸入疊代器回傳 `None` 時就回傳 `None`。

這些所有呼叫都是可行的，因爲我們已經定義了 `next` 運作的行爲，而標準函式庫會提供其他呼叫 `next` 方法的預設實作。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch13-02-iterators.md)
> - updated: 2020-09-18
