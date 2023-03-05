## 正常關機與清理

範例 20-20 的程式碼能如我們所預期地使用執行緒池來同時回應多重請求。我們有看到些警告說 `workers`、`id` 與 `thread` 欄位沒有被直接使用，這提醒我們尚未清理所有內容。當我們使用比較不優雅的 <span class="keystroke">ctrl-c</span> 方式來中斷主執行緒時，所有其他執行緒也會立即停止，不管它們是否正在處理請求。

接著我們要實作 `Drop` 特徵來對池中的每個執行緒呼叫 `join`，讓它們能在關閉前把任務處理完畢。然後我們要實作個方式來告訴執行緒它們該停止接收新的請求並關閉。為了觀察此程式碼的實際運作，我們會修改伺服器讓它在正常關機（graceful shutdown）前，只接收兩個請求。

###  對 `ThreadPool` 實作 `Drop` 特徵

讓我們先對執行緒池實作 `Drop` 。當池被釋放時，我們的執行緒都該加入（join）回來以確保它們有完成它們的工作。範例 20-22 為實作 `Drop` 的第一次嘗試，不過此程式碼還無法執行。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-22/src/lib.rs:here}}
```

<span class="caption">範例 20-22：在執行緒池離開作用域將每個執行緒加入回來</span>

首先，我們遍歷執行緒池中的每個 `workers`。我們對此使用 `&mut` 因為 `self` 是個可變參考，而且我們也需要能夠改變 `worker`。我們對每個工作者印出訊息來說明此工作者正要關閉，然後我們對工作者的執行緒呼叫 `join`。如果 `join` 的呼叫失敗的話，我們使用 `unwrap` 來讓 Rust 恐慌使其變成較不正常的關機方式。

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

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```

而第一個錯誤則位在 `Drop` 的實作中。我們剛剛有提到我們打算對 `Option` 呼叫 `take` 來將 `thread` 移出 `worker`。所以以下改變就能修正：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```

如同第十七章所討論的，`Option` 的 `take` 方法會取走 `Some` 變體的數值並在原地留下 `None`。我們使用 `if let` 來解構 `Some` 並取得執行緒，然後我們對執行緒呼叫 `join`。如果工作者的執行緒已經是 `None`，我們就知道該該工作者已經清理其執行緒了，所以沒有必要再處理。

### 對執行緒發送停止接收任務的信號

有了以上的改變，我們的程式碼就能成功編譯且沒有任何警告。但壞消息是此程式碼並沒有如我們所預期地運作。關鍵邏輯位於 `Worker` 實例中執行緒執行的閉包，現在雖然我們有呼叫 `join`，但這無法關閉執行緒，因為它們會一直 `loop` 來尋找任務執行。如果我們嘗試以目前的 `drop` 實作釋放 `ThreadPool` 的話，主執行緒會被阻擋，一直等待第一個執行緒處理完成。

要修正此問題，我們要修改 `ThreadPool` `drop` 的實作以及 `Worker` 內的一些程式碼。

首先我們先將 `ThreadPool` `drop` 的實作改成在執行緒完成前就顯式釋放 `sender`。範例 20-23 展示了 `ThreadPool` 顯式釋放 `sender`。我們使用處理執行緒時一樣的 `Option` 與 `take` 技巧來將 `sender` 移出 `ThreadPool`：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-23/src/lib.rs:here}}
```

<span class="caption">範例 20-23：在工作者執行緒加入回來前顯式釋放 `sender`</span>

釋放 `sender` 會關閉通道，也就代表沒有任何訊息會再被傳送。工作者在無限迴圈呼叫的 `recv` 會回傳錯誤。在範例 20-24 中，我們改變 `Worker` 的迴圈來處理該狀況並正常退出迴圈，也就是說 `ThreadPool` `drop` 的實作呼叫 `join` 時，執行緒就會工作完成。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-24/src/lib.rs:here}}
```

<span class="caption">範例 20-24：當 `recv` 回傳錯誤時就顯式退出迴圈</span>

要實際看到此程式碼的運作情形，讓我們修改 `main` 來在正常關閉伺服器前，只接收兩個請求，如範例 20-25 所示。

<span class="filename">檔案名稱：src/bin/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-25/src/main.rs:here}}
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
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

你可能會看到不同順序的工作者與訊息輸出。我們可以從訊息中看到此程式碼如何執行的，工作者 0 與 3 獲得前兩個請求。在第二個請求之後，伺服器會停止接受連線。然後在工作者 3 開始工作之前，`ThreadPool` 的 `Drop` 實作就會執行。釋放 `sender` 會將所有工作者斷線並告訴它們關閉。每個工作者在斷線時都印出訊息，然後執行緒池會呼叫 `join` 來等待每個工作者的執行緒完成。

此特定執行方式中有個有趣的地方值得注意：在 `ThreadPool` 釋放 `sender` 然後任何工作者收到錯誤之前，我們嘗試將工作者 0 加入回來。工作者 0 尚未從 `recv` 收到錯誤，所以主執行緒會被擋住並等待工作者 0 完成。同一時間，工作者 3 收到一份工作但所有執行緒都收到錯誤。當工作者 0 完成時，主執行緒會等待剩下的工作者完成任務。屆時，它們都會退出它們的迴圈並能夠關閉。

恭喜！我們的專案完成了，我們有個基礎的網頁瀏覽器，其使用執行緒池來做非同步回應。我們能夠對伺服器正常關機，並清理池中所有的執行緒。

以下是完整的程式碼參考：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-final-code/src/main.rs}}
```

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-final-code/src/lib.rs}}
```

我們還可以做更多事！如果你想繼續改善此專案的話，以下是些不錯的點子：

* 對 `ThreadPool` 與其公開方法加上技術文件。
* 對函式庫功能加上測試。
* 將 `unwrap` 的呼叫改成更完善的錯誤處理。
* 使用 `ThreadPool` 來處理其他種類的任務，而不只是網頁請求。
* 在 [crates.io](https://crates.io/) 找到一個執行緒池 crate，並使用該 crate 實作類似的網頁伺服器。然後比較該 crate 與我們實作的執行緒池之間的 API 與穩固程度。

## 總結

做得好！你已經讀完整本書了！我們由衷感謝你一同加入 Rust 的旅途。現在你已經準備好實作你自己的 Rust 專案並協助其他人的專案。別忘了我們有個友善的社群，其他 Rustaceans 會很樂意幫助你一同面對 Rust 旅途中的任何挑戰。
