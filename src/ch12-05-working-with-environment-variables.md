## 處理環境變數

我們會新增一個額外的功能來改善 `minigrep`：使用者可以透過環境變數來啟用不區分大小寫的搜尋功能。我們可以將此功能設爲命令列選項並要求使用者每次需要時就要加上它，但是這次我們選擇使用環境變數。這樣一來能讓使用者設置環境變數一次就好，然後在該終端機 session 中所有的搜尋都會是不區分大小寫的。

### 寫個不區分大小寫的 `search` 函式的失敗測試

我們想新增個 `search_case_insensitive` 函式在環境變數啟用時呼叫它。我們將繼續遵守 TDD 流程，所以第一步一樣是先寫個會失敗的測試。我們會爲新函式 `search_case_insensitive` 新增一個測試，並將舊測試從 `one_result` 改名爲 `case_sensitive` 以便清楚兩個測試的差別，如範例 12-20 所示。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

<span class="caption">範例 12-20：爲準備加入的不區分大小寫函式新增個失敗測試</span>

注意到我們也修改了舊測試 `contents`。我們新增了一行使用大寫 D 的 `"Duct tape."`，當我們以區分大小寫來搜尋時，就不會符合要搜尋的 `"duct"`。這樣變更舊測試能確保我們沒有意外破壞我們已經實作好的區分大小寫的功能。此測試應該要能通過，並在我們實作不區分大小寫的搜尋時仍能繼續通過。

新的不區分大小寫的搜尋測試使用 `"rUsT"` 來搜尋。在我們準備要加入的 `search_case_insensitive` 函式中，要搜尋的 `"rUsT"` 應該要能符合有大寫 R 的 `"Rust:"` 以及 `"Trust me."` 這幾行，就算兩者都與搜尋字串有不同的大小寫。這是我們的失敗測試而且它還無法編譯，因爲我們還沒有定義 `search_case_insensitive` 函式。歡迎自行加上一個永遠回傳空向量的骨架實作，就像我們在範例 12-16 所做的一樣，然後看看測試能不能編譯過並失敗。

### 實作 `search_case_insensitive` 函式

範例 12-21 顯示的 `search_case_insensitive` 函式會與 `search` 函式幾乎一樣。唯一的不同在於我們將 `query` 與每個 `line` 都變成小寫，所以無論輸入引數是大寫還是小寫，當我們在檢查行數是否包含搜尋的字串時，它們都會是小寫。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

<span class="caption">範例 12-21：定義 `search_case_insensitive` 並在比較前將搜尋字串與行數均改爲小寫</span>

首先我們將 `query` 字串變成小寫並儲存到同名的遮蔽變數中。我們必須呼叫對要搜尋的字串 `to_lowercase`，這樣無論使用者輸入的是 `"rust"`、`"RUST"`、`"Rust"` 或 `"rUsT"`，我們都會將字串視爲 `"rust"` 並以此來不區分大小寫。雖然 `to_lowercase` 能處理基本的 Unicode，但它不會是 100% 準確的。如果我們是在寫真正的應用程式的話，我們需要處理更多條件，但在此段落是爲了理解環境變數而非 Unicode，所以我們維持這樣寫就好。

注意到 `query` 現在是個 `String` 而非字串切片，因爲呼叫 `to_lowercase` 會建立新的資料而非引用現存的資料。假設要搜尋的字串是 `"rUsT"` 的話，該字串切片並沒有包含小寫的 `u` 或 `t` 能讓我們來使用，所以我們必須分配一個包含 `"rust"` 的新 `String`。現在當我們將 `query` 作爲引數傳給 `contains` 方法時，我們需要加上「&」，因爲 `contains` 所定義的簽名接收的是一個字串切片。

接著，在我們檢查是否包含小寫的 `query` 前，我們對每個 `line` 加上 `to_lowercase` 的呼叫。現在我們將 `line` 和 `query` 都轉換成小寫了。我們可以不區分大小寫來找到符合的行數。

讓我們來看看實作是否能通過測試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

很好！測試通過。現在讓我們從 `run` 函式呼叫新的 `search_case_insensitive` 函式。首先，我們要在 `Config` 中新增一個配置選項來切換區分大小寫與不區分大小寫之間的搜尋。新增此欄位會造成編譯錯誤，因爲我們還沒有初始化該欄位：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

注意到我們新增了 `case_sensitive` 欄位並存有布林值。接著，我們需要 `run` 函式檢查 `case_sensitive` 欄位的數值並以此決定要呼叫 `search` 函式或是 `search_case_insensitive` 函式，如範例 12-22 所示。不過目前還無法編譯。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

<span class="caption">範例 12-22：依據 `config.case_sensitive` 的數值來呼叫 `search` 或 `search_case_insensitive`</span>

最後，我們需要檢查環境變數。處理環境變數的函式位於標準函式庫中的 `env` 模組中，所以我們可以在 *src/lib.rs* 最上方加上 `use std::env;` 來將該模組引入作用域。然後我們使用 `env` 模組中的 `var` 函式來檢查一個叫做 `CASE_INSENSITIVE` 的環境變數，如範例 12-23 所示。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

<span class="caption">範例 12-23：檢查環境變數 `CASE_INSENSITIVE`</span>

我們在此建立了一個新的變數 `case_sensitive`。要設置其數值，我們可以呼叫 `env::var` 函式並傳入環境變數 `CASE_INSENSITIVE` 的名稱。`env::var` 函式會回傳 `Result`，如果有設置環境變數的話，這就會是包含環境變數數值的成功 `Ok`變體；如果環境變數沒有設置的話，這就會回傳 `Err` 變體。

我們在 `Result` 使用 `is_err` 方法來檢查是否爲錯誤，如果是的話就代表沒有設置，也意味著它*該*使用區分大小寫的搜尋。如果 `CASE_INSENSITIVE` 環境變數有設置成任何數值的話，`is_err` 會回傳否，所以程式就會進行不區分大小寫的搜尋。我們不在乎環境變數的*數值*，只在意它有沒有被設置而已，所以我們使用 `is_err` 來檢查而非使用 `unwrap`、`expect` 或其他任何我們看過的 `Result` 方法。

我們將變數  `case_sensitive` 的數值傳給 `Config` 實例，讓 `run` 函式可以讀取該數值並決定該呼叫 `search` 還是 `search_case_insensitive`，如範例 12-22 所實作的一樣。

讓我們試看看吧！首先，我們先不設置環境變數並執行程式來搜尋 `to`，任何包含小寫單字「to」的行數都應要符合：

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

看起來運作仍十分正常！現在，讓我們設置 `CASE_INSENSITIVE` 爲 `1` 並執行程式來搜尋相同的字串 `to`。

如果你使用的是 PowerShell，你需要將設置變數與執行程式分爲不同的命令：

```console
PS> $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
```

這會在你的 shell session 中設置 `CASE_INSENSITIVE`。它可以透過 `Remove-Item` cmdlet 來取消設置：

```console
PS> Remove-Item Env:CASE_INSENSITIVE
```

我們應該會得到包含可能有大寫的「to」的行數：

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
CASE_INSENSITIVE=1 cargo run to poem.txt
can't extract because of the environment variable
-->

```console
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

太好了，我們也取得了包含「To」的行數！我們的 `minigrep` 程式現在可以進行不區分大小寫的搜尋並以環境變數配置。現在你知道如何使用命令列引數或環境變數來管理設置選項了。

有些程式允許同時使用引數*與*環境變數來配置。在這種情況下，程式會決定各種選項的優先層級。你想要練習的話，嘗試使用命令列引數與環境變數來控制不區分大小寫的選項。並在程式執行時，其中一個設置爲區分大小寫，而另一個爲不區分大小寫時，自行決定該優先使用命令列引數還是環境變數。

`std::env` 模組還包含很多處理環境變數的實用功能，歡迎查閱其官方文件來瞭解有哪些可用。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch12-05-working-with-environment-variables.md)
> - updated: 2020-10-03
