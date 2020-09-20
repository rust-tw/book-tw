## `RefCell<T>` 與內部可變性模式

*內部可變性（Interior mutability）*是 Rust 中的一種設計模式，能讓你能對即使是不可變引用的資料也能改變。正常狀況下，借用規則是不允許這種動作的。爲了改變資料，這樣的模式會在資料結構內使用 `unsafe` 程式碼來繞過 Rust 的常見可變性與借用規則。我們尚未介紹什麼是不安全（unsafe）的程式碼，我們會在第十九章談到。當編譯器無法保障，但我們可以確保借用規則在執行時能夠遵循的話，我們就可以使用擁有內部可變性模式的型別。其內的 `unsafe` 程式碼會透過安全的 API 封裝起來，讓外部型別仍然是不可變的。

讓我們觀察擁有內部可變性模式的 `RefCell<T>` 型別來探討此概念。

### 透過 `RefCell<T>` 在執行時強制檢測借用規則

不像 `Rc<T>`，`RefCell<T>` 型別的資料只會有一個所有權。所以 `RefCell<T>` 與 `Box<T>` 這種型別有何差別呢？回憶一下你在第四章學到的借用規則：

* 在任何時候，我們要麼*只能有*一個可變引用，要麼可以有*任意數量*的不可變引用。
* 引用必須永遠有效。

對於引用與 `Box<T>`，借用規則會在編譯期強制檢測。對於 `RefCell<T>`，這些規則會在*執行時*才強制執行。對於引用來說，如果你打破這些規則，你會得到編譯錯誤。而對 `RefCell<T>` 來說，如果你打破這些規則，你的程式會恐慌並離開。

在編譯時期檢查借用規則的優勢在於錯誤能在開發過程及早獲取，而且這對執行時的效能沒有任何影響，因爲所有的分析都預先完成了。基於這些原因，在編譯時檢查借用規則在大多數情形都是最佳選擇，這也是爲何這是 Rust 預設設置的原因。

在執行時檢查借用規則的優勢則在於能允許一些特定記憶體安全的場合，而這些原本是不被編譯時檢查所允許的。像 Rust 編譯器這種靜態分析本質上是保守的。有些程式碼特性是無法透過分析程式碼檢測出的，最註明的範例就是停機問題（Halting Problem），這超出本書的範疇，但是是個有趣的研究議題。

因爲有些分析是不可能的，如果 Rust 編譯器無法確定程式碼是否符合所有權規則，它可能會拒絕一支正確的程式，所以由此觀點來看能知道 Rust 編譯器是保守的。如果 Rust 接受不正確的程式，使用者就無法信任 Rust 帶來的保障。然而如果 Rust 拒絕正確的程式，對程式設計師就會很不方便，但沒有任何嚴重的災難會發生。`RefCell<T>` 型別就適用於當你確定你的程式碼有遵循借用規則，但是編譯器無法理解並保證的時候。

類似於 `Rc<T>`，`RefCell<T>` 也只能用於單一執行緒（single-threaded）的場合，所以如果你嘗試用在多執行緒上的話就會出現編譯時錯誤。我們會在第十六章討論如何在多執行緒程式擁有 `RefCell<T>` 的功能。

以下是何時選擇 `Box<T>`、`Rc<T>` 或 `RefCell<T>` 的理由:

* `Rc<T>` 讓數個擁有者能共享相同資料；`Box<T>` 與 `RefCell<T>` 只能有一個擁有者。
* `Box<T>` 能有不可變或可變的借用並在編譯時檢查；`Rc<T>` 則只能有不可變借用並在編譯時檢查：`RefCell<T>` 能有不可變或可變借用但是在執行時檢查。
* 由於 `RefCell<T>` 允許在執行時檢查可變引用，你可以改變 `RefCell<T>` 內部的數值，就算 `RefCell<T>` 是不可變的。

改變不可變數值內部的值就*內部可變性*模式。讓我們看看內部可變性何時會有用，且觀察爲何是可行的。

### 內部可變性：不可變數值的可變借用

借用規則的影響是當你有個不可變數值，你就無法取得可變引用。舉例來說，以下程式碼會無法編譯：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

如果你嘗試編譯此程式碼，你會獲得以下錯誤：

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

然而在某些特定情況，我們會想要能夠有個方法可以改變一個數值，但該數值對其他程式碼而言仍然是不可變的。數值提供的方法以外的程式碼都無法改變其值。使用 `RefCell<T>` 是取得內部可變性的方式之一。但 `RefCell<T>` 仍然要完全遵守借用規則：編譯器的借用檢查器會允許這些內部可變性，然後在執行時才檢查借用規則。如果你違反規則，你就會得到 `panic!` 而非編譯錯誤。

讓我們用一個實際例子來探討如何使用 `RefCell<T>` 來改變不可變數值，並瞭解爲何這是很實用的。

#### 內部可變性的使用案例：模擬物件

*測試替身（test double）*是一個通用程式設計概念，表示一個在測試中替代某種型別的型別。*模擬物件（Mock objects）*是測試替身其中一種特定型別，這能紀錄測試過程中發生什麼事並讓你能判斷動作是否正確。

Rust 的物件與其他語言中的物件概念並不全然相同，而且 Rust 的標準函式庫內也沒有如其他語言會內建的模擬物件功能。不過你還是可以有方法來建立結構體來作爲模擬物件。

以下是我們要測試的情境：我們建立一個函式庫來追蹤一個數值與最大值的差距，並依據該差距傳送訊息。舉例來說，此函式庫就能用來追蹤使用者允許呼叫 API 次數的上限。

我們的函式庫提供的功能這只有追蹤與最大值的距離以及何時該傳送什麼訊息。使用函式庫的應用程式要提供傳送訊息的機制，應用程式可以將訊息存在應用程式內、傳送電子郵件、傳送文字訊息或其他等等。函式庫不需要知道細節，它只需要在意會有項目實作我們提供的 `Messenger` 特徵。範例 15-20 顯示了函式庫的程式碼：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

<span class="caption">範例 15-20：追蹤某個值與最大值差距的函式庫並以此值的特定層級傳送警告</span>

此程式碼其中一個重點是 `Messenger` 特徵有個方法叫做 `send`，這會接收一個 `self` 的不可變引用與一串訊息文字。這就是我們的模擬物件所需的介面。另一個重點是我們想要測試 `LimitTracker` 中 `set_value` 方法的行爲。我們可以改變傳給參數 `value` 的值，但是 `set_value` 沒有回傳任何東西好讓我們做判斷。我們希望如果我們透過某個實作 `Messenger` 的型別與特定數值 `max` 來建立 `LimitTracker` 時，傳送訊息者能被通知要傳遞合適的訊息。

我們需要有個模擬物件，而不是在呼叫 `send` 時真的傳送電子郵件或文字訊息，我們只想紀錄訊息被通知要傳送了。我們可以建立模擬物件的實例，以此建立 `LimitTracker`、呼叫 `LimitTracker` 的 `set_value`，並檢查模擬物件有我們預期的訊息。範例 15-21 展示一個嘗試實作此事的模擬物件，但借用檢查器卻不允許：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

<span class="caption">範例 15-21：嘗試實作 `MockMessenger` 但借用檢查器不允許</span>

此測試程式碼定義了一個結構體 `MockMessenger` 其有個 `sent_messages` 欄位並存有 `String` 數值的 `Vec` 來追蹤被通知要傳送的訊息。我們也定義了一個關聯函式 `new` 讓我們可以方便建立起始訊息列表爲空的 `MockMessenger`。我們對 `MockMessenger` 實作 `Messenger` 特徵，這樣我們才能將 `MockMessenger` 交給 `LimitTracker`。在 `send` 方法的定義中，我們取得由參數傳遞的訊息，並存入 `MockMessenger` 的 `sent_messages` 列表中。

在測試中，我們測試當 `LimitTracker` 被通知將 `value` 設爲超過 `max` 數值 75% 的某個值。首先，我們建立新的 `MockMessenger`，其起始爲一個空的訊息列表。然後我們建立一個新的 `LimitTracker` 並將 `MockMessenger` 的引用與一個 `max` 爲 100 的數值賦值給它。我們用數值 80 來呼叫 `LimitTracker` 的 `set_value` 方法，此值會超過 100 的 75%。然後我們判定 `MockMessenger` 追蹤的訊息列表需要至守有一個訊息。

但是此測試有個問題，如以下所示：

```text
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

我們無法修改 `MockMessenger` 來追蹤訊息，因爲 `send` 方法取得的是 `self` 的不可變引用。而我們也無法使用錯誤訊息中推薦使用的 `&mut self`，因爲 `send` 的簽名就會與 `Messenger` 特徵所定義的不相符（你可以是看看並觀察錯誤訊息）。

這就是內部可變性能帶來幫助的場合！我們會將 `sent_messages` 存入 `RefCell<T>` 內，然後 `send` 訊息就也能夠進行修改存入訊息。範例 15-22 顯示了變更後的程式碼：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

<span class="caption">範例 15-22：在外部數值爲不可變時，使用 `RefCell<T>` 來改變內部數值</span>

`sent_messages` 欄位現在是型別 `RefCell<Vec<String>>` 而非 `Vec<String>`。在 `new` 函式中，我們用空的 vector 來建立新的 `RefCell<Vec<String>>`。

至於 `send` 方法的實作，第一個參數仍然是 `self` 的不可變借用，這就符合特徵所定義的。我們在 `self.sent_messages` 對 `RefCell<Vec<String>>` 呼叫 `borrow_mut` 來取得 `RefCell<Vec<String>>` 內的可變引用數值，也就是 vector。然後我們對 vector 的可變引用呼叫 `push` 來追蹤測試中的訊息。

最後一項改變是判定：要看到內部 vector 有多少項目的話，我們對 `RefCell<Vec<String>>` 呼叫 `borrow` 來取得 vector 的不可變引用。

現在你已經知道如何使用 `RefCell<T>`，讓我們進一步探討它如何運作的吧！

#### Keeping Track of Borrows at Runtime with `RefCell<T>`

當建立不可變與可變引用時，我們分別使用 `&` 和 `&mut` 語法。而對於 `RefCell<T>` 的話，我們使用 `borrow` 和 `borrow_mut` 方法，這是 `RefCell<T>` 所提供的安全 API 之一。`borrow` 方法回傳一個智慧指標型別 `Ref<T>`，而 `borrow_mut` 回傳智慧指標型別 `RefMut<T>`。這兩個型別都有實作 `Deref`，所以我們可以像一般引用來對待它們。

`RefCell<T>` 會追蹤當前有多少 `Ref<T>` 和 `RefMut<T>` 智慧指標存在。每次我們呼叫 `borrow` 時，`RefCell<T>` 會增加不可變借用計數。當 `Ref<T>` 離開作用域時，不可變借用計數就會減一。就和編譯時借用規則一樣，`RefCell<T>` 讓我們同一時間要麼只能有一個可變引用，要麼可以有數個不可變引用。

如果我們嘗試違法這些規則，我們不會像引用那樣得到編譯器錯誤，`RefCell<T>` 的實作會在執行時恐慌。 範例 15-23 修改了範例 15-22 的 `send` 實作。我們故意嘗試在同個作用域下建立兩個可變引用，來說明 `RefCell<T>` 會不允許我們在執行時這樣做。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

<span class="caption">範例 15-23：在同個作用域建立兩個可變引用並觀察到 `RefCell<T>` 會恐慌</span>

我們從 `borrow_mut` 回傳的 `RefMut<T>` 智慧指標來建立變數 `one_borrow`。然後我們再以相同方式建立另一個變數 `two_borrow`。這在同個作用域下產生了兩個可變引用，而這是不允許的。我們執行函式庫的測試時，範例 15-23 可以編譯通過，但是執行測試會失敗：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

注意到程式碼恐慌時的訊息 `already borrowed: BorrowMutError`。這就是 `RefCell<T>` 如何在執行時處理違反借用規則的情況。

在執行時獲取借用錯誤而不是在編譯時代表你會在開發過程之後才找到程式碼錯誤，並有可能一直到程式碼部署到生產環境後才查覺。而且你的程式碼也會多了一寫小小的執行時效能開銷，作爲在執行時而非編譯時檢查的代價。不過使用 `RefCell<T>`  讓你能在只允許有不可變數值的環境中寫出能夠變更內部追蹤訊息的模擬物件。這是想獲得 `RefCell<T>` 帶來的功能時，要與一般引用之間作出的取捨。

### 組合 `Rc<T>` 與 `RefCell<T>` 來擁有多個可變資料的擁有者

`RefCell<T>` 的常見使用方法是搭配 `Rc<T>`。回想一下 `Rc<T>` 讓你可以對數個擁有者共享相同資料，但是它只能用於不可變資料。如果你有一個 `Rc<T>` 並存有 `RefCell<T>` 的話，你就可以取得一個有數個擁有者*而且*可變的數值！

舉例來說，回憶一下範例 15-18 cons list 的範例我們使用了 `Rc<T>` 來讓數個列表可以共享另一個列表的所有權。因爲 `Rc<T>` 只能有不可變數值，我們一旦建立它們後就無法變更列表中的任何數值。讓我們加上 `RefCell<T>` 來獲得能改變列表數值的能力吧。範例 15-24 顯示了在 `Cons` 定義中使用 `RefCell<T>`，這樣一來我們就可以變更儲存在列表中的所有數值：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

<span class="caption">範例 15-24：使用 `Rc<RefCell<i32>>` 建立一個可變的 `List`</span>

我們建立了一個 `Rc<RefCell<i32>>` 實例數值並將其存入變數 `value` 好像我們之後可以直接取得。然後我們在 `a` 用持有 `value` 的 `Cons` 變體來建立 `List`。我們需要克隆 `value`，這樣 `a` 和 `value` 才能都有內部數值 `5` 的所有權，而不是從 `value` 轉移所有權給 `a`，或是讓 `a` 借用 `value`。

我們用 `Rc<T>` 封裝列表 `a`，所以當我們建立列表 `b` 和 `c` 時，它們都可以引用 `a`，就像範例 15-18 一樣。

在我們建立完列表 `a`、`b` 和 `c` 之後，我們對 `value` 的數值加上 10。我們對 `value` 呼叫 `borrow_mut`，其中使用到了我們在第五章討論過的自動解引用功能（請查閱[「`->` 運算子跑去哪了？」][wheres-the---operator]<!-- ignore -->的段落）來解引用 `Rc<T>` 成內部的 `RefCell<T>` 數值。`borrow_mut` 方法會回傳 `RefMut<T>` 智慧指標，而我們使用解引用運算子並改變其內部數值。

當我們印出 `a`、`b` 和 `c` 時，我們可以看到它們的數值都改成了 15 而非 5：

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

此技巧是不是很厲害！透過使用 `RefCell<T>`，我們可以得到一個外部是不可變的 `List` 數值，但是我們可以使用 `RefCell<T>` 提供的方法來取得其內部可變性，讓我們可以在我們想要時改變我們的資料。執行時的借用規則檢查檢查能防止資料競爭，並在某些場合犧牲一點速度來換取資料結構的彈性。

標準函式庫也提供了其他具有內部可變性的型別。像是 `Cell<T>`，這類似 `RefCell<T>` 但不同於給予內部數值的引用，`Cell<T>` 的數值會被拷貝出去。還有 `Mutex<T>` 能提供跨執行緒安全的內部可變性，我們會在第十六章討論如何使用它。歡迎查閱標準函式庫的計數文件來瞭解這些型別之間的細節差異。

[wheres-the---operator]: ch05-03-method-syntax.html#運算子跑去哪了

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch15-05-interior-mutability.md)
> - updated: 2020-09-20
