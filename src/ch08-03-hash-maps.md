## 使用雜湊映射儲存鍵值配對

我們最後一個常見的集合是*雜湊映射（hash map）*，`HashMap<K, V>` 型別會儲存一個鍵（key）型別 `K` 對應到一個數值（value）型別 `V`。它透過*雜湊函式（hashing function）*來決定要將這些鍵與值放在記憶體何處。許多程式語言都有支援這種類型的資料結構，不過通常它們會提供不同的名稱，像是 hash、map、object、hash table、dictionary 或 associative array 等等。

雜湊映射適合用於當你不想像 vector 那樣用索引搜尋資料，而是透過一個可以爲任意型別的鍵來搜尋的情況。舉例來說，在比賽中我們可以使用雜湊映射來儲存每隊的分數，每個鍵代表對與名稱，而每個值代表隊伍分數。給予一個隊伍名稱，你就能取得該隊伍分數。

我們會在此段落介紹雜湊映射的基本 API，但還有很多實用的函式定義在標準函式庫的 `HashMap<K, V>` 中，所以別忘了查閱標準函式庫的技術文件來瞭解更多資訊。

### 建立新的雜湊映射

你可以用 `new` 建立一個空的雜湊映射並用 `insert` 加入新元素。在範例 8-20 我們追蹤兩支隊伍的分數，分別爲藍隊與黃隊。藍隊初始分數有 10 分，黃隊則有 50 分。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">範例 8-20：建立新的雜湊映射並插入一些鍵值</span>

注意到我們需要先使用 `use` 將標準函式庫的 `HashMap` 集合引入。在我們介紹的三個常見集合中，此集合是最少被用到的，所以它並沒有包含在 prelude 內讓我們能自動引用。雜湊映射也沒有像前者那麼多標準函式庫提供的支援，像是內建建構它們的巨集。

和 vector 一樣，雜湊映射會將它們的資料儲存在堆積上。此 `HashMap` 得鍵是 `String` 型別而值是 `i32` 型別。和 vector 一樣，雜湊函式宣告後就都得是同類的，所有的鍵都必須是同型別，且所有的值也都必須是同型別。

另一種建構雜湊映射的方式爲使用疊代器並在一個元組組成的 vector 中使用 `collect` 方法，其中每個元組都包含一個鍵與值的配對。我們會在第十三章的[「使用疊代器來處理一系列的項目」][iterators]<!-- ignore -->段落中深入探討疊代器與它們相關的方法。`collect` 方法會將收集的資料轉換成其他集合型別，包含 `HashMap`。舉例來說，如果我們有兩個 vector 分別是隊伍名稱與隊伍分數的話，我們可以使用 `zip` 方法來產生由元組組成的 vector，其中「Blue」會與 10 配對，以此類推。然後我們就能用 `collect` 方法將元組 vector 轉換成雜湊映射，如範例 8-21 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">範例 8-21：從隊伍列表與分數列表來產生雜湊映射</span>


`HashMap<_, _>` 的型別詮釋是必要的，因爲 `collect` 可以產生不同種類的資料結構，而除非你指明不然 Rust 無法知道你要何種型別。但在指明鍵值型別的參數中，我們卻使用底線。這是因爲 Rust 可以依據 vector
的資料型別推導出雜湊映射的型別。在範例 8-21 中的鍵型別就會是 `String` 然後值的型別就會是 `i32`，如同範例 8-20 的型別一樣。

### 雜湊映射與所有權

像是 `i32` 這種有實作 `Copy` 特徵的型別其數值可以被拷貝進雜湊映射之中。但對於像是 `String` 這種擁有所有權的數值則會被移動到雜湊映射，並成爲該數值新的擁有者，如範例 8-22 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">範例 8-22：展示當鍵值插入雜湊映射後就會擁有它們</span>

我們之後就無法使用變數 `field_name` 和 `field_value`，因爲它們的值已經透過呼叫 `insert` 被移入雜湊映射之中。

如果我們插入雜湊映射的數值是引用的話，該值就不會被移動到雜湊映射之中。不過該值的引用就必須一直有效，至少直到該雜湊映射離開作用域爲止。我們會在第十章的[“使用生命週期驗證引用”][validating-references-with-lifetimes]<!-- ignore --> 段落討落更多細節。

### 取得雜湊映射的數值

我們可以透過 `get` 方法並提供鍵來取得其在雜湊映射對應的值，如範例 8-23 所示。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">範例 8-23：取得雜湊映射中藍隊的分數</span>

`score` 在此將會是對應藍隊的分數，而且結果會是 `Some(&10)`。結果是使用 `Some` 的原因是因爲 `get` 回傳的是 `Option<&V>`。如果雜湊映射中該鍵沒有對應值的話，`get` 就會回傳 `None`。所以程式會需要透過我們在第六章談到的方式處理 `Option`。

我們也可以使用 `for` 迴圈用類似的方式來遍歷雜湊映射中每個鍵值配對：

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

此程式會以任意順序印出每個配對：

```text
Yellow: 50
Blue: 10
```

### 更新雜湊映射

雖然鍵值配對的數量可以增加，但每個鍵同一時間就只能有一個對應的值而已。當你想要改變雜湊映射的資料的話，你必須決定如何處理當一個鍵已經有一個值的情況。你可以不管舊的值，直接用新值取代。你也可以保留舊值、忽略新值，只有在該鍵*尚未*擁有對應數值時才賦值給它。或者你也可以將舊值與新值組合起來。讓我們看看分別怎麼處理吧！

#### 覆蓋數值

如果我們在雜湊映射插入一個鍵值配對，然後又在相同鍵插入不同的數值的話，該鍵相對應的數值就會被取代。如範例 8-24 雖然我們呼叫了兩次 `insert`，但是雜湊映射只會保留一個鍵值配對，因爲我們向藍隊的鍵插入了兩次數值。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">範例 8-24：替換某個特定鍵對應的數值</span>

此程式碼會印出 `{"Blue": 25}`，原本的數值 `10` 會被覆蓋。

#### 只在鍵沒有值的情況下插入數值

通常檢查某個特定的鍵有沒有數值，如果沒有的話才插入數值是很常見的。雜湊映射提供了一個特別的 API 叫做 `entry` 讓你可以用想要檢查的鍵作爲參數。`entry` 方法的回傳值是一個枚舉叫做 `Entry`，它代表了一個可能存在或不存在的數值。假設我們想要檢查黃隊的鍵有沒有對應的數值。如果沒有的話，我們想插入 50。而對藍隊也一樣。使用 `entry` API 的話，程式碼會長得像範例 8-25。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">範例 8-25：使用 `entry` 方法在只有該鍵尚無任何數值時插入數值</span>

`Entry` 中的 `or_insert` 方法定義了如果 `Entry` 的鍵有對應的數值的話，就回傳該值得可變引用；如果沒有的話，那就插入參數作爲新數值，並回傳此值的可變引用。這樣的技巧比我們親自寫邏輯還來的清楚，而且更有利於借用檢查器的檢查。

執行範例 8-25 的程式碼會印出 `{"Yellow": 50, "Blue": 10}`。第一次 `entry` 的呼叫會對黃隊插入數值 50，因爲黃隊尚未有任何數值。第二次 `entry` 的呼叫則不會改變雜湊映射，因爲藍隊已經有數值 10。

#### 依據舊值更新數值

雜湊映射還有另一種常見的用法是，依照鍵的舊數值來更新它。舉例來說，範例 8-26 展示了一支如何計算一些文字內每個單字各出現多少次的程式碼。我們使用雜湊映射，鍵爲單字然後值爲我們每次追蹤計算對應單字出現多少次的次數。如果我們是第一次看到該單字的話，我們插入數值 0。

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-26/src/main.rs:here}}
```

<span class="caption">範例 8-26：使用雜湊映射儲存單字與次數來計算每個字出現的次數</span>

此程式碼會印出 `{"world": 2, "hello": 1, "wonderful": 1}`。`or_insert` 方法會回傳該鍵對應數值的可變引用（`&mut V`）。在此我們將可變引用儲存在 `count` 變數中，所以要賦值的話，我們必須先使用 `*` 來解引用（dereference）`count`。可變引用會在 `for` 結束時離開作用域，所以所有的改變都是安全的且符合借用規則。

### 雜湊函式

`HashMap` 預設是使用一種「密碼學安全（cryptographically strong）」[^siphash]的雜湊函式（hashing function），這可以抵禦阻斷服務（Denial of Service, DoS）的攻擊。這並不是最快的雜湊演算法，但爲了提升安全性唲犧牲一點效能是值得的。如果你做評測時覺得預設的雜湊函式太慢無法滿足你的需求的話，你可以指定不同的 *hasher* 來切換成其他雜湊函式。Hasher 是一個有實作 `BuildHasher` 特徵的型別。我們會在第十章討論到特徵以及如何實作它們。你不必從頭自己實作一個 hasher，[crates.io](https://crates.io/) 上有其他 Rust 使用者分享的函式庫，其中就有不少提供許多常見雜湊演算法的 hasher 實作。

[^siphash]: [https://www.131002.net/siphash/siphash.pdf](https://www.131002.net/siphash/siphash.pdf)

## 總結

當你的程式需要儲存、取得、修改資料時，vector、字串與雜湊映射可以提供大量的功能。以下是一些你應該能夠解決的練習題：

* 給予一個整數列表，請使用 vector 並回傳算數平均數、中位數（排序列表後正中間的值）以及眾數（出現最多次的值，雜湊映射在此應該會很有用）。
* 將字串轉換成 pig latin。每個單字的第一個字母爲子音的話，就將該字母移到單字後方，並加上「ay」，所以「first」會變成「irst-fay」。而單字第一個字母爲母音的話，就在單字後方加上「hay」，所以「apple」會變成「apple-hay」。請注意要考慮到 UTF-8 編碼！
* 使用雜湊映射與 vector 來建立文字介面，讓使用者能新增員工名字到公司內的一個部門。舉來來說「Add Sally to Engineering」或「Add Amir to Sales」。然後讓使用者可以索取一個部門所有的員工列表，或是依據部門用字點順序排序，取得公司內所有的員工。

標準函式庫的 API 技術文件有詳細介紹 vector、字串與雜湊映射的所有方法，這對於這些練習題應該會很有幫助！

我們現在已經開始遇到有可能會運作失敗的複雜程式了，所以接下來正是來討論錯誤處理的時候！

[iterators]: ch13-02-iterators.html
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch08-03-hash-maps.md)
> - updated: 2020-09-11
