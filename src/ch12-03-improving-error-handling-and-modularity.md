## 透過重構來改善模組性與錯誤處理

為了改善我們的程式，我們需要修正四個問題，這與程式架構與如何處理潛在錯誤有關。

首先，我們的 `main` 函式會處理兩件任務：它得解析引數並讀取檔案。對於這麼小的函式來說，這不是大問題。然而，要是我們持續在 `main` 增加我們的程式，`main` 函式中要處理的任務就會增加。要是一個函式有這麼多責任，它就會越來越難理解、越難測試並且難在不破壞其他部分的情況下做改變。我們最好能將不同功能拆開，讓每個函式只負責一項任務。

而這也和第二個問題有關：雖然 `query` 與 `filename` 是我們程式的設置變數，而變數 `contents` 則用於程式邏輯。隨著 `main` 增長，我們會需要引入越多變數至作用域中。而作用域中有越多變數，我們就越難追蹤每個變數的用途。我們最好是將設置變數集結成一個結構體，讓它們的用途清楚明白。

第三個問題是當讀取檔案失敗時，我們使用 `expect` 來印出錯誤訊息，但是錯誤訊息只印出 `讀取檔案時發生了錯誤`。讀取檔案可以有好幾種失敗的方式：舉例來說，檔案可能不存在，或是我們可能沒有權限能開啟它。目前不管原因為何，我們都只印出錯誤訊息 `讀取檔案時發生了錯誤`，這並沒有給使用者足夠的資訊！

第四，我們重複使用 `expect` 來處理不同錯誤，而如果有使用者沒有指定足夠的引數來執行程式的話，他們會從 Rust 獲得 `index out of bounds` 的錯誤，這並沒有清楚解釋問題。最好是所有的錯誤處理程式碼都可以位於同個地方，讓未來的維護者只需要在此處來修改錯誤處理的程式碼。將所有錯誤處理的程式碼置於同處也能確保我們能提供對終端使用者有意義的訊息。

讓我們來重構專案以解決這四個問題吧。

### 分開二進制專案的任務

`main` 函式負責多數任務的組織分配問題在許多二進制專案中都很常見。所以 Rust 社群開發出了一種流程，這在當 `main` 開始變大時，能作為分開二進制程式中任務的指導原則。此流程有以下步驟：

* 將你的程式分成 *main.rs* 與 *lib.rs* 並將程式邏輯放到 *lib.rs*。
* 只要你的命令列解析邏輯很小，它可以留在 *main.rs*。
* 當命令行解析邏輯變得複雜時，就將其從 *main.rs* 移至 *lib.rs*。

在此流程之後的 `main` 函式應該要只負責以下任務：

* 透過引數數值呼叫命令列解析邏輯
* 設置任何其他的配置
* 呼叫 *lib.rs* 中的 `run` 函式
* 如果 `run` 回傳錯誤的話，處理該錯誤

此模式用於分開不同任務：*main.rs* 處理程式的執行，然後 *lib.rs* 處理眼前的所有任務邏輯。因為你無法直接測試 `main`，此架構讓你能測試所有移至 *lib.rs* 的程式函式邏輯。留在 *main.rs* 的程式碼會非常小，所以容易直接用閱讀來驗證。讓我們用此流程來重構程式吧。

#### 提取引數解析器

我們會提取解析引數的功能到一個 `main` 會呼叫的函式中，以將命令列解析邏輯妥善地移至 *src/lib.rs*。範例 12-5 展示新的 `main` 會呼叫新的函式 `parse_config`，而此函式我們先暫時留在 *src/main.rs*。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<span class="caption">範例 12-5：從 `main` 提取 `parse_config` 函式</span>

我們仍然收集命令列引數至向量中，但不同於在 `main` 函式中將索引 1 的引數數值賦值給變數 `query` 且將索引 2 的引數數值賦值給變數 `filename`，我們將整個向量傳至 `parse_config` 函式。`parse_config` 函式會擁有決定哪些引數要賦值給哪些變數的邏輯，並將數值回傳給 `main`。我們仍然在 `main` 中建立變數 `query` and `filename`，但 `main` 不再負責決定命令列引數與變數之間的關係。

此重構可能對我們的小程式來說有點像是殺雞焉用牛刀，但是我們正一小步一小步地累積重構。做了這項改變後，請再次執行程式來驗證引數解析有沒有正常運作。經常檢查你的進展是很好的，這能幫助你找出問題發生的原因。

#### 集結配置數值

我們可以再進一步改善 `parse_config` 函式。目前我們回傳的是元組，但是我們馬上又將元組拆成獨立部分。這是個我們還沒有建立正確抽象的信號。

另外一個告訴我們還有改善空間的地方是 `parse_config` 名稱中的 `config`，這指示我們回傳的兩個數值是相關的，且都是配置數值的一部分。我們現在沒有確實表達出這樣的資料結構，而只有將兩個數值組合成一個元組而已，我們可以將這兩個數值存入一個結構體，並對每個結構體欄位給予有意義的名稱。這樣做能讓未來的維護者可以清楚知道這些數值的不同與關聯，以及它們的用途。

> 注意：當使用複雜型別會比較理想時，卻仍使用原始數值的反模式（anti-pattern）被稱之為**原始型別偏執（primitive obsession）**。

範例 12-6 改善了 `parse_config` 函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<span class="caption">範例 12-6：重構 `parse_config` 來返回 `Config` 結構體實例</span>

我們定義了一個結構體 `Config` 其欄位有 `query` 與 `filename`。`parse_config` 的簽名現在指明它會回傳一個 `Config` 數值。在 `parse_config` 的本體中，我們原先回傳 `args` 中 `String` 數值引用的字串切片，現在我們定義 `Config` 來包含具所有權的 `String` 數值。`main` 中的 `args` 變數是引數數值的擁有者，而且只是借用它們給 `parse_config` 函式，這意味著如果 `Config` 嘗試取得 `args` 中數值的所有權的話，我們會違反 Rust 的借用規則。

我們可以用許多不同的方式來管理 `String` 的資料，但最簡單（卻較不有效率）的方式是對數值呼叫 `clone` 方法。這會複製整個資料讓 `Config` 能夠擁有，這會比引用字串資料還要花時間與記憶體。然而克隆資料讓我們的程式碼比較直白，因為在此情況下我們就不需要管理引用的生命週期，犧牲一點效能以換取簡潔性是值得的。

> ### 使用 `clone` 的權衡取捨
>
> 由於 `clone` 會有運行時消耗，所以許多 Rustaceans 傾向於避免使用它來修正所有權問題。在[第十三章][ch13]<!-- ignore -->中，你會學到如何更有效率的處理這種情況。但現在我們可以先複製字串來繼續進行下去，因為你只複製了一次，而且檔案名稱與搜尋字串都算很小。先寫出較沒有效率但可執行的程式會比第一次就要過分優化還來的好。隨著你對 Rust 越熟練，你的確就可以從有效率的解決方案開始，但現在呼叫 `clone` 是完全可以接受的。

我們更新 `main` 來將 `parse_config` 回傳的 `Config` 實例儲存至 `config` 變數中，並更新之前分別使用變數 `query` 與 `filename` 的程式碼段落來改使用 `Config` 結構體中的欄位。

現在我們的程式碼更能表達出 `query` 與 `filename` 是相關的，而且它們的目的是配置程式的行為。任何使用這些數值的程式碼都會從 `config` 實例中的欄位名稱知道它們的用途。

#### 建立 `Config` 的建構子

目前我們將負責解析命令列引數的邏輯從 `main` 移至 `parse_config` 函式。這樣做能幫助我們理解 `query` 與 `filename` 數值是相關的，且此關係應該要能在我們的程式碼中表達出來。然後我們增加了結構體 `Config` 來描述 `query` 與 `filename` 的相關性，並在 `parse_config` 函式中將數值名稱作為結構體欄位名稱來回傳。

所以現在 `parse_config` 函式的目的是要建立 `Config` 實例，我們可以將 `parse_config` 從普通的函式變成與 `Config` 結構體相關連的 `new` 函式。這樣做能讓程式碼更符合慣例。我們可以對像是 `String` 等標準函式庫中的型別呼叫 `String::new` 來建立實例。同樣地，透過將 `parse_config` 改為 `Config` 的關聯函式 `new`，我們可以透過呼叫 `Config::new` 來建立 `Config` 的實例。範例 12-7 正是我們要作出的改變。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<span class="caption">範例 12-7：變更 `parse_config` 成 `Config::new`</span>

我們更新了 `main` 原先呼叫 `parse_config` 的地方來改呼叫 `Config::new`。我們變更了 `parse_config` 的名稱成 `new` 並移入 `impl` 區塊中，讓 `new` 成為 `Config` 的關聯函式。請嘗試再次編譯此程式碼來確保它能執行。

### 修正錯誤處理

現在我們要來修正錯誤處理。回想一下要是 `args`向量中的項目太少的話，嘗試取得向量中索引 1 或索引 2 的數值的話可能就會導致程式恐慌。試著不用任何引數執行程式的話，它會產生以下結果：

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` 這行是給程式設計師看得錯誤訊息。這無法協助我們的終端使用者理解發生了什麼事以及他們開怎麼處理。讓我們來修正吧。

#### 改善錯誤訊息

在範例 12-8 中，我們在 `new` 函式加上了一項檢查來驗證 slice 是否夠長，接著才會取得索引 1 和 2。如果 slice 不夠長的話，程式就會恐慌並顯示比 `indexout of bounds` 還清楚的錯誤訊息。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

<span class="caption">範例 12-8：新增對引數數量的檢查</span>

此程式碼類似於我們在[範例 9-10 寫的 `Guess::new` 函式][ch9-custom-types]<!-- ignore -->，在那裡當 `value` 超出有效數值的範圍時，我們就呼叫 `panic!`。然而在此我們不是檢查數值的範圍，而是檢查 `args` 的長度是否至少為 3，然後函式剩餘的段落都能在假設此條件成立情況下正常執行。如果 `args` 的項目數量少於三的話，此條件會為真，然後我們就會立即呼叫 `panic!` 巨集來結束程式。

在 `new` 多了這些額外的程式碼之後，讓我們不用任何引數再次執行程式，來看看錯誤訊息為何：

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

這樣的輸出就好多了，我們現在有個合理的錯誤訊息。然而我們還是顯示了一些額外資訊給使用者。也許在此使用範例 9-10 的技巧並不是最好的選擇，如同[第九章所提及的][ch9-error-guidelines]<!-- ignore -->，`panic!` 的呼叫比較屬於程式設計問題，而不是使用問題。我們可以改使用第九章的其他技巧，像是[回傳 `Result`][ch9-result]<!-- ignore -->來表達是成功還是失敗。

#### 從 `new` 回傳 `Result` 而非呼叫 `panic!`

我們可以回傳 `Result` 數值，在成功時包含 `Config` 的實例並在錯誤時描述問題原因。當 `Config::new` 與 `main` 溝通時，我們可以使用 `Result` 型別來表達這裡有問題發生。然後我們改變 `main` 來將 `Err` 變體轉換成適當的錯誤訊息給使用者，而不是像呼叫 `panic!` 時出現圍繞著 `thread 'main'` 與 `RUST_BACKTRACE` 的文字。

範例 12-9 顯示我們得改變 `Config::new` 的回傳值並讓函式本體回傳 `Result`。注意到這還不能編譯，直到我們也更新 `main` 為止，這會在下個範例解釋。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<span class="caption">範例 12-9：從 `Config::new` 回傳 `Result`</span>

我們的 `new` 函式現在會回傳 `Result`，在成功時會有 `Config` 實例，而在錯誤時會有個 `&str`。

我們在 `new` 函式本體作出了兩項改變：不同於呼叫 `panic!`，當使用者沒有傳遞足夠引數時，我們現在會回傳 `Err` 數值。此外我們也將 `Config` 封裝進 `Ok` 作為回傳值。這些改變讓函式能符合其新的型別簽名。

從 `Config::new` 回傳 `Err` 數值讓 `main` 函式能處理 `new` 函式回傳的 `Result` 數值，並明確地在錯誤情況下離開程序。

#### 呼叫 `Config::new` 並處理錯誤

為了能處理錯誤情形並印出對使用者友善的訊息，我們需要更新 `main` 來處理 `Config::new` 回傳的 `Result`，如範例 12-10 所示。我們還要負責用一個非零的錯誤碼來離開命令列工具，這原先是 `panic!` 會處理的，現在我們得自己實作。非零退出狀態是個常見信號，用來告訴呼叫程式的程序，該程式離開時有個錯誤狀態。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<span class="caption">範例 12-10：如果建立新的 `Config` 失敗時會用錯誤碼離開</span>

在此範例中，我們使用一個還沒詳細介紹的方法 `unwrap_or_else`，這定義在標準函式庫的 `Result<T, E>` 中。使用 `unwrap_or_else` 讓我們能定義一些自訂的非 `panic!` 錯誤處理。如果 `Result` 數值為 `Ok`，此方法行為就類似於 `unwrap`，它會回傳`Ok` 所封裝的內部數值。然而，如果數值為 `Err` 的話，此方法會呼叫**閉包**（closure）內的程式碼，這會是由我們所定義的匿名函式並作為引數傳給 `unwrap_or_else`。我們會在[第十三章][ch13]<!-- ignore -->詳細介紹閉包。現在你只需要知道 `unwrap_or_else` 回傳遞 `Err` 的內部數值，在此例中就是我們在範例 12-9 新增的靜態字串「引數不足」，將此數值傳遞給閉包中兩條直線之間的 `err` 引數。閉包內的程式碼就可以在執行時使用 `err` 數值。

我們新增了一行 `use` 來將標準函式庫中的 `process` 引入作用域。在錯誤情形下要執行的閉包程式碼只有兩行：我們印出 `err` 數值並呼叫 `process::exit`。`process::exit` 函式會立即停止程式並回傳給予的數字來作為退出狀態碼。這與範例 12-8 我們使用 `panic!` 來處理的方式類似，但我們不再顯示多餘的輸出結果。讓我們試試看：

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

很好！這樣的輸出結果對使用者友善多了。

### 從提取 `main` 邏輯

現在我們完成配置解析的重構了，接下來輪到程式邏輯了。如同我們在[「分開二進制專案的任務」](#分開二進制專案的任務)<!-- ignore -->中所提及的，我們會提取一個函式叫做 `run`，這會存有目前 `main` 函式中除了設置配置或處理錯誤以外的所有邏輯。當我們完成後，`main` 會變得非常簡潔，且能輕鬆用肉眼來驗證，然後我就能對所有其他邏輯進行測試了。

範例 12-11 提取了 `run` 函式。目前我們在提取函式時，會逐步作出小小的改善。我們仍然在 *src/main.rs* 底下定義函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<span class="caption">範例 12-11：提取 `run` 函式來包含剩餘的程式邏輯</span>

`run` 現在會包含 `main` 中從讀取文件開始的所有剩餘邏輯。`run` 函式會接收 `Config` 實例來作為引數。

#### 從 `run` 函式回傳錯誤

隨著剩餘程式邏輯都移至 `run` 函式，我們可以像範例 12-9 的 `Config::new` 一樣來改善錯誤處理。不同於讓程式呼叫 `expect` 來恐慌，當有問題發生時，`run` 函式會回傳 `Result<T, E>`。這能讓我們進一步穩固 `main` 中對使用者友善的處理錯誤邏輯。範例 12-12 展示我們對 `run` 的簽名與本體所需要做的改變。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<span class="caption">範例 12-12：變更 `run` 函式來回傳 `Result`</span>

我們在此做了三項明顯的修改。首先，我們改變了 `run` 函式的回傳型別為 `Result<(), Box<dyn Error>>`，此函式之前回傳的是單元型別 `()`，而它現在仍作為 `Ok` 條件內的數值。

對於錯誤型別，我們使用**特徵物件（trait object）** `Box<dyn Error>`（然後我們在最上方透過 `use` 陳述式來將 `std::error::Error` 引入作用域）。我們會在[第十七章][ch17]<!-- ignore -->討論特徵物件。現在你只需要知道 `Box<dyn Error>` 代表函式會回傳有實作 `Error` 特徵的型別，但我們不必指定回傳值的明確型別。這增加了回傳錯誤數值的彈性，其在不同錯誤情形中可能有不同的型別。`dyn` 關鍵字是「動態（dynamic）」的縮寫。

再來，我們移出了 `expect` 的呼叫並改為[第九章][ch9-question-mark]<!-- ignore -->所介紹的 `?` 運算子。所以與其對錯誤 `panic!`，`?` 會回傳當前函式的錯誤數值，並交由呼叫者處理。

第三，`run` 函式現在成功時會回傳 `Ok` 數值。我們在 `run` 函式簽名中的成功型別為 `()`，這意味著我們需要將單元型別封裝進 `Ok` 數值。`Ok(())` 這樣的語法一開始看可能會覺得有點奇怪，但這樣子使用 `()` 的確符合慣例，說明我們呼叫 `run` 只是會了它的副作用，它不會回傳我們需要的數值。

當你執行此程式時，它雖然能編譯但會顯示一個警告：

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust 告訴我們程式碼忽略了 `Result` 數值且 `Result` 數值可能代表會有錯誤發生。但我們沒有檢查是不是會發生錯誤，所以編譯器提醒我們可能要在此寫些錯誤處理的程式碼！我們現在就來修正此問題。

#### 在 `main` 中處理 `run` 回傳的錯誤

我們會用類似範例 12-10 中處理 `Config::new` 的技巧來處理錯誤，不過會有一些差別：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

我們使用 `if let` 而非 `unwrap_or_else` 來檢查 `run` 是否有回傳 `Err` 數值，並以此呼叫 `process::exit(1)`。`run` 函式沒有回傳數值，所以我們不必像處理 `Config::new` 得用 `unwrap` 取得 `Config` 實例。因為 `run` 在成功時會回傳 `()`，而我們只在乎偵測錯誤，所以我們不需要 `unwrap_or_else` 來回傳解封裝後的數值，因為它只會是 `()`。

`if let` 的本體與 `unwrap_or_else` 函式則都做一樣的事情：印出錯誤並離開。

### 將程式碼拆到函式庫 Crate

我們的 `minigrep` 專案目前看起來不錯！接下來我們要將 *src/main.rs* 檔案分開來，將一些程式碼放入 *src/lib.rs* 檔案中，讓我們可以進行測試，並讓 *src/main.rs* 檔案的負擔變得少一點。

讓我們將 `main` 以外的所有程式碼從 *src/main.rs* 移到 *src/lib.rs*：

* `run` 函式定義
* 相關的 `use` 陳述式
* `Config` 的定義
* `Config::new` 的函式定義

*src/lib.rs* 的內容應該要如範例 12-13 所示（為了簡潔，我們省略了函式本體）。注意到這還無法編譯，直到我們也修改 *src/main.rs* 成範例 12-14 為止。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

<span class="caption">範例 12-13：將 `Config` 與 `run` 移至 *src/lib.rs*</span>

我們對許多項目都使用了 `pub` 關鍵字，這包含 `Config` 與其欄位，以及其 `new` 方法，還有 `run` 函式。我們現在有個函式庫會提供公開 API 能讓我們來測試！

現在我們需要將移至 *src/lib.rs* 的程式碼引入二進制 crate 的 *src/main.rs* 作用域中，如範例 12-14 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<span class="caption">範例 12-14：在 *src/main.rs* 使用 `minigrep` 函式庫 crate</span>

我們加上 `use minigrep::Config` 這行來將 `Config` 型別從函式庫 crate 引入二進制 crate 的作用域中，然後我們使用 `run` 函式的方式是在其前面再加上 crate 的名稱。現在所有的功能都應該正常並能執行了。透過 `cargo run` 來執行程式並確保一切正常。

哇！辛苦了，不過我們為未來的成功打下了基礎。現在處理錯誤就輕鬆多了，而且我們讓程式更模組化。現在幾乎所有的工作都會在 *src/lib.rs* 中進行。

讓我們利用這個新的模組化優勢來進行些原本在就程式碼會很難處理的工作，但在新的程式碼會變得非常容易，那就是寫些測試！

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#建立自訂型別來驗證
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#錯誤處理的指導原則
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#傳播錯誤的捷徑-運算子
