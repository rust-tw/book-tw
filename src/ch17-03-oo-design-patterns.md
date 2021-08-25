## 實作物件導向設計模式

**狀態模式**（state pattern）是種物件導向設計模式。此模式的關鍵在於一個數值會有些內部狀態，會以**狀態物件**（state objects）呈現，然後數值的行為會依據內部狀態而有所改變。狀態物件會分享功能，當然在 Rust 中我們使用結構體與特徵，而不是使用物件與繼承。每個狀態物件負責本身的行為並監測何時要改變成其他狀態。持有狀態物件的數值不會知道狀態中不同的行為，或是何時要轉換狀態。

使用狀態模式表示當我們程式的業務需求改變時，我們不需要改變持有狀態的數值或使用其數值的程式碼。我們只需要變更其中一個狀態物件的程式碼來改變其規則，或者新增更多狀態物件。讓我們看看狀態設計模式的範例，以及如何在 Rust 中使用。

我們會漸進式地實作一個網誌文章工作流程。網誌最終的功能會長得像這樣：

1. 網誌文章從空白草稿開始。
2. 當草稿完成時，請求審核文章。
3. 當文章通過時，它就會被發佈。
4. 只有發佈的網誌文章內容會顯示出來，所以沒被通過的文章不會被意外顯示出來。

其他任何對文章的修改不會有任何影響。舉例來說，如果我們嘗試在請求審核一個文章前，通過其他網誌文章草稿的話，該文章應維持未發佈的狀態。

此範例顯示了此工作流程的程式碼形式，這是個會用到我們等等會實作的函式庫 crate `blog` API 的範例。這目前還無法編譯，因為我們還沒有實作 `blog`。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:all}}
```

<span class="caption">範例 17-11：展示我們所希望的 `blog` crate 預期行為的程式碼</span>

我們想要讓使用者能透過 `Post::new` 來建立新的網誌文章草稿，然後我們希望在草稿階段時能對網誌文章加入文字。如果我們想在通過前立即取得文章內容的話，我們什麼都不會看到，因為該文章還只是個草稿。我們加入的 `assert_eq!` 在此只是作為解釋目的。更好地做法是寫個判定網誌文章草稿是否會從 `content` 方法回傳空字串的單元測試，不過我們在此例不會寫任何測試。

接著，我們想要請求文章審核，且我們希望在等待審核時 `content` 仍是回傳空字串。當文章通過時，它就會被發佈，代表當 `content` 呼叫時，文章中的文字就會回傳。

注意到我們要使用此 crate 時只會接觸到到一個型別 `Post`。此型別會使用狀態模式，並持有個數值能包含三種狀態物件其中之一，來代表文章狀態可以是擬稿中、等待審核或已發佈。變更狀態由 `Post` 型別內部管理。狀態依據函式庫使用者對 `Post` 實例呼叫的方法而改變，但他們不用手動管理狀態的變更。而且使用者也不可能會在狀態中出錯，像是在審核前就發佈文章。

### 定義 `Post` 並在草稿階段建立新實例

讓我們開始實作出函式庫吧！我們知道我們需要一個公開的結構體 `Post` 來存有些內容，所以我們先從結構體的定義開始，它會有個公開的關聯函式 `new` 來建立 `Post` 的實例，如範例 17-12 所示。我們還會再定義一個私有的特徵 `State`。然後 `Post` 會有個私有欄位 `state` 來擁有 `Option<T>` 且其內會存有一個特徵物件 `Box<dyn State>`。你會在之後瞭解為何 `Option<T>` 在此是必要的。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-12/src/lib.rs}}
```

<span class="caption">範例 17-12：`Post` 結構體、能建立新的 `Post` 實例的 `new` 函式、`State` 特徵與 `Draft` 結構體的定義</span>

`State` 定義了不同文章狀態共享的行為，而且`Draft`、`PendingReview` 與 `Published` 狀態都會實作 `State` 特徵。目前特徵還沒有任何方法，而且我們也只先定義 `Draft` 狀態，因為這是文章的初始狀態。

當我們建立新的 `Post`，我們對其 `state` 欄位給予存有 `Box` 的 `Some` 數值。此 `Box` 會指向一個新的 `Draft` 結構體實例。這確保每當我們建立 `Post` 的新實例時，它會從草稿起始。因為 `Post` 的 `state` 欄位是私有的，我們沒有任何方法可以建立處於其他狀態的 `Post`！在 `Post::new` 函式中，我們設置 `content` 欄位為一個新的空 `String`。

### 儲存文章內容的文字

範例 17-11 展示我們想要能夠呼叫一個叫做 `add_text` 的函式並傳入 `&str` 來對網誌文章增加文字內容。我們實作此方法，而不是將 `content` 欄位透過 `pub` 公開出去。這代表我們之後可以實作個方法來控制 `content` 欄位資料該怎麼讀取。`add_text` 方法非常直觀，所以讓我們在 `impl Post` 區塊中加上範例 17-13 的實作吧：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-13/src/lib.rs:here}}
```

<span class="caption">範例 17-13：實作 `add_text` 方法來將文字加入文章的 `content` 中</span>

`add_text` 方法接收 `self` 的可變引用，因為我們在呼叫 `add_text` 時會改變 `Post` 實例。然後我們對 `content` 中的 `String` 呼叫`push_str`，並傳入 `text` 引數來存到 `content` 之中。此行為與文章的狀態無關，所以它沒有被包含在狀態模式中。`add_text` 方法不會與 `state` 欄位有關係，但它是我們想支援的部分行為之一。

### 確保文章草稿的內容為空

儘管我們已經能透過 `add_text` 來為我們的文章加些內容，但我們還是希望 `content` 方法會回傳空字串切片，因為文章還在草稿階段中，如範例 17-11 的第七行所示。現在先讓我們用能滿足需求最簡單的方式來實作 `content` 方法，也就是永遠回傳空字串切片。之後一旦我們實作出能改變文章狀態為已發佈的能力，我們會回來修改這部分。目前文章只能處於草稿階段，所以文章內容應該要永遠為空。範例 17-14 顯示了此暫時的實作方式：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-14/src/lib.rs:here}}
```

<span class="caption">範例 17-14：`Post` 暫時實作的 `content` 方法，這會永遠回傳一個空字串切片</span>

透過此 `content` 方法，範例 17-11 的程式碼到地七行都能如期執行。

### 請求文章審核來變更它的狀態

接下來，我們需要增加請求文章審核的功能，這會將其狀態從 `Draft` 變更為 `PendingReview`。如範例 14-15 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-15/src/lib.rs:here}}
```

<span class="caption">範例 17-15：對 `Post` 與 `State` 特徵實作的 `request_review` 方法</span>

`Post` 現在有個公開方法叫做 `request_review`，這會接收 `self` 的可變引用。然後我們對 `Post` 目前的狀態呼叫其內部的 `request_review` 方法，然後此 `request_review` 方法會消耗目前的狀態並回傳新的狀態。

我們對 `State` 特徵也加上了 `request_review` 方法，所有有實作此特徵的型別現在都需要實作 `request_review` 方法。注意到不同於擁有 `self`、`&self` 或 `&mut self` 來作為方法的第一個參數，我們用的是 `self: Box<Self>`。此語法代表對持有型別的 `Box` 呼叫方法才有效。此語法取得 `Box<Self>` 的所有權，將舊的狀態無效化，讓 `Post` 的狀態數值可以轉換成新的狀態。

要消耗掉舊的狀態，`request_review` 方法需要取得狀態數值的所有權。這正是 `Post` 的 `state` 欄位中使用 `Option` 的用途，我們呼叫 `take` 方法來取得 `state` 欄位中 `Some` 的數值，並留下 `None`，因為 Rust 不允許結構體的欄位為空。這讓我們將 `Post` 的 `state` 移出來，而不只是借用。然後我們會將文章 `state` 數值設為此運算的結果。

我們需要暫時將 `state` 設為 `None`，而非只是像這樣 `self.state = self.state.request_review();` 直接設置來取得 `state` 的數值。這確保 `Post` 不會在我們轉換到新狀態時，使用到舊的 `state` 數值。

`Draft` 的 `request_review` 方法需要回傳一個新的結構體 `PendingReview` box 實例，這代表文章正在等待審核的狀態。`PendingReview` 結構體也實作了 `request_review` 方法但沒有做任何轉換。反之，它只會回傳自己，因為當我們向已經處於 `PendingReview` 狀態的文章請求審核的話，它應該會維持 `PendingReview` 的狀態。

現在我們可以開始看出狀態模式的優勢了，`Post` 的 `request_review` 方法不管其 `state` 數值為何都是一樣的。每個狀態負責自己的規則。

我們維持 `Post` 的方法 `content` 不變，依然回傳一個空字串切片。我們現在的 `Post` 可以處於 `PendingReview` 狀態與 `Draft` 狀態，但我們想要 `PendingReview` 狀態也有相同的行為。現在範例 17-11 可以運行到第十行了！

### 新增改變 `content` 行為的 `approve` 方法

`approve` 方法會類似於 `request_review` 方法，它會設置 `state` 的數值為目前狀態審核通過時該處於的狀態，如範例 17-16 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-16/src/lib.rs:here}}
```

<span class="caption">範例 17-16：對 `Post` 與 `State` 特徵實作 `approve` 方法</span>

我們在 `State` 特徵加上 `approve` 方法，並新增一個也有實作 `State` 的新結構體 `Published` 特徵。

和 `request_review` 類似，如果我們對 `Draft` 呼叫 `approve` 方法，它不會有任何效果，因為它會回傳 `self`。當我們對 `PendingReview` 呼叫 `approve`，它會回傳一個新的結構體 `Published` box 實例。`Published` 也有實作 `State` 特徵，對於 `request_review` 方法與 `approve` 方法，它只會回傳自己，因為文章在這些情況下都應該維持 `Published` 狀態。

現在我們需要更新 `Post` 的 `content` 方法，如果狀態是 `Published` 的話，我們想回傳文章的 `content` 欄位；不然的話，我們仍然回傳一個空字串切片，如範例 17-17 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-17/src/lib.rs:here}}
```

<span class="caption">範例 17-17：更新 `Post` 的 `content` 方法來呼叫 `State` 的 `content` 方法</span>

因為目標是將這些所有規則維持在實作 `State` 的結構體內，我們對 `state` 呼叫 `content` 方法並傳遞文章實例（也就是 `self`）來作為引數。然後我們的回傳值就是對 `state` 數值使用 `content` 的回傳值。

我們對 `Option` 呼叫 `as_ref` 方法，因為我們希望取得 `Option` 內的數值引用，而不是該值的所有權。因為 `state` 的型別是 `Option<Box<dyn State>>`，當我們呼叫 `as_ref` 時會回傳 `Option<&Box<dyn State>>`。如果我們沒有呼叫 `as_ref` 的話，我們會得到錯誤，因為我們無法從借用的函式參數 `&self` 移動 `state`。

然後我們呼叫 `unwrap` 方法，我們知道這絕對不會恐慌，因為我們知道當 `Post` 的方法完成執行時，它們會確保 `state` 永遠包含一個 `Some` 數值。這是我們在第九章的[「當你知道的比編譯器還多的時候」][more-info-than-rustc]<!-- ignore -->段落介紹過的其中一種情況。雖然編譯器不能理解，但我們知道永遠不可能會有 `None` 數值。

此時當我們呼叫 `&Box<dyn State>` 的 `content`，強制解引用（deref coercion）對 `&` 與 `Box` 產生影響，讓 `content` 方法最終對有實作 `State` 特徵的型別呼叫。這代表我需要在 `State` 特徵定義加上 `content`，而這正是我們要填入依據狀態為何來回傳何種內容的地方，如範例 17-18 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-18/src/lib.rs:here}}
```

<span class="caption">範例 17-18：在 `State` 特徵加上 `content` 方法</span>

我們對 `content` 方法加上預設實作來回傳一個空字串切片。這代表我們不需要在 `Draft` 與 `PendingReview` 結構體實作 `content`。`Published` 結構體會覆寫 `content` 方法並回傳 `post.content` 的數值。

注意到我們在此方法需要生命週期詮釋，如我們在第十章所討論到的。我們取得 `post` 的引用作為引數並回傳 `post` 的部分引用，所以回傳引用的生命週期與 `post` 引數的生命週期有關聯。

這樣就完成了！範例 17-11 可以成功執行！我們實作了網誌文章工作流程規則的狀態模式。規則邏輯會位於狀態物件中，而不會分散在 `Post` 中。

### 狀態模式的權衡取捨

我們展示了 Rust 能夠實作出物件導向狀態模式，來封裝文章每個狀態之間不同的行為。`Post` 的方法不會知道這些不同的行為。我們組織程式碼的方式，讓我們可以只看一個地方就能知道已發佈文章會擁有的各種行為，也就是實作 `State` 特徵的 `Published` 結構體。

如果我們要建立個不使用狀態模式的替代實作，我們可能會在 `Post` 或甚至在 `main` 程式碼中改使用 `match` 表達式檢查文章狀態並變更行為。這意味著我們得查看許多地方才能知道已發佈文章狀態的含義！而且當我們增加的狀態越多，每個 `match` 表達式就需要更多分支。

透過狀態模式，`Post` 方法以及我們使用 `Post` 的地方就不需要 `match` 表達式，而且要加入新的狀態的話，我們只需要新增一個結構體並對其實作特徵方法。

使用狀態模式的實作能非常容易地擴展功能。為了觀察維護使用狀態模式的程式碼有多簡單，你可以嘗試以下一些建議：

* 新增一個 `reject` 方法讓文章狀態從 `PendingReview` 變回 `Draft`。
* 要求要呼叫兩次 `approve` 狀態才會變成 `Published`。
* 只允許使用者在 `Draft` 狀態才能新增文字內容。提示：讓狀態物件負責內容會發生什麼改變，但不負責修改 `Post`。

不過狀態模式有個劣勢，由於狀態實作狀態的轉換，有些狀態之間是彼此耦合的。如果我們在 `PendingReview` 與 `Published` 之間再加上另一個狀態像是 `Scheduled` 的話，我們就需要變更 `PendingReview` 的程式碼改轉換成
`Scheduled`。如果 `PendingReview` 不需要因為新狀態的加入做改變的話，我們可以少寫些程式碼，但這就意味著切換成其他種設計模式。

另一項劣勢是我們重複了一些邏輯。要消除掉一些重複的部分，我們可以是著對 `State` 的 `request_review` 和 `approve` 方法提供回傳 `self` 的預設實作。但是這樣就違反物件安全了，因為特徵不知道 `self` 的實際型別為何。我們想要能將 `State` 用在特徵物件中，所以它的方法必須是物件安全的。

另一個重複的部分包含 `Post` 的 `request_review` 與 `approve` 方法都以類似的方式實作。兩者均呼叫 `state` 欄位中 `Option` 內數值對應的相同方法。如果 `Post` 有很多方法都遵循這樣的模式的話，我們可以考慮定義巨集（macro）來消除重複的部分（請查閱第十九章的[「巨集」][macros]<!-- ignore -->段落）。

如其他物件導向語言所定義的來實作狀態模式，我們並沒有完全發揮出 Rust 的所有潛力。讓我們看看我們能對 `blog` crate 做些什麼改善，讓無效的狀態與轉換會產生成編譯時錯誤。

#### 定義狀態與行為成型別

我們會向你展示如何重新思考狀態模式，來達到不同的取捨效果。與其完全封裝狀態與轉換，讓外部程式碼完全看不到它們，我們會將狀態定義成不同的型別。這樣一來，Rust 的型別檢查系統就能避免在只能使用已發佈文章的地方使用到了文章草稿，並在編譯時就回傳錯誤。

讓我們先想一下範例 17-11 中 `main` 的第一個部分：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-11/src/main.rs:here}}
```

我們仍使用 `Post::new` 來建立新文章的草稿狀態以及能對文章內容新增文字的能力。但不同於在文章草稿的 `content` 方法中回傳空字串，我們這次選擇文章草稿不會實作 `content` 方法。這樣如果我們嘗試取得文章草稿內容時，我們會得到編譯錯誤告訴我們該方法不存在。如此一來，我們就不可能在生產環境意外顯示出文章草稿內容了，因為程式碼根本不會編譯過。範例 17-19 顯示了 `Post` 結構體與 `DraftPost` 結構體的定義，以及它們個別的方法：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-19/src/lib.rs}}
```

<span class="caption">範例 17-19：`Post` 有 `content` 方法，而 `DraftPost` 則沒有 `content` 方法</span>

`Post` 與 `DraftPost` 結構體都有個私有欄位 `content` 來儲存網誌文章文字。結構體不再有 `state` 欄位，因為我們將狀態的定義移到了結構體的型別中。`Post` 結構體就代表已發佈的文章，且其有個 `content` 方法來回傳 `content`。

我們仍然有 `Post::new` 函式，但是它沒有回傳 `Post` 實例，而是回傳了 `DraftPost` 的實例。因為 `content` 是私有的，而且沒有任何函式回傳 `Post`，所以目前沒有任何辦法能建立 `Post` 的實例。

`DraftPost` 結構體有個 `add_text` 方法，所以我們可以像之前一樣為 `content` 新增文字，但注意到 `DraftPost` 沒有定義 `content` 方法！所以現在程式確保所有文章都已草稿為起始，而且文章草稿不會提供顯示其內容的方法。任何想嘗試繞過此約束的方式都會產生變意錯誤。

#### 透過不同型別的轉移來實作狀態轉換

所以我們該怎麼取得發佈的文章呢？我們想要遵守執行的規則，文章草稿在審核並通過後才能夠發佈。在審核中的文章狀態應保持不顯示任何內容。讓我們新增另一個結構體 `PendingReviewPost` 來遵守這些約束吧。在 `DraftPost` 中訂一個會回傳 `PendingReviewPost` 的 `request_review` 方法，再對 `PendingReviewPost` 定義 `approve` 方法來回傳 `Post`，如範例 17-20 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-20/src/lib.rs:here}}
```

<span class="caption">範例 17-20：呼叫 `DraftPost` 的 `request_review` 來建立 `PendingReviewPost`，且其有個 `approve` 方法能將 `PendingReviewPost` 轉換成已發佈的 `Post`</span>

`request_review` 與 `approve` 方法都會取得 `self` 的所有權，因此會消耗 `DraftPost` 與 `PendingReviewPost` 實例，並分別轉換成 `PendingReviewPost` 與已發佈的 `Post`。這樣在我們呼叫 `request_review` 時，就不會有殘留的 `DraftPost` 實例，以此類推。`PendingReviewPost` 結構體也沒有定義 `content` 方法，所以嘗試讀取其內容會導致編譯錯誤，就如同 `DraftPost`。由於唯一能取得有 `content` 方法定義的已發佈 `Post` 是透過呼叫 `PendingReviewPost` 的 `approve` 方法，而唯一能取得 `PendingReviewPost` 的方法是呼叫 `DraftPost` 的 `request_review` 方法，我們現在將網誌文章工作流程寫進了型別系統中。

但我們也得對 `main` 做些小修改。`request_review` 與 `approve` 方法會回傳新的實例，而不是修改它們所呼叫的結構體，所以我們需要加些 `let post =` 來遮蔽賦值來儲存回傳的實例。我們也無法判定草稿與審核中的文章內容是否是空字串，不過我們其實也不需要它們，我們不再能編譯嘗試讀取這些狀態文章內容的程式碼了。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-21/src/main.rs}}
```

<span class="caption">範例 17-21：修改 `main` 來使用新的網誌文章工作流程實作</span>

我們修改 `main` 來重新賦值 `post` 意味著此實作不再遵循物件導向狀態模式了，狀態的轉換不再完全封裝在 `Post` 實作內部。然而我們得到的好處是的是無效狀態是不可能發生的了，這都多虧了型別系統與編譯時型別檢查！這確保了特定程式錯誤會在進入生產環境前就被察覺，像是顯示尚未發佈的文章內容。

你可以試試看在範例 17-20 之後，對 `blog` crate 實作我們在此段落稍早提及的額外需求任務建議，來看看你覺得此版本的程式碼設計如何。注意有些任務很可能在此設計就已經實作完成了。

我們看到儘管 Rust 能夠實作物件導向設計模式、其他像是將狀態寫入型別系統中的模式在 Rust 中也是可行的。這些模式有不同的取捨。雖然你可能非常熟悉物件導向模式，但重新思考問題，並善用 Rust 的特色可以帶來不少優勢，像是在執行時就避免錯誤發生。物件導向模式在 Rust 中不會永遠是最好的解決方案，因為 Rust 有像是所有權這樣物件導向語言所沒有的特定功能。

## 總結

無論你讀完此章後，認為 Rust 是否屬於物件導向語言，你都知道在 Rust 中你可以使用特徵物件來取得一些物件導向的特色。動態分配能給予你的程式碼更多的彈性，但會犧牲一點執行時效能。你可以使用此彈性來實作物件導向模式，幫助提升程式碼可維護性。Rust 還有其他像是所有權等物件導向語言所沒有的功能。物件導向模式不會永遠是善用 Rust 潛能的最佳方案，不過仍是個不錯的選項。

接下來，我們要看看模式（patterns），這是 Rust 另一個可以提供大量彈性的功能。我們在書中一路下來簡單看過它們好幾次了，不過我們還沒見識到它們全部的本事。讓我們來探索吧！

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#當你知道的比編譯器還多的時候
[macros]: ch19-06-macros.html#macros
