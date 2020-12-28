## 使用訊息傳遞在執行緒間傳送資料

有一種確保安全並行且漸漸流行起來的方式是**訊息傳遞（message passing）**，執行緒或 actors 透過傳遞包含資料的訊息給彼此來溝通。此理念源自於 [Go 語言技術文件](https://golang.org/doc/effective_go.html#concurrency)中的口號：「別透過共享記憶體來溝通，而是透過溝通來共享記憶體。」 

Rust 其中一個達成訊息傳遞並行的主要工具是**通道（channel）**，這是 Rust 標準函式庫有提供的程式設計概念。你可以把程式設計的通道想像成水流的通道，像是河流或小溪。如果你將橡皮小鴨或船隻放入河流中，它會順流而下到下游。

程式設計中的通道有兩個部分：發送者（transmitter）與接收者（receiver）。發送者正是你會放置橡皮小鴨到河流中的上游，而接收者則是橡皮小鴨最後漂流到的下游。你程式碼中的一部分會呼叫發送者的方法來傳送你想要傳遞的資料，然後另一部分的程式碼會檢查接收者收到的訊息。當發送者或接收者有一方被釋放掉時，該通道就會被**關閉**。

我們在此將寫一支程式，它會在一個執行緒中產生數值，傳送給通道，然後另一個執行緒會接收到數值並印出來。我們會使用通道在執行緒間傳送簡單的數值來作為這個功能的解說。一旦你熟悉此技巧後，你可以使用通道來實作個聊天系統，或是一個利用數個執行緒進行運算，然後將結果傳入一個執行緒統整結果的分散式系統。

首先在範例 16 -6，我們會建立個通道但還不會做任何事。注意這樣不會編譯通過因為 Rust 無法知道我們想對通道傳入的數值型別為何。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">範例 16-6：建立通道並賦值分別兩個部分給 `tx` 與 `rx`</span>

我們使用 `mpsc::channel` 函式來建立新的通道，`mpsc` 指的是**多重生產者、唯一消費者（multiple producer, single consumer）**。簡單來說，Rust 標準函式庫實作通道的方式讓通道可以有多個**發送端**來產生數值，不過只有一個**接收端**能消耗這些數值。想像有數個溪流匯聚成一條大河流，任何溪流傳送的任何東西最終都會流向河流的下游。我們會先從單一生產者開始，等這個範例能夠執行後我們再來增加數個生產者。

`mpsc::channel` 函式會回傳一個元組，第一個元素是發送端然後第二個元素是接收端。`tx` 與 `rx` 通常分別作為**發送者**（transmitter）與**接收者**（receiver）的縮寫，所以我們以此作為我們的變數名稱。我們的 `let` 陳述式使用到了能解構元組的模式我們會在第時八章討論 `let` 陳述式的模式與解構方式。用這樣的方式使用 `let` 能輕鬆取出 `mpsc::channel` 回傳的元組每個部分。

讓我們將發送端移進一個新產生的執行緒並讓它傳送一條字串，這樣產生的執行緒就可以與主執行緒溝通了，如範例 16-7 所示。這就像是在河流上游放了一隻橡皮小鴨，或是從一條執行緒傳送一條聊天訊息給別條執行緒一樣。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<span class="caption">範例 16-7：將 `tx` 移入產生的執行緒並傳送「hi」</span>

我們再次使用 `thread::spawn` 來建立新的執行緒並使用 `move` 將 `tx` 移入閉包，讓產生的執行緒擁有 `tx`。產生的執行緒必須要擁有通道的發送端才能夠傳送訊息至通道。

發送端有個 `send` 方法可以接受我們想傳遞的數值。`send` 方法會回傳 `Result<T, E>` 型別，所以如果接收端已經被釋放因而沒有任何地方可以傳遞數值的話，傳送的動作就會回傳錯誤。在此例中，我們呼叫 `unwrap` 所以有錯誤時就會直接恐慌。但在實際的應用程式中，我們會更妥善地處理它，你可以回顧第九章來複習如何適當地處理錯誤。

在範例 16-8 我們會在主執行緒中從通道的接收端取得數值。這就像在河流下游取回順流而下的橡皮小鴨，或是像取得一條聊天訊息一樣。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<span class="caption">範例 16-8：在主執行緒取得數值「hi」並顯示出來</span>

通道的接收端有兩個實用的方法：`recv` 與 `try_recv`。我們使用 `recv` 作為**接收**（receive）的縮寫，這位阻擋主執行緒的運行並等待直到通道有訊息傳入。一旦有數值傳遞，`recv` 會就以此回傳 `Result<T, E>`。當通道的發送端關閉時，`recv` 會回傳錯誤來通知不會再有任何數值出現了。

`try_recv` 方法則不會阻擋，而是會立即回傳 `Result<T, E>`。如果有數值的話，就會是存有訊息的 `Ok` 數值，如果尚未有任何數值的話，就會是 `Err` 數值。`try_recv` 適用於如果此執行緒在等待訊息的同時有其他事要做的情形。我們可以寫個迴圈來時不時呼叫 `try_recv`，當有數值時處理訊息，不然的話就先做點其他事直到再次檢查為止。

我們出於方便考量在此例使用 `recv`，我們的主執行緒除了等待訊息以外沒有其他事好做，所以阻擋主執行緒是合理的。

當我們執行範例 16-8 的程式碼，我們會看到主執行緒印出的數值：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
取得：嗨
```

太棒了！

### 通道與所有權轉移

所有權規則在訊息傳遞中扮演了重要的角色，因為它們可以幫助你寫出安全的並行程式碼。在 Rust 程式中考慮所有權的其中一項好處就是你能在並行程式設計避免錯誤發生。讓我們做個實驗來看通道與所有權如何一起合作來避免問題發生，我們會在 `val` 數值傳送給通道**之後**嘗試使用其值。請嘗試編譯範例 16-9 的程式碼並看看為何此程式碼不被允許：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<span class="caption">範例 16-9：在我們將 `val` 傳入通道後嘗試使用其值</span>

我們在這裡透過 `tx.send` 將 `val` 傳入通道之後嘗試印出其值。允許這麼做的話會是個壞主意，一旦數值被傳至其他執行緒，該執行緒就可以在我們嘗試再次使用該值之前修改或釋放其值。其他執行緒的修改有機會因為不一致或不存在的資料而導致錯誤或意料之外的結果。不過如果我試著編譯範例 16-9 的程式碼的話，Rust 會給我們一個錯誤：

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

我們的並行錯誤產生了一個編譯時錯誤。`send` 函式會取走其參數的所有權，並當數值移動時，接收端會在取得其所有權。這能阻止我們在傳送數值過後不小心再次使用其值，所有權系統會檢查一切是否符合規則。

### 傳送多重數值並觀察接收者等待

範例 16-8 的程式碼可以編譯通過並執行，但它並沒有清楚表達出兩個不同的執行緒正透過通道彼此溝通。在範例 16-10 中我們做了些修改來證明範例 16-8 的程式有正確執行，產生的執行緒先在會傳送數個訊息並在每個訊息間暫停個一秒鐘。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<span class="caption">範例 16-10：傳送數個訊息並在之間暫停片刻</span>

這次產生的執行緒有個字串向量，我們希望能傳送它們到主執行緒中。我們遍歷它們，單獨傳送每個值，然後透過 `Duration` 數值呼叫 `thread::sleep` 來暫停一秒。

在主執行緒中，我們不再顯式呼叫 `recv` 函式，我們改將 `rx` 作為疊代器使用。對每個接收到的數值，我們印出它。當通道關閉時，疊代器就會結束。

當執行範例 16-10 的程式碼，你應該會看到以下輸出，每一行會間格一秒鐘：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
取得：執行緒
取得：傳來
取得：的
取得：嗨
```

因為我們在主執行緒中的 `for` 迴圈內沒有任何會暫停或延遲的程式碼，所以我們可以看出主執行緒是在等待產生的執行緒傳送的數值。

### 透過克隆發送者來建立多重生產者

稍早之前我們提過 `mpsc` 是**多重生產者、唯一消費者**（multiple producer, single consumer）的縮寫。讓我們來使用 `mpsc` 並擴產範例 16-10 的程式碼來建立數個執行緒，它們都將傳遞數值給同個接收者。為此我們可以克隆通道的發送部分，如範例 16-11 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<span class="caption">範例 16-11：從多重生產者傳遞數個訊息</span>

這次在我們建立第一個產生的執行緒前，我們會對通道的發送端呼叫 `clone`。這能給我們一個新的發送者，讓我們可以移入第一個產生的執行緒。接著我們將原本的通道發送端移入第二個產生的執行緒中。這樣我們就有了兩條執行緒，每條都能傳送不同的訊息給通道的接收端。

當你執行程式碼時，你的輸出應該會類似以下結果：

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
取得：執行緒
取得：更多
取得：傳來
取得：給你
取得：的
取得：的
取得：嗨
取得：訊息
```

你可能會看到數值以不同順序排序，這完全依據你的系統來決定。這正是並行程式設計既有趣卻又困難的地方。如果你加上 `thread::sleep` 來實驗，並在不同執行緒給予不同數值的話，就會發現每一輪都會更不確定，每次都會產生不同的輸出結果。

現在我們已經看完通道如何運作，接著讓我們來看看並行的不同方法吧。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch16-02-message-passing.md)
> - updated: 2020-09-21
