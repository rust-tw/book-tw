## 正常關機與清理

範例 20-20 的程式碼能如我們所預期地使用執行緒池來同時回應多重請求。我們有看到些警告說 `workers`、`id` 與 `thread` 欄位沒有被直接使用，這提醒我們尚未清理所有內容。當我們使用比較不優雅的 <span class="keystroke">ctrl-c</span> 方式來中斷主執行緒時，所有其他執行緒也會立即停止，不管它們是否正在處理請求。

現在我們要實作 `Drop` 特徵來對池中的每個執行緒呼叫 `join`，讓它們能在關閉前把任務處理完畢。然後我們要實作個方式來告訴執行緒它們該停止接收新的請求並關閉。為了觀察此程式碼的實際運作，我們會修改伺服器讓它在正常關機（graceful shutdown）前，只接收兩個請求。

###  對 `ThreadPool` 實作 `Drop` 特徵

讓我們先對執行緒池實作 `Drop` 。當池被釋放時，我們的執行緒都該加入（join）回來以確保它們有完成它們的工作。範例 20-22 為實作 `Drop` 的第一次嘗試，不過此程式碼還無法執行。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-22/src/lib.rs:here}}
```

<span class="caption">範例 20-22：在執行緒池離開作用域將每個執行緒加入回來</span>

首先，我們遍歷執行緒池中的每個 `workers`。我們對此使用 `&mut` 因為 `self` 是個可變引用，而且我們也需要能夠改變 `worker`。我們對每個工作者印出訊息來說明此工作者正要關閉，然後我們對工作者的執行緒呼叫 `join`。如果 `join` 的呼叫失敗的話，我們使用 `unwrap` 來讓 Rust 恐慌使其變成較不正常的關機方式。

以下是當我們編譯此程式碼時產生的錯誤：

```console
{{#include ../listings/ch20-web-server/listing-20-22/output.txt}}
```

錯誤告訴我們無法呼叫 `join`，因為我們只有每個 `worker` 的可變借用，而 `join` 會取走其引數的所有權。要解決此問題，我們需要將 `thread` 中的執行緒移出 `Worker` 實例，讓 `join` 可以消耗該執行緒。我們在範例 17-15 做過這樣的事，如果 `Worker` 改持有 `Option<thread::JoinHandle<()>>` 的話，我們可以對 `Option` 呼叫 `take` 方法來移動 `Some` 變體中的數值，並在原處留下 `None` 變體。換句話說，`thread` 中有 `Some` 變體的話就代表 `Worker` 正在執行，而當我們清理 `Worker` 時，我們會將 `Some` 換成 `None` 來讓 `Worker` 沒有任何執行緒可以執行。

所以我們想要更新 `Worker` 的定義如以下所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-04-update-worker-definition/src/lib.rs:here}}
```

現在讓我們再看看編譯器的結果中還有哪些地方需要修改。檢查此程式碼，我們會得到兩個錯誤：

```console
{{#include ../listings/ch20-web-server/no-listing-04-update-worker-definition/output.txt}}
```

讓我們來修復第二個錯誤，這指向程式碼中 `Worker::new` 的結尾。當我們建立新的 `Worker`，我們需要將 `thread` 的數值封裝到 `Some`。請作出以下改變來修正程式碼：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```

而第一個錯誤則位在 `Drop` 的實作中。我們剛剛有提到我們打算對 `Option` 呼叫 `take` 來將 `thread` 移出 `worker`。所以以下改變就能修正：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```

如同第十七章所討論的，`Option` 的 `take` 方法會取走 `Some` 變體的數值並在原地留下 `None`。我們使用 `if let` 來解構 `Some` 並取得執行緒，然後我們對執行緒呼叫 `join`。如果工作者的執行緒已經是 `None`，我們就知道該該工作者已經清理其執行緒了，所以沒有必要再處理。

### 對執行緒發送停止接收任務的信號

有了以上的改變，我們的程式碼就能成功編譯且沒有任何警告。但壞消息是此程式碼並沒有如我們所預期地運作。關鍵邏輯位於 `Worker` 實例中執行緒執行的閉包，現在雖然我們有呼叫 `join`，但這無法關閉執行緒，因為它們會一直 `loop` 來尋找任務執行。如果我們嘗試以目前的 `drop` 實作釋放 `ThreadPool` 的話，主執行緒會被阻擋，一直等待第一個執行緒處理完成。

要修正此問題，我們要修改執行緒，讓它們除了接收 `Job` 來執行以外，也要能收到告訴它們要停止接收並離開無限迴圈的信號。所以我們的通道將傳送以下兩種枚舉變體，而不再是 `Job` 實例。
<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-define-message-enum/src/lib.rs:here}}
```

`Message` 此枚舉可以是存有該執行緒要執行的 `Job` 的 `NewJob` 變體，或是通知執行緒離開其迴圈並停止的 `Terminate` 變體。

我們需要調整通道來使用 `Message` 型別，而不是 `Job` 型別，如範例 20-23 所示。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-23/src/lib.rs:here}}
```

<span class="caption">範例 20-23：傳送與接收 `Message` 數值，且如果 `Worker` 收到 `Message::Terminate` 時就會離開迴圈</span>

為了改用 `Message` 枚舉，我們有兩個地方得將 `Job` 改成 `Message`：`ThreadPool` 的定義與 `Worker::new` 的簽名。`ThreadPool` 的 `execute` 方法需要傳送封裝成 `Message::NewJob` 的任務。然後在 `Worker::new` 中，也就是取得 `Message` 的通道接收端中，如果收到 `NewJob` 變體的話，其就會處理任務；而如果收到 `Terminate` 變體的話，執行緒就會打破迴圈。

有了這些改變，程式碼就能編譯並繼續以範例 20-20 之後的行為來執行。但我們會得到一個警告，因為我們還沒建立任何 `Terminate` 變體的訊息。讓我們來修正此警告，如範例 20-24 所示來改變 `Drop` 的實作。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-24/src/lib.rs:here}}
```

<span class="caption">範例 20-24：在對每個工作者執行緒呼叫 `join` 之前，傳送 `Message::Terminate` 給工作者</span>

我們現在會遍歷工作者們兩次，一次是傳送 `Terminate` 訊息給每個工作者，另一次是對每個工作者執行緒呼叫 `join`。如果我們嘗試在同個迴圈中傳送訊息並立即呼叫 `join` 的話，我們無法保證在當前疊代中的工作者就是從通道中取得訊息的工作者。

為了更好理解為何我們需要兩個分開的迴圈，想像一個情境中有兩個工作者。如果我們用一個迴圈來遍歷每個工作者，在第一次疊代中會有個關機訊息傳至通道，並對第一個工作者執行緒呼叫 `join`。如果第一個工作者正在忙於處理請求的話，第二個工作者就會從通道取得關機訊息並關閉。這樣會變成持續等待第一個工作者關閉，但是它永遠不會關閉，因為是第二個執行緒取得關機訊息的。死結（deadlock）就發生了！

為了預防此情形，我們首先在一個迴圈中對通道傳送所有的 `Terminate` 訊息，然後我們在另一個迴圈才將所有的執行緒加回來。每個工作者一旦收到關機訊息後，就會停止從通道中接收訊息。所以我們可以確定如果我們發送與執行緒數量相當的關機訊息的話，每個工作者都會在其執行緒被呼叫 `join` 前收到關機訊息。

要實際看到此程式碼的運作情形，讓我們修改 `main` 來在正常關閉伺服器前，只接收兩個請求，如範例 20-25 所示。

<span class="filename">檔案名稱：src/bin/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-25/src/bin/main.rs:here}}
```

<span class="caption">範例 20-25：在處理兩個請求後，離開迴圈並關閉伺服器</span>

在真實世界中的網頁伺服器當然不會只處理兩個請求就關機。此程式碼只是用來說明正常關機與清理的運作流程。

`take` 方法是由 `Iterator` 特徵所定義且我們限制該疊代最多只會取得前兩項。`ThreadPool` 會在 `main` 結束時離開作用域，然後 `drop` 的實作就會執行。

使用 `cargo run` 開啟伺服器，並下達三個請求。第三個請求應該會出現錯誤，而在你的終端機中你應該會看到類似以下的輸出：

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/main`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

你可能會看到不同順序的工作者與訊息輸出。我們可以從訊息中看到此程式碼如何執行的，工作者 0 與 3 獲得前兩個請求。然後對於第三個請求，伺服器會停止接受連線。當 `ThreadPool` 在 `main` 結尾離開作用域時，它 `Drop` 的實作就會生效，然後執行緒池告訴所有工作者關閉。當工作者看到關機訊息時，它們就會印出訊息，然後執行緒池會呼叫 `join` 來關閉每個工作者的執行緒。

此特定執行方式中有個有趣的地方值得注意：`ThreadPool` 傳送關機訊息至通道，且在任何工作者收到訊息前，我們就已經著將工作者 0 加入回來。工作者 0 此時尚未收到關機訊息，所以主執行緒會被擋住並等待工作者 0 完成。同一時間，每個工作者會開始收到關機訊息。當工作者 0 完成時，主執行緒會等待剩下的工作者完成任務。屆時，它們都會收到關機訊息並能夠關閉。

恭喜！我們的專案完成了，我們有個基礎的網頁瀏覽器，其使用執行緒池來做非同步回應。我們能夠對伺服器正常關機，並清理池中所有的執行緒。

以下是完整的程式碼參考：

<span class="filename">檔案名稱：src/bin/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-08-final-code/src/bin/main.rs}}
```

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-08-final-code/src/lib.rs}}
```

我們還可以做更多事！如果你想繼續改善此專案的話，以下是些不錯的點子：

* 對 `ThreadPool` 與其公開方法加上技術文件。
* 對函式庫功能加上測試。
* 將 `unwrap` 的呼叫改成更完善的錯誤處理。
* 使用 `ThreadPool` 來處理其他種類的任務，而不只是網頁請求。
* 在 [crates.io](https://crates.io/) 找到一個執行緒池 crate，並使用該 crate 實作類似的網頁伺服器。然後比較該 crate 與我們實作的執行緒池之間的 API 與穩固程度。

## 總結

做得好！你已經讀完整本書了！我們由衷感謝你一同加入 Rust 的旅途。現在你已經準備好實作你自己的 Rust 專案並協助其他人的專案。別忘了我們有個友善的社群，其他 Rustaceans 會很樂意幫助你一同面對 Rust 旅途中的任何挑戰。
