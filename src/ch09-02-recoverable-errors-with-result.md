## `Result` 與可復原的錯誤

大多數的錯誤沒有嚴重到需要讓整個程式停止執行。有時候當函式失敗時，你是可以輕易理解並作出反應的。舉例來說，如果你嘗試開啟一個檔案，但該動作卻因為沒有該檔案而失敗的話，你可能會想要建立檔案，而不是終止程序。

回憶一下第二章的[「使用 `Result` 型別處理可能的錯誤」][handle_failure]<!-- ignore -->提到 `Result` 枚舉的定義有兩個變體 `Ok` 和 `Err`，如以下所示：

[handle_failure]: ch02-00-guessing-game-tutorial.html#使用-result-型別處理可能的錯誤

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` 和 `E` 是泛型型別參數，我們會在第十章深入討論泛型。你現在需要知道的是 `T` 代表我們在成功時會在 `Ok` 變體回傳的型別，而 `E` 則代表失敗時在 `Err` 變體會回傳的錯誤型別。因為 `Result` 有這些泛型型別參數，我們可以將 `Result` 型別和標準函式庫運用到它的函式用在許多不同場合，讓成功與失敗時回傳的型別不相同。

讓我們呼叫一個可能會失敗的函式並回傳 `Result` 型別。在範例 9-3 我們嘗試開啟一個檔案。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">範例 9-3：嘗試開啟一個檔案</span>

我們怎麼知道 `File::open` 會回傳 `Result`呢？我們可以查閱[標準函式庫的 API 技術文件](https://doc.rust-lang.org/std/index.html)<!-- ignore -->，或者我們也可以親自去問編譯器！如果我們給予 `f` 一個型別詮釋，但是我們知道它和函式回傳值**並不**相同，接著嘗試編譯程式碼的話，編譯器會告訴我們型別不符。錯誤訊息會告訴我們 `f` **該有**何種型別。讓我們試試看！我們知道 `File::open` 的回傳型別不是 `u32`，所以讓我們改變 `let f` 成這樣：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

嘗試編譯的話會給我們以下輸出結果：

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

這告訴我們函式 `File::open` 的回傳型別為 `Result<T, E>`。泛型參數 `T` 在此已經被指明成功時會用到的型別 `std::fs::File`，也就是檔案的控制代碼（handle）。用於錯誤時的 `E` 型別則是 `std::io::Error`。

這樣的回傳型別代表 `File::open` 的呼叫在成功時會回傳我們可以讀寫的檔案控制代碼，但該函式呼叫也可能失敗。舉例來說，該檔案可能會不存在，或者我們沒有檔案的存取權限。`File::open` 需要有某種方式能告訴我們它的結果是成功或失敗，並回傳檔案控制代碼或是錯誤資訊。這樣的資訊正是 `Result` 枚舉想表達的。

如果 `File::open` 成功的話，變數 `f` 的數值就會獲得包含檔案控制代碼的 `Ok` 實例。如果失敗的話，`f` 的值就會是包含為何產生該錯誤的資訊的 `Err` 實例。

我們需要讓範例 9-3 的程式碼依據 `File::open` 回傳不同的結果採取不同的動作。範例 9-4 展示了其中一種處理 `Result` 的方式，我們使用第六章提到的 `match` 表達式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">範例 9-4：使用 `match` 表達式來處理回傳的 `Result` 變體</span>

和 `Option` 枚舉一樣，`Result` 枚舉與其變體都會透過 prelude 引入作用域，所以我們不需要指明 `Result::`，可以直接在 `match` 的分支中使用 `Ok` 和 `Err` 變體。

我們在此告訴 Rust 結果是 `Ok` 的話，就回傳 `Ok` 變體中內部的 `file`，然後我們就可以將檔案控制代碼賦值給變數 `f`。在 `match` 之後，我們就可以適用檔案控制代碼來讀寫。

`match` 的另一個分支則負責處理我們從 `File::open` 中取得的 `Err` 數值。在此範例中，我們選擇呼叫 `panic!` 巨集。如果檔案 *hello.txt* 不存在我們當前的目錄的話，我們就會執行此程式碼，接著就會看到來自 `panic!` 巨集的輸出結果：

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

如往常一樣，此輸出告訴我們哪裡出錯了。

### 配對不同種的錯誤

範例 9-4 的程式碼不管 `File::open` 為何失敗都會呼叫 `panic!`。我們希望做的是依據不同的錯誤原因採取不同的動作，如果 `File::open` 是因為檔案不存在的話，我們想要建立檔案並回傳新檔案的控制代碼。如果 `File::open` 是因為其他原因失敗的話，像是我們沒有開啟檔案的權限，我們仍然要像範例 9-4 這樣呼叫 `panic!`。範例 9-5 就這樣對 `match` 表達式加了更多條件。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">範例 9-5：針對不同種類的錯誤採取不同動作</span>

`File::open` 在 `Err` 變體的回傳型別為 `io::Error`，這是標準函式庫提供的結構體。此結構體有個 `kind` 方法讓我們可以取得 `io::ErrorKind` 數值。標準函式庫提供的枚舉 `io::ErrorKind` 有從 `io` 運算可能發生的各種錯誤。我們想處理的變體是 `ErrorKind::NotFound`，這指的是我們嘗試開啟的檔案還不存在。所以我們對 `f` 配對並在用 `error.kind()` 繼續配對下去。

我們從內部配對檢查 `error.kind()` 的回傳值是否是 `ErrorKind` 枚舉中的 `NotFound` 變體。如果是的話，我們就嘗試使用 `File::create` 建立檔案。不過 `File::create` 也可能會失敗，所以我們需要第二個內部 `match` 表達式來處理。如果檔案無法建立的話，我們就會印出不同的錯誤訊息。第二個分支的外部 `match` 分支保持不變，如果程式遇到其他錯誤的話就會恐慌。

我們用的 `match` 的確有點多！`match` 表達式雖然很實用，不過它的行為非常基本。在第十三章你會學到閉包（closure），`Result<T, E>` 型別有很多接收閉包並採用 `match` 實作的方法。使用那些方法可以讓你的程式碼更簡潔。更熟練的 Rustacean 可能會像這樣寫範例 9-5 的程式數碼：

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-03-closures/src/main.rs}}
```

雖然此程式碼的行為和範例 9-5 一樣，但他沒有包含任何 `match` 表達式而且更易閱讀。當你讀完第十三章後，別忘了回來看看此範例，並查閱標準函式庫中的 `unwrap_or_else` 方法。除此方法以外，還有更多方法可以來解決處理錯誤時龐大的 `match` 表達式。

### 錯誤發生時產生恐慌的捷徑：`unwrap` 與 `expect`

雖然 `match` 已經足以勝任指派的任務了，但它還是有點冗長，而且可能無法正確傳遞錯誤的嚴重性。`Result<T, E>` 型別有非常多的輔助方法來執行不同的任務。其中一個方法就是 `unwrap`，這是和我們在範例 9-4 所寫的 `match` 表達式一樣，擁有類似效果的捷徑方法。如果 `Result` 的值是 `Ok` 變體，`unwrap`會回傳 `Ok` 裡面的值；如果 `Result` 是 `Err` 變體的話，`unwrap` 會呼叫 `panic!` 巨集。以下是使用 `unwrap` 的方式：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

如果我們沒有 *hello.txt* 這個檔案並執行此程式碼的話，我們會看到從 `unwrap` 方法所呼叫的 `panic!` 回傳訊息：

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

還有另一個方法 `expect` 和 `unwrap` 類似，不過能讓我們選擇 `panic!` 回傳的錯誤訊息。使用 `expect` 而非 `unwrap` 並提供完善的錯誤訊息可以表明你的意圖，讓追蹤恐慌的源頭更容易。`expect` 的語法看起來就像這樣：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

我們使用 `expect` 的方式和 `unwrap` 一樣，不是回傳檔案控制代碼就是呼叫 `panic!` 巨集。使用 `expect` 呼叫 `panic!` 時的錯誤訊息會是我們傳遞給 `expect` 的參數，而不是像 `unwrap` 使用 `panic!` 預設的訊息。訊息看起來就會像這樣：

```text
thread 'main' panicked at '開啟 hello.txt 失敗: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

由於此錯誤訊息指明了我們想表達的訊息「開啟 hello.txt 失敗」，我們比較能知道此錯誤訊息是從哪裡發生的。如果我們在多處使用 `unwrap`，我們會需要一些時間才能理解 `unwrap` 是從哪裡引發恐慌的，因為 `unwrap` 很可能會顯示相同的訊息。

### 傳播錯誤

當你在寫某函式實作時，要是它的呼叫的程式碼可能會失敗，與其直接在此函式處理錯誤，你可以回傳錯誤給呼叫此程式的程式碼，由它們決定如何處理。這稱之為**傳播**（propagating）錯誤並讓呼叫者可以有更多的控制權，因為比起你程式碼當下的內容，回傳的錯誤可能提供更多資訊與邏輯以利處理。

舉例來說，範例 9-6 展示了一個從檔案讀取使用者名稱的函式。如果檔案不存在或無法讀取的話，此函式會回傳該錯誤給呼叫此函式的程式碼。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">範例 9-6：使用 `match` 回傳錯誤給呼叫者的函式</span>

此函式還能再更簡化，但我們要先繼續手動處理來進一步探討錯誤處理，最後我們會展示最精簡的方式。讓我們先看看此函式的回傳型別 `Result<String, io::Error>`。這代表此函式回傳的型別為 `Result<T, E>`，而泛型型別 `T` 已經指明為實際型別 `String` 然後泛型型別 `E` 也已經指明為實際型別 `io::Error`。如果函式正確無誤的話，程式碼會呼叫此函式並收到擁有 `String` 的 `Ok` 數值。如果程式遇到任何問題的話，呼叫此函式的程式碼就會獲得擁有包含相關問題發生資訊的 `io::Error` 實例的 `Err` 數值。我們選擇 `io::Error` 作為函式的回傳值是因為它正是 `File::open` 函式和 `read_to_string` 方法失敗時的回傳的錯誤型別。

函式本體從呼叫 `File::open` 開始，然後我們使用 `match` 回傳 `Result` 數值，就和範例 9-4 的 `match` 類似。如果 `File::open` 成功的話，變數 `file` 中的檔案控制代碼賦值給可變變數 `f` 並讓函式繼續執行下去。但在 `Err` 的情形時，與其呼叫 `panic!`，我們使用 `return` 關鍵字來讓函式提早回傳，並將 `File::open` 的錯誤值，也就是模式中的變數 `e`，作為此函式的錯誤值回傳給呼叫的程式碼。

所以如果我們在 `f` 有拿到檔案控制代碼的話，接著函式就會在變數 `s` 建立新的 `String` 並對檔案控制代碼 `f` 呼叫 `read_to_string` 方法來讀取檔案內容至 `s`。`read_to_string` 也會回傳 `Result` 因為它也可能失敗，就算 `File::open` 是執行成功的。所以我們需要另一個 `match` 來處理該 `Result`，如果 `read_to_string` 成功的話，我們的函式就是成功的，然後在 `Ok` 回傳 `s` 中該檔案的使用者名稱。如果 `read_to_string` 失敗的話，我們就像處理 `File::open` 的 `match` 一樣回傳錯誤值。不過我們不需要顯式寫出 `return`，因為這是函式中的最後一個表達式。

呼叫此程式碼的程式就會需要處理包含使用者名稱的 `Ok` 數值以及包含 `io::Error` 的 `Err` 數值。我們不會知道呼叫此程式碼的人會如何處理這些數值。舉例來說，如果呼叫此程式碼而獲得錯誤的話，它可能選擇呼叫 `panic!` 讓程式崩潰，或者使用預設的使用者名稱從檔案以外的地方尋找該使用者。所以我們傳播所有成功或錯誤的資訊給呼叫者，讓它們能妥善處理。

這樣傳播錯誤的模式是非常常見的，所以 Rust 提供了 `?` 來簡化流程。

#### 傳播錯誤的捷徑：`?` 運算子

範例 9-7 是另一個 `read_username_from_file`的實作，擁有和範例 9-6 一樣的效果，不過這次使用了 `?` 運算子。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">範例 9-7：使用 `?` 運算子回傳錯誤給呼叫者的函式</span>

定義在 `Result` 數值後的 `?` 運作方式幾乎與範例 9-6 的 `match` 表達式處理 `Result` 的方式一樣。如果 `Result` 的數值是 `Ok` 的話，`Ok` 內的數值就會從此表達式回傳，然後程式就會繼續執行。如果數值是 `Err` 的話，`Err` 就會使用 `return` 關鍵字作為整個函式的回傳值回傳，讓錯誤數值可以傳遞給呼叫者的程式碼。

不過範例 9-6 的 `match` 表達式做的事和 `?` 運算子做的事還是有不同的地方：`?` 運算子呼叫所使用的錯誤數值會傳遞到 `from` 函式中，這是定義在標準函式庫的 `From` 特徵中，用來將錯誤從一種型別轉換另一種型別。當 `?` 運算子呼叫 `from` 函式時，接收到的錯誤型別會轉換成目前函式回傳值的錯誤型別。這在當函式要回傳一個錯誤型別來代表所有函式可能的失敗是很有用的，即使可能會失敗的原因有很多種。只要有 `impl From<OtherError> for ReturnedError` 來定義特徵的 `from` 函式的轉換，`?` 運算子就能自動呼叫並處理 `from` 函式。

在範例 9-7 中，在 `File::open` 的結尾中 `?` 回傳 `Ok` 中的數值給變數 `f`。如果有錯誤發生時，`?` 運算子會提早回傳整個函式並將 `Err` 的數值傳給呼叫的程式碼。同理也適用在呼叫 `read_to_string` 結尾的 `?`。

`?` 運算子可以消除大量樣板程式碼並讓函式實作更簡單。我們還可以再進一步將方法直接串接到 `?` 後來簡化程式碼，如範例 9-8 所示。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">範例 9-8：在 `?` 運算子後方串接方法呼叫</span>

我們將建立新 `String` 的變數 `s` 移到函式的開頭，這部分沒有任何改變。再來與建立變數 `f` 的地方不同的是，我們直接將 `read_to_string` 串接到 `File::open("hello.txt")?` 的結果後方。我們在 `read_to_string` 呼叫的結尾還是有 `?`，然後我們還是在 `File::open` 和 `read_to_string` 成功沒有失敗時，回傳包含 `s` 的 `Ok` 數值。函式達成的效果仍然與範例 9-6 與 9-7 相同。這只是一個比較不同但慣用的寫法。

說到此函式不同的寫法，範例 9-9 展示了另一個更短的寫法。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">範例 9-9：使用 `fs::read_to_string` 而不是開啟檔案後才讀取</span>

讀取檔案至字串中算是個常見動作，所以 Rust 提供了一個方便的函式 `fs::read_to_string` 來開啟檔案、建立新的 `String`、讀取檔案內容、將內容放入該 `String` 並回傳它。不過使用 `fs::read_to_string` 就沒有機會讓我們來解釋所有的錯誤處理，所以我們一開始才用比較長的寫法。

#### `?` 運算子可以用哪裡？

`?` 運算子只能用在有函式的回傳值相容於 `?` 使用的值才行。這是因為 `?` 運算子會在函式中提早回傳數值，就像我們在範例 9-6 那樣用 `match` 表達式提早回傳一樣。在範例 9-6 中，`match` 使用的是 `Result` 數值，函式的回傳值必須是 `Result` 才能相容於此 `return`。

讓我們看看在範例 9-10 的 `main` 函式中回傳值為 `()`，如果我們使用 `?` 運算子會發生什麼事：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">範例 9-10：嘗試在回傳 `()` 的 `main` 函式中使用 `?` 會無法編譯</span>

此程式法會開啟檔案，所以可能會失敗。`?` 運算子會拿到 `File::open` 回傳的 `Result` 數值，但是此 `main` 函式的回傳值為`()` 而非 `Result`。當我們編譯此程式碼時，我們會得到以下錯誤訊息：

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

此錯誤告訴我們只能在回傳型別為 `Result` 或 `Option` 或其他有實作 `FromResidual` 的型別的函式才能使用 `?` 運算子。要修正此錯誤的話，你有兩種選擇。其中一種是如果你沒有任何限制的話，你可以將函式回傳值變更成 `Result<T, E>`。另一種則是依照可能的情境使用 `match` 或 `Result<T, E>` 其中一中方法來處理 `Result<T, E>`。

錯誤訊息還提到了 `?` 也能用在 `Option<T>` 的數值。就像 `?` 能用在 `Result`一樣，你只能在有回傳 `Option` 的函式中，對 `Option` 的值使用 `?`。在 `Option<T>` 呼叫 `?` 的行為與在 `Result<T, E>` 上呼叫類似：如果數值為　`None`，`None` 就會在函式該處被提早回傳；如果數值為 `Some`，`Some` 中的值就會是表達式的結果數值，且程式會繼續執行下去。以下範例 9-11 的函式會尋找給予文字的第一行中最後一個字元：

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">範例 9-11：`Option<T>` 的數值上使用在 `?` 運算子</span>

此函式會回傳 `Option<char>`，因為它可能會在此真的找到字元，或者可能根本沒有半個字存在。此程式碼接受引數 `text` 字串切片，並呼叫它的 `lines` 方法，這會回傳一個遍歷字串每一行的疊代器。因為此函式想要的是第一行，它對疊代器只呼叫 `next` 來取得疊代器的第一個數值。如果 `text` 是空字串的話，這裡 `next` 的呼叫就會回傳 `None`。如果是這種情況的話，我們這裡就可以使用 `?` 來中斷 `last_char_of_first_line` 並回傳 `None`。如果 `text` 不是空字串的話，`next` 會用 `Some` 數值來回傳 `text` 的第一行字串切片。

`?` 會取出字串切片，然後我們可以對字串切片呼叫 `chars` 來取得它的字元疊代器。我們在意的是第一行的最後一個字元，所以我們呼叫 `last` 來取得此字元疊代器的最後一個值。這也是個 `Option` 因為第一行可能是個空字串。如果 `text` 開頭就換行，但在下一行有字元的話，它可能就會是 `"\nhi"`。不過如果第一行真的有最後一個字元的話，它就會回傳 `Some` 變體。在這過程中的 `?` 運算子讓我們能簡潔地表達此邏輯，並讓此函式只用一行就能實作出來。如果我們對 `Option` 無法使用 `?` 運算子的話，我們使用更多方法呼叫或 `match` 表達式才能實作此邏輯。

請注意你可以在有回傳 `Result` 的函式對 `Result` 的值使用 `?` 運算子，你可以在有回傳 `Option` 的函式對 `Option` 的值使用 `?` 運算子，但你無法混合使用。`?` 運算子無法自動轉換 `Result` 與 `Option` 之間的值。在這種狀況下會需要顯式表達，`Result` 的話有提供 `ok` 方法，`Option` 的話有提供 `ok_or` 方法。

目前為止，所有我們使用過的 `main` 函式都是回傳 `()`。`main` 是個特別的函式，因為它是可執行程式的入口點與出口點，而要讓程式可預期執行的話，它的回傳型別就得要有些限制。用 C 語言寫的執行檔在退出時會回傳整數，而 Rust 執行檔也遵循這個規則：程式成功退出的話會回傳整數 `0`，而程式退出錯誤的話則會回傳不是 `0` 的其他整數。當 `main` 回傳 `()` 時，Rust 執行檔會在 `main` 回傳時回傳 `0` ，而如果程式在 `main` 結束前恐慌的話則會回傳非零數值。

`main` 可以擁有的另一種回傳型別為 `Result<(), E>`。範例 9-12 取自範例 9-10，不過我們更改 `main` 的回傳型別為`Result<(), Box<dyn Error>>`，並在結尾的回傳數值加上 `Ok(())`。這樣的程式碼是能編譯的：

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">範例 9-12：將 `main` 改成回傳 `Result<(), E>` 就能允許在 `Result` 數值上使用 `?` 運算子</span>

`Box<dyn Error>` 型別使用了特徵物件（trait object）我們會在第十七章的[「允許不同型別數值的特徵物件」][trait-objects]<!-- ignore -->討論到。現在你可以將 `Box<dyn Error>` 視為它是「任何種類的錯誤」。這樣 `main` 函式中的 `Result` 使用 `?` 就允許了，因為現在 `Err` 數值可以被提早回傳。當 `main` 函式回傳 `Result<(), E>` 時，如果 `main` 回傳 `Ok(())` 的話，執行檔就會用 `0` 退出；如果 `main` 回傳 `Err` 數值的話，就會用非零數值退出。

`main` 之所以能夠退出是因為實作了 [`std::process::Termination`][termination]<!-- ignore --> 特徵。在本段落撰寫時，`Termination` 仍然是 Nightly Rust 中的不穩定特徵，所以你還無法在 Stable Rust 中對你的型別實作它。不過也許未來的某一天是可以的！

現在我們已經討論了呼叫 `panic!` 與回傳 `Result` 的細節。現在讓我們回到何時該使用何種辦法的主題上吧。

[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: ../std/process/trait.Termination.html
