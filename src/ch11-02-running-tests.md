## 控制程式如何執行

就像 `cargo run` 會編譯你的程式碼並執行產生的二進制檔案，`cargo test` 會在測試模式編譯你的程式碼並執行產生的測試二進制檔案。你可以指定命令列選項來改變 `cargo test` 的預設行為。舉例來說 `cargo test` 預設行為產生的二進制執行檔會平行執行所有測試並獲取測試執行時產生的輸出，讓測試各自的輸出結果不會顯示出來，以更容易讀取相關測試的結果。

有些命令列選項用於 `cargo test` 而有些則用於產生的測試二進制檔案。要分開這兩種引數，你可以先寫奧用於 `cargo test` 的引數然後加上 `--` 分隔線來區隔要用於測試二進制檔案的引數。執行 `cargo test --help` 可以顯示你能用在 `cargo test` 的選項，而執行 `cargo test -- --help` 在則會顯示你在 `--` 之後能用的選項。

### 平行或接續執行測試

當你執行數個測試時，它們預設會使用執行緒（thread）來平行執行。這樣測試可以更快完成，讓你可以從你或其他人的程式碼更快獲得回饋。因為測試是同時一起執行的，請確保你的測試並不依賴其他測試或是共享的狀態。這包含共享環境，像是目前的工作目錄或是環境變數。

舉例來說，假設你的每個測試都會執行些程式碼會在硬碟上產生一個檔案叫做 *test-output.txt* 並將一些資料寫入檔案中。然後每個測試讀取檔案中的資料，並判定該檔案有沒有包含特定的值，而這個值在每個測試都不相同。因為測試同時執行，其中的測試可以能覆蓋其他測試寫入與讀取的內容。這樣其他測試就會失敗，並不是因為程式碼不正確，而是因為平行執行時該測試會被其他測試所影響。其中一個解決辦法是確保每個測試都寫入不同的檔案，或者也可以選擇一次只執行一個測試。

如果你不想平行執行測試，或者你想要能更加掌控使用的執行緒數量，你可以傳遞 `--test-threads` 的選項以及你希望在測試執行檔使用的執行緒數量。請看一下以下範例：

```console
$ cargo test -- --test-threads=1
```

我們將測試執行緒設為 `1`，告訴程式不要做任何平行化。使用一條執行緒執行測試會比平行執行它們還來的久，但是如果測試有共享狀態的話，它們就會不互相影響到對方了。

### 顯示函式輸出結果

如果測試通過的話，Rust 的測試函式庫預設會獲取所有印出的標準輸出。舉例來說，如果我們在測試中呼叫 `println!` 然後測試通過的話，我們不會在終端機看到 `println!` 的輸出，我們只會看到一行表達測試通過的訊息。如果測試失敗，我們才會看到所有印出的標準輸出與失敗訊息。

舉例來說，範例 11-10 有個蠢蠢的函式只會印出它的參數並回傳 10，以及一個會通過的測試與一個會失敗的測試。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,panics
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs:here}}
```

<span class="caption">範例 11-10：測試會呼叫 `println!` 的函式</span>

當我們使用 `cargo test` 執行這些程式時，我們會看到以下輸出結果：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

注意到此輸出結果我們看不到 `我得到的數值為 4`，這是當測試通過時印出的訊息。這個輸出被獲取走了。而測試會失敗的標準輸出 `我得到的數值為 8` 則會出現在測試總結輸出的段落上，並同時顯示錯誤發生的原因。

如果我們希望在測試通過時也能看到印出的數值，我們可以用 `--show-output` 告訴 Rust 也在成功的測試顯示輸出結果。

```console
$ cargo test -- --show-output
```

當我們使用 `--show-output` 再次執行範例 11-10 的話，我們就能看到以下輸出：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### 透過名稱來執行部分測試

有時執行完整所有的測試會很花時間。如果你正專注於程式碼的特定部分，你可能會想要只執行與該程式碼有關的測試。你可以向 `cargo test` 傳遞你想要執行的測試名稱作為引數。

為了解釋如何執行部分測試，我們將為 `add_two` 函式建立三個測試，如範例 11-11 所示，然後選擇其中一個執行。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

<span class="caption">範例 11-11：三個名稱不同的測試</span>

如果我們沒有傳遞任何引數來執行測試的話，如我們前面看過的一樣，所有測試會平行執行：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### 執行單獨一個測試

我們可以傳遞任何測試函式的名稱給 `cargo test` 來只執行該測試：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

只有名稱為 `one_hundred` 的測試會執行，其他兩個的名稱並不符合。測試輸出會在總結的最後顯示 `2 filtered out` 告訴我們除了命令列執行的測試以外，還有更多其他測試。

我們無法用此方式指定多個測試名稱，只有第一個傳給 `cargo test` 有用。但我們有其他方式能執行數個測試。

#### 過濾執行數個測試

我們可以指定部分測試名稱，然後任何測試名稱中有相符的就會被執行。舉例來說，因為我們有兩個測試的名稱都包含 `add`，我們可以透過執行 `cargo test add` 來執行這兩個測試：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

此命令會執行所有名稱中包含 `add` 的測試，並過濾掉 `one_hundred` 的測試名稱。另外測試所在的模組也屬於測試名稱中，所以我們可以透過過濾模組名稱來執行該模組的所有測試。

### 忽略某些測試除非特別指定

有時候有些特定的測試執行會花非常多時間，所以你可能希望在執行 `cargo test` 時能排除它們。與其列出所有你想要的測試作為引數，你可以在花時間的測試前加上 `ignore` 屬性詮釋來排除它們，如以下所示：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

對於想排除的測試，我們在 `#[test]` 之後我們加上 `#[ignore]`。現在當我們執行我們的測試時，`it_works` 會執行但 `expensive_test` 就不會：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`expensive_test` 函式會列在 `ignored`，如果我們希望只執行被忽略的測試，我們可以使用 `cargo test -- --ignored`：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

透過控制哪些測試能執行，你能夠確保快速執行 `cargo test`。當你有時間能夠執行 `ignored` 的測試時，你可以執行 `cargo test -- --ignored` 來等待結果。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch11-02-running-tests.md)
> - updated: 2020-09-17
