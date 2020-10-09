## 透過測試驅動開發完善函式庫功能

現在我們提取邏輯到 *src/lib.rs* 並在 src/main.rs* 留下引數收集與錯誤處理的任務，現在對程式碼中的核心功能進行測試會簡單許多。我們可以使用各種引數直接呼叫函式來檢查回傳值，而不用從命令列呼叫我們的執行檔。歡迎自行對 `Config::new` 與 `run` 函式的功能寫些測試。

在此段落中，我們會在 `minigrep` 程式中利用試驅動開發（Test-driven development, TDD）來新增搜尋邏輯。此程式開發技巧遵循以下步驟：

1. 寫出一個會失敗的測試並執行它來確保它失敗的原因如你所預期。
2. 寫出或修改足夠的程式碼來讓新測試可以通過。
3. 重構你新增或變更的程式碼並確保測試仍能持續通過。
4. 重複第一步！

此流程是編寫軟體的許多方式之一，但 TDD 也有助於程式碼的設計。在寫出能通過測試的程式碼之前先寫好測試能夠協助在開發過程中維持高測試覆蓋率。

我們將用測試驅動功能的實作，而要實作的功能就是在檔案內容中找到欲搜尋的字串，並產生符合查詢字串的行數列表。我們會在一個叫做 `search` 的函式新增此功能。

### 編寫失敗的測試

讓我們移除 *src/lib.rs* 與 *src/main.rs* 中用來檢查程式行爲的 `println!` 陳述式，因爲我們不再需要它們了。然後在 *src/lib.rs* 中，我們加上 `tests` 模組與一個測試函式，如我們在[第十一章][ch11-anatomy]<!-- ignore -->所做的一樣。測試函式會指定我們希望 `search` 函式所能擁有的行爲，它會接收搜尋字串與一段要被搜尋的文字，然後它只回傳文字中包含該搜尋字串的行數。範例 12-15 展示了此測試，但還不能編譯。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">範例 12-15：建立一個我們預期 `search` 函式該有的行爲的失敗測試</span>

此測試搜尋字串 `"duct"`。而要被搜尋的文字有三行，只有一行包含 `"duct"`。我們判定 `search` 函式回傳的數值只會包含我們預期的那一行。

我們還無法執行此程式並觀察其失敗，因爲測試還無法編譯，`search` 函式根本還不存在！所以現在我們要加上足夠的程式碼讓測試可以編譯並執行，而我們要加上的事 `search` 函式的定義並永遠回傳一個空的向量，如範例 12-16 所示。然後測試應該就能編譯並失敗，因爲空向量並不符合包含 `"safe, fast, productive."` 此行的向量。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">範例 12-16：定義足夠的 `search` 函式讓我們的測試能夠編譯</span>

值得注意的是在 `search` 的簽名中需要定義一個顯式的生命週期 `'a`，並用於 `contents` 引數與回傳值。回想一下在[第十章][ch10-lifetimes]<!-- ignore -->中生命週期參數會連結引數生命週期與回傳值生命週期。在此例中，我們指明回傳值應包含字串 slice 且其會引用 `contents` 引數的 slices（而非引數 `query`）。

換句話說，我們告訴 Rust `search` 函式回傳的資料會跟傳遞給 `search` 函式的引數 `contents` 資料存活的一樣久。這點很重要！被 slice 引用的資料必須有效，這樣其引用才會有效。如果編譯器假設是在建立 `query` 而非 `contents` 的字串 slice，它的安全檢查就會不正確。

如果我們忘記詮釋生命週期並嘗試編譯此函式，我們會得到以下錯誤：

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust 無法知道這兩個引數哪個才是我們需要的，所以我們得告訴它。由於引數 `contents` 包含所有文字且我們想要回傳符合條件的部分文字，所以我們知道 `contents` 引數要用生命週期語法與回傳值做連結。

其他程式設計語言不會要求你要在簽名中連結引數與回傳值。雖然這看起來有點奇怪，但久而久之就會越來越簡單。你可能會想要將此例與第十章的[「透過生命週期驗證引用」][validating-references-with-lifetimes]<!-- ignore -->段落做比較。

現在讓我們執行測試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

很好！測試如我們所預期地失敗。接下來我們要讓測試通過！

### 寫出讓測試成功的程式碼

目前我們的測試會失敗，因爲我們永遠只回傳一個空向量。要修正並實作 `search`，我們的程式需要完成以下步驟：

* 遍歷內容的每一行。
* 檢查該行是否包含我們要搜尋的字串。
* 如果有的話，將它加入我們要回傳的數值列表。
* 如果沒有的話，不做任何事。
* 回傳符合的結果列表。

讓我們來完成每個步驟，先從遍歷每一行開始。

#### 透過 `lines` 方法來遍歷每一行

Rust 有個實用的方法能逐步處理字串的每一行，這方法就叫 `lines`，而使用方式就如範例 12-17 所示。注意此例還不法編譯。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">範例 12-17：在 `contents` 中遍歷每一行
</span>

`lines` 方法會回傳疊代器（iterator）。我們會在[第十三章][ch13-iterators]<!-- ignore -->詳細解釋疊代器，不過回想一下你在[範例 3-5][ch3-iter]<!-- ignore -->就看過疊代器的用法了，我們對疊代器使用 `for` 迴圈來對集合中的每個項目執行一些程式碼。

#### 檢查每行是否有要搜尋的字串

接著，我們要檢查目前的行數是否有包含我們要搜尋的字串。幸運的是，字串有個好用的方法叫做 `contains` 能幫我處理這件事！在 `search` 函式中加上方法 `contains` 的呼叫，如範例 12-18 所示。注意這仍然無法編譯。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">範例 12-18：增加檢查行數是否包含 `query` 字串的功能</span>

#### 儲存符合條件的行數

我們也需要有個方式能儲存包含搜尋字串的行數。爲此我們可以在 `for` 迴圈前建立一個可變向量然後對向量呼叫 `push` 方法來儲存 `line`。在 `for` 迴圈之後，我們回傳向量，如範例 12-19 所示。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">範例 12-19：儲存符合的行數讓我們可以回傳它們</span>

現在 `search` 函式應該只會回傳包含 `query` 的行數，而我們的測試也該通過。讓我們執行測試：

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

我們的測試通過了，所以我們確定它運作無誤！

在此刻之後，我們可以考慮重構搜尋函式的實作，並確保測試能通過以維持功能不變。搜尋函式的程式碼並沒有很糟，但它沒有用到疊代器中的一些實用功能優勢。我們會在[第十三章][ch13-iterators]<!-- ignore -->詳細探討疊代器之後，再回過頭來看這個例子，來看看如何改善。

#### 在 `run` 函式中使用 `search` 函式

現在 `search` 函式能夠執行且也有測試過了，我們需要從 `run` 函式呼叫 `search`。我們需要將 `config.query` 數值與 `run` 從檔案讀取到的 `contents` 傳給 `search` 函式。然後 `run` 會印出 `search` 回傳的每一行：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

我們仍會使用 `for` 迴圈來取得 `search` 回傳的每一行並顯示出來。

現在整支程式應該都能執行了！讓我們來試試看。首先用一個只會在 Emily Dickinson 的詩中回傳剛好一行的單字「frog」：

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

酷喔！現在讓我們試試看能符合多行的單字，像是「body」：

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

最後，讓我們確保使用詩中沒出現的單字來搜尋時，我們不會得到任何一行，像是「monomorphization」：

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

漂亮！我們建立了一個屬於自己的迷你經典工具，並學到了很多如何架構應用程式的知識。我們也學了一些檔案輸入與輸出、生命週期、測試與命令列解析。

爲了讓此專案更完勝，我們會簡單介紹如何使用環境變數，以及如何印出到標準錯誤（standard error），這兩項在寫命令列程式時都很實用。

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#透過生命週期驗證引用
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch12-04-testing-the-librarys-functionality.md)
> - updated: 2020-10-02
