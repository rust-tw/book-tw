## 發佈 Crate 到 Crates.io

我們已經使用過 [crates.io](https://crates.io/)<!-- ignore --> 的套件來作為我們專案的依賴函式庫，但是你也可以發佈你自己的套件來將你的程式碼提供給其他人使用。[crates.io](https://crates.io/)<!-- ignore --> 會發行套件的原始碼，所以它主要用來託管開源程式碼。

Rust 與 Cargo 有許多功能可以幫助其他人更容易找到並使用你發佈的套件。我們會介紹其中一些功能並解釋如何發佈套件。

### 寫上有幫助的技術文件註解

準確地加上套件的技術文件有助於其他使用者知道如何及何時使用它們，所以投資時間在寫技術文件上是值得的。在第三章我們提過如何使用兩條斜線 `//` 來加上 Rust 程式碼註解。Rust 還有個特別的註解用來作為技術文件，俗稱為**技術文件註解（documentation comment）**，這能用來產生 HTML 技術文件。這些 HTML 顯示公開 API 項目中技術文件註解的內容，讓對此函式庫有興趣的開發者知道如何**使用**你的 crate，而不需知道 crate 是如何**實作**的。

技術文件註解使用三條斜線 `///` 而不是兩條，並支援 Markdown 符號來格式化文字。技術文件註解位於它們對應項目的上方。範例 14-1 顯示了 `my_crate` crate 中 `add_one` 的技術文件註解。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

<span class="caption">範例 14-1：函式的技術文件註解</span>

我們在這裡加上了解釋函式 `add_one` 行為的描述、加上一個標題為 `Examples` 的段落並附上展示如何使用 `add_one` 函式的程式碼。我們可以透過執行 `cargo doc` 來從技術文件註解產生 HTML 技術文件。此命令會執行隨著 Rust 一起發佈的工具 `rustdoc`，並在 *target/doc* 目錄下產生 HTML 技術文件。

為了方便起見，你可以執行 `cargo doc --open` 來建構當前 crate 的 HTML 技術文件（以及 crate 所有依賴的技術文件）並在網頁瀏覽器中開啟結果。導向到函式 `add_one` 而你就能看到技術文件註解是如何呈現的，如圖示 14-1 所示：

<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">圖示 14-1：函式 `add_one` 的 HTML 技術文件</span>

#### 常見技術文件段落

我們在範例 14-1 使用 `# Examples` Markdown 標題來在 HTML 中建立一個標題為「Examples」的段落。以下是 crate 技術文件中常見的段落標題：

* **Panics**：該函式可能會導致恐慌的可能場合。函式的呼叫者不希望他們的程式恐慌的話，就要確保他們沒有在這些情況下呼叫該函式。
* **Errors**：如果函式回傳 `Result`，解釋發生錯誤的可能種類以及在何種條件下可能會回傳這些錯誤有助於呼叫者，讓他們可以用不同方式來寫出處理不同種錯誤的程式碼。
* **Safety**: 如果呼叫的函式是 `unsafe` 的話（我們會在第十九章討論不安全的議題），就必須要有個段落解釋為何該函式是不安全的，並提及函式預期呼叫者要確保哪些不變條件（invariants）。

大多數的技術文件註解不全都需要這些段落，但這些是呼叫程式碼的人可能有興趣瞭解的內容，你可以作為提醒你的檢查列表。

#### 將技術文件註解作為測試

在技術文件註解加上範例程式碼區塊有助於解釋如何使用你的函式庫，而且這麼做還有個額外好處：執行 `cargo test` 也會將你的技術文件視為測試來執行！在技術文件加上範例的確是最佳示範，但是如果程式碼在技術文件寫完之後變更的話，該範例可能就會無法執行了。如果我們對範例 14-1 中有附上技術文件的函式 `add_one` 執行 `cargo test` 的話，我們會看見測試結果有以下這樣的段落：

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

現在如果我們變更函式或範例使其內的 `assert_eq!` 會恐慌並再次執行 `cargo test` 的話，我們會看到技術文件測試能互相獲取錯誤，告訴我們範例與程式碼已經不同不了！

#### 包含項目結構的註解

還有另一種技術文件註解的風格為 `//!`，這是對其包含該註解的項目所加上的技術文件，而不是對註解後的項目所加上的技術文件。我通常將此技術文件註解用於 crate 源頭檔（通常為 *src/lib.rs*）或模組來對整個 crate 或模組加上技術文件。

舉例來說，如果我們希望能加上技術文件來描述包含 `add_one` 函式的 `my_crate` 目的，我們可以用  `//!` 在 *src/lib.rs* 檔案開頭加上技術文件註解，如範例 14-2 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

<span class="caption">範例 14-2：描述整個 `my_crate` crate 的技術文件</span>

注意到 `//!` 最後一行之後並沒有緊貼任何程式碼，因為我們是用 `//!` 而非 `///` 來下註解，我們是對包含此註解的整個項目加上技術文件，而不是此註解之後的項目。在此例中，包含此註解的項目為 *src/lib.rs* 檔案，也就是 crate 的源頭。這些註解會描述整個 crate。

當我們執行 `cargo doc --open`，這些註解會顯示在 `my_crate` 技術文件的首頁，位於 crate 公開項目列表的上方，如圖示 14-2 所示：

<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trpl14-02.png" class="center" />

<span class="caption">圖示 14-2：`my_crate` 的技術文件，包含描述整個 crate 的註解</span>

項目中的技術文件註解可以用來分別描述 crate 和模組。用它們來將解釋容器整體的目的有助於你的使用者瞭解該 crate 的程式碼組織架構。

### 透過 `pub use` 匯出理想的公開 API

在第七章中，我們介紹了如何使用 `mod` 關鍵字來組織我們的程式碼成模組、如何使用 `pub` 關鍵字來公開項目，以及如何使用 `use` 關鍵字在將項目引入作用域。然而在開發 crate 時的架構雖然對你來說是合理的，但對你的使用者來說可能就不是那麼合適了。你可能會希望用有數個層級的分層架構來組織你的程式碼，但是要是有人想使用你定義在分層架構裡的型別時，它們可能就很難發現這些型別的存在。而且輸入 `use my_crate::some_module::another_module::UsefulType;` 是非常惱人的，我們會希望輸入 `use my_crate::UsefulType;` 就好。

公開 API 的架構是發佈 crate 時要考量到的一大重點。使用 crate 的人可能並沒有你那麼熟悉其中的架構，而且如果你的 crate 模組分層越深的話，他們可能就難以找到他們想使用的部分。

好消息是如果你的架構**不便於**其他函式庫所使用的話，你不必重新組織你的內部架構：你可以透過使用 `pub use`選擇重新匯出（re-export）項目來建立一個不同於內部私有架構的公開架構。重新匯出會先取得某處的公開項目，再從其他地方使其公開，讓它像是被定義在其他地方一樣。

舉例來說，我們建立了一個函式庫叫做 `art` 來模擬藝術概念。在函式庫中有兩個模組：`kinds` 模組包含兩個枚舉 `PrimaryColor` 和 `SecondaryColor`；而 `utils` 模組包含一個函式 `mix`，如範例 14-3 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

<span class="caption">範例 14-3：函式庫 `art` 有兩個模組項目 `kinds` 和 `utils`</span>

圖示 14-3 顯示了此 crate 透過 `cargo doc` 產生的技術文件首頁：

<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />

<span class="caption">圖示 14-3：`art` 的技術文件首頁陳列了 `kinds` 和 `utils` 模組</span>

注意到 `PrimaryColor` 與 `SecondaryColor` 型別沒有列在首頁，而函式 `mix` 也沒有。我們必須點擊 `kinds` 與 `utils` 才能看到它們。

其他依賴此函式庫的 crate 需要使用 `use` 陳述式來將 `art` 的項目引入作用域中，並指定當前模組定義的架構。範例 14-4 顯示了從 `art` crate 使用 `PrimaryColor` 和 `mix` 項目的 crate 範例：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

<span class="caption">範例 14-4：一個使用 `art` 並匯出內部架構項目的 crate</span>

範例 14-4 中使用 `art` crate 的程式碼作者必須搞清楚 `PrimaryColor` 位於 `kinds` 模組中而 `mix` 位於 `utils` 模組中。`art` crate 的模組架構對開發 `art` crate 的開發者才比較有意義，對使用 `art` crate 的開發者來說就沒那麼重要。內部架構是為了組織 crate 的不同部分至 `kinds` 模組與 `utils` 模組，這對想要知道如何使用 `art` crate 的人來說沒有提供什麼有用的資訊。`art` crate 模組架構還容易造成混淆，因為開發者得自己搞清楚要從何處找起。而且這樣的架構也很不方便，因為開發者必須在 `use` 陳述式中指定每個模組名稱。

要從公開 API 移除內部架構，我們可以修改範例 14-3 中 `art` crate 的程式碼，並加上 `pub use` 陳述式來在頂層重新匯出項目，如範例 14-5 所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

<span class="caption">範例 14-5：加上 `pub use` 陳述式來重新匯出項目</span>

`cargo doc` 對此 crate 產生的 API 技術文件現在就會顯示與連結重新匯出的項目到首頁中，如圖示 14-4 所示。讓`PrimaryColor` 與 `SecondaryColor` 型別以及函式 `mix` 更容易被找到。

<img alt="Rendered documentation for the `art` crate with the re-exports on the front page" src="img/trpl14-04.png" class="center" />

<span class="caption">圖示 14-：`art` 的技術文件首頁會連結重新匯出的結果</span>

`art` crate 使用者仍可以看到並使用範例 14-3 的內部架構，如範例 14-4 所展示的方式，或者它們可以使用像範例 14-5 這樣更方便的架構，如範例 14-6 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

<span class="caption">範例 14-6：使用從 `art` crate 重新匯出項目的程式</span>

如果你有許多巢狀模組（nested modules）的話，在頂層透過 `pub use` 重新匯出型別可以大大提升使用 crate 的體驗。

提供實用的公開 API 架構更像是一門藝術而不只是科學，而你可以一步步來尋找最適合使用者的 API 架構。使用 `pub use` 可以給你更多組織 crate 內部架構的彈性，並將內部架構與你要呈現給使用者的介面互相解偶（decouple）。你可以觀察一些你安裝過的程式碼，看看它們的內部架構是不是不同於它們的公開 API。

### 設定 Crates.io 帳號

在你可以發佈任何 crate 之前，你需要建立一個 [crates.io](https://crates.io/)<!-- ignore --> 的帳號並取得一個 API token。請前往 [crates.io](https://crates.io/)<!-- ignore --> 的首頁並透過 GitHub 帳號來登入（GitHub 目前是必要的，但未來可能會支援其他建立帳號的方式）一旦你登入好了之後，到你的帳號設定 [https://crates.io/me/](https://crates.io/me/)<!-- ignore --> 並索取你的 API key，然後用這個 API key 來執行  `cargo login` 命令，如以下所示：

```console
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

此命令會傳遞你的 API token 給 Cargo 並儲存在本地的 *~/.cargo/credentials*。注意此 token 是個**祕密（secret）**，千萬不要分享給其他人。如果你因為任何原因分享給任何人的話，你最好撤銷掉並回到 [crates.io](https://crates.io/)<!-- ignore --> 產生新的 token。

### 新增詮釋資料到新的 Crate

現在你已經有個帳號，然後讓我們假設你有個 crate 想要發佈。在發佈之前，你需要對你的 crate 加上一些詮釋資料（metadata），也就是在 crate 的 *Cargo.toml* 檔案中 `[package]` 的段落內加上更多資料。

你的 crate 必須要有個獨特的名稱。雖然你在本地端開發 crate 時，你的 crate 可以是任何你想要的名稱。但是 [crates.io](https://crates.io/)<!-- ignore --> 上的 crate 名稱採先搶先贏制。一旦有 crate 名稱被取走了，其他人就不能再使用該名稱來發佈 crate。在嘗試發佈 crate 前，最好先在 [crates.io](https://crates.io/)<!-- ignore --> 上搜尋你想使用的名稱。如果該名稱已被其他 crate 使用，你就需要想另一個名稱，並在 *Cargo.toml* 檔案中 `[package]` 段落的 `name` 欄位使用新的名稱來發佈，如以下所示：

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

當你選好獨特名稱後，此時執行 `cargo publish` 來發佈 crate 的話，你會得到以下警告與錯誤：

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--省略--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
```

原因是因為你還缺少一些關鍵資訊：描述與授權條款是必須的，所以人們才能知道你的 crate 在做什麼以及在何種情況下允許使用。要修正此錯誤，你就需要將這些資訊加到 *Cargo.toml* 檔案中。

加上一兩句描述，它就會顯示在你的 crate 的搜尋結果中。至於 `license` 欄位，你需要給予 *license identifier value*。[Linux Foundation’s Software Package Data
Exchange (SPDX)][spdx] 有列出你可以使用的標識符數值。舉例來說，要指定你的 crate 使用 MIT 授權條款的話，就加上 `MIT` 標識符：

[spdx]: http://spdx.org/licenses/

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

如果你想使用沒有出現在 SPDX 的授權條款，你需要將該授權條款的文字儲存在一個檔案中、將該檔案加入你的專案中並使用 `license-file` 來指定該檔案名稱，而不使用 `license`。

你的專案適合使用什麼樣的授權條款超出了本書的範疇。不過 Rust 社群中許多人都會用 `MIT OR Apache-2.0` 雙授權條款作為它們專案的授權方式，這和 Rust 的授權條款一樣。這也剛好展示你也可以用 `OR` 指定數個授權條款，讓你的專案擁有數個不同的授權方式。

有了獨特名稱、版本、描述與授權條款，已經準備好發佈的 *Cargo.toml* 檔案會如以下所示：

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[Cargo 技術文件](https://doc.rust-lang.org/cargo/)還介紹了其他你可以指定的詮釋資料，讓你的 crate 更容易被其他人發掘並使用。

### 發佈至 Crates.io

現在你已經建立了帳號、儲存了 API token、選擇了 crate 的獨特名稱並指定了所需的詮釋資料，你現在已經準備好發佈了！發佈 crate 會上傳一個指定版本到 [crates.io](https://crates.io/)<!-- ignore --> 供其他人使用。

發佈 crate 時請格外小心，因為發佈是會**永遠**存在的。該版本無法被覆寫，而且程式碼無法被刪除。[crates.io](https://crates.io/)<!-- ignore --> 其中一個主要目標就是要作為儲存程式碼的永久伺服器，讓所有依賴 [crates.io](https://crates.io/)<!-- ignore --> 的 crate 的專案可以持續正常運作。允許刪除版本會讓此目標幾乎無法達成。不過你能發佈的 crate 版本不會有數量限制。

再次執行 `cargo publish` 命令，這次就應該會成功了：

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

恭喜！你現在將你的程式碼分享給 Rust 社群了，任何人現在都可以輕鬆將你的 crate 加到他們的專案中作為依賴了。

### 對現有 Crate 發佈新版本

當你對你的 crate 做了一些改變並準備好發佈新版本時，你可以變更 *Cargo.toml* 中的 `version` 數值，並再發佈一次。請使用[語意化版本規則][semver]依據你作出的改變來決定下一個妥當的版本數字。接著執行 `cargo publish` 來上傳新版本。

[semver]: https://semver.org/lang/zh-TW/

### 透過 `cargo yank` 移除 Crates.io 的版本

雖然你無法刪除 crate 之前的版本，你還是可以防止任何未來的專案加入它們作為依賴。這在 crate 版本因某些原因而被破壞時會很有用。在這樣的情況下，Cargo 支援**撤回（yanking）** crate 版本。

撤回一個版本能防止新專案用該版本作為依賴，同時允許現存依賴它的專案能夠繼續下載並依賴該版本。實際上，撤回代表所有專案的 *Cargo.lock* 都不會被破壞，且任何未來產生的 *Cargo.lock* 檔案不會使用被撤回的版本。

要撤回一個 crate 的版本，執行 `cargo yank` 並指定你想撤回的版本：

```console
$ cargo yank --vers 1.0.1
```

而對命令加上 `--undo` 的話，你還可以在復原撤回的動作，允許其他專案可以再次依賴該版本：

```console
$ cargo yank --vers 1.0.1 --undo
```

撤回**並不會**刪除任何程式碼。舉例來說，撤回此功能並不會刪除任何不小心上傳的祕密訊息。如果真的出現這種情形，你必須立即重設那些資訊。
