## `Result` 與可復原的錯誤

大多數的錯誤沒有嚴重到需要讓整個程式停止執行。有時後當函式失敗時，你是可以輕易理解並作出反應的。舉例來說，如果你嘗試開啟一個檔案，但該動作卻因爲沒有該檔案而失敗的話，你可能會想要建立檔案，而不是終止程序。

回憶一下第二章的[“使用 `Result` 型別處理可能的錯誤”][handle_failure]<!-- ignore -->提到 `Result` 枚舉的定義有兩個變體 `Ok` 和 `Err`，如以下所示：

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` 和 `E` 是泛型型別參數，我們會在第十章深入討論泛型。你現在需要知道的事 `T` 代表我們在成功時會在 `Ok` 變體回傳的型別，而 `E` 則代表失敗時在 `Err` 變體會回傳的錯誤型別。因爲 `Result` 有這些泛型型別參數，我們可以將 `Result` 型別和標準函式庫運用到它的函式用在許多不同場合，讓成功與失敗時回傳的型別不相同。

讓我們呼叫一個可能會失敗的函式並回傳 `Result` 型別。在範例 9-3 我們嘗試開啟一個檔案。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">範例 9-3：嘗試開啟一個檔案</span>

我們怎麼知道 `File::open` 會回傳 `Result`呢？我們可以查閱[標準函式庫的 API 技術文件](../std/index.html)<!-- ignore -->，或者我們也可以親自去問編譯器！如果我們給予 `f` 一個型別詮釋，但是我們知道它和函式回傳值*並不*相同，接著嘗試編譯程式碼的話，編譯器會告訴我們型別不服。錯誤訊息會告訴我們 `f` *該有*何種型別。讓我們試試看！我們知道 `File::open` 的回傳型別不是 `u32`，所以讓我們改變 `let f` 成這樣：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

嘗試編譯的話會給我們以下輸出結果：

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

這告訴我們函式 `File::open` 的回傳型別爲 `Result<T, E>`。泛型參數 `T` 在此已經被指明成功時會用到的型別 `std::fs::File`，也就是檔案的句柄（handle）。用於錯誤時的 `E` 型別則是 `std::io::Error`。

這樣的回傳型別代表 `File::open` 的呼叫在成功時會回傳我們可以讀寫的檔案句柄，但該函式呼叫也可能失敗。舉例來說，該檔案可能會不存在，或者我們沒有檔案的存取權限。`File::open` 需要有某種方式能告訴我們它的結果是成功或失敗，並回傳檔案句柄或是錯誤資訊。這樣的資訊正是 `Result` 枚舉想表達的。

如果 `File::open` 成功的話，變數 `f` 的數值就會獲得包含檔案句柄的 `Ok` 實例。如果失敗的話，`f` 的值就會是包含爲何產生該錯誤的資訊的 `Err` 實例。

我們需要讓範例 9-3 的程式碼依據 `File::open` 回傳不同的結果採取不同的動作。範例 9-4 展示了其中一種處理 `Result` 的方式，我們使用第六章提到的 `match` 表達式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">範例 9-4：使用 `match` 表達式來處理回傳的 `Result` 變體</span>

和 `Option` 枚舉一樣，`Result` 枚舉與其變體都會透過 prelude 引入作用域，所以我們不需要指明 `Result::`，可以直接在 `match` 的分支中使用 `Ok` 和 `Err` 變體。

我們在此告訴 Rust 結果是 `Ok` 的話，就回傳 `Ok` 變體中內部的 `file`，然後我們就可以將檔案句柄賦值給變數 `f`。在 `match` 之後，我們就可以適用檔案句柄來讀寫。

`match` 的另一個分支則負責處理我們從 `File::open` 中取得的 `Err` 數值。在此範例中，我們選擇呼叫 `panic!` 巨集。如果檔案 *hello.txt* 不存在我們當前的目錄的話，我們就會執行此程式碼，接著就會看到來自 `panic!` 巨集的輸出結果：

```text
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

如往常一樣，此輸出告訴我們哪裡出錯了。

### 配對不同種的錯誤

範例 9-4 的程式碼不管 `File::open` 爲何失敗都會呼叫 `panic!`。我們希望做的是依據不同的錯誤原因採取不同的動作，如果 `File::open` 是因爲檔案不存在的話，我們想要建立檔案並回傳新檔案的句柄。如果 `File::open` 是因爲其他原因失敗的話，像是我們沒有開啟檔案的權限，我們仍然要像範例 9-4 這樣呼叫 `panic!`。範例 9-5 就這樣對 `match` 表達式加了更多條件。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">範例 9-5：針對不同種類的錯誤採取不同動作</span>

`File::open` 在 `Err` 變體的回傳型別爲 `io::Error`，這是標準函式庫提供的結構體。此結構體有個 `kind` 方法讓我們可以取得 `io::ErrorKind` 數值。標準函式庫提供的枚舉 `io::ErrorKind` 有從 `io` 運算可能發生的各種錯誤。我們想處理的變體是 `ErrorKind::NotFound`，這指的是我們嘗試開啟的檔案還不存在。所以我們對 `f` 配對並在用 `error.kind()` 繼續配對下去。

我們像從內部配對檢查 `error.kind()` 的回傳值是否是 `ErrorKind` 枚舉中的 `NotFound` 變體。如果是的話，我們就嘗試使用 `File::create` 建立檔案。不過 `File::create` 也可能會失敗，所以我們需要第二個內部 `match` 表達式來處理。如果檔案無法建立的話，我們就會印出不同的錯誤訊息。第二個分支的外部 `match` 分支保持不變，如果程式遇到其他錯誤的話就會恐慌。

我們用的 `match` 的確有點多！`match` 表達式雖然很實用，不過它的行爲非常基本。在第十三章你會學到閉包（closure），`Result<T, E>` 型別有很多接收閉包並採用 `match` 實作的方法。使用那些方法可以讓你的程式碼更簡潔。更熟練的 Rustacean 可能會像這樣寫範例 9-5 的程式數碼：

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-03-closures/src/main.rs}}
```

雖然此程式碼的行爲和範例 9-5 一樣，但他沒有包含任何 `match` 表達式而且更易閱讀。當你讀完第十三章後，別忘了回來看看此範例，並查閱標準函式庫中的 `unwrap_or_else` 方法。除此方法以外，還有更多方法可以來解決處理錯誤時龐大的 `match` 表達式。

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

還有另一個方法 `expect` 和 `unwrap` 類似，不過能讓我們選擇 `panic!` 回傳的錯誤訊息。使用 `expect` 而非 `unwrap` 並提供完善的錯誤訊息哦可以表明你的意圖，讓追蹤恐慌的源頭更容易。`expect` 的語法看起來就像這樣：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

我們使用 `expect` 的方式和 `unwrap` 一樣，不是回傳檔案句柄就是呼叫 `panic!` 巨集。使用 `expect` 呼叫 `panic!` 時的錯誤訊息會是我們傳遞給 `expect` 的參數，而不是像 `unwrap` 使用 `panic!` 預設的訊息。訊息看起來就會像這樣：

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

由於此錯誤訊息指明了我們想表達的訊息「Failed to open hello.txt」，我們比較能知道此錯誤訊息是從哪裡發生的。如果我們在多處使用 `unwrap`，我們會需要一些時間才能理解 `unwrap` 是從哪裡引發恐慌的，因爲 `unwrap` 很可能會顯示相同的訊息。

### 傳播錯誤

當你在寫某函式實作時，要是它的呼叫的程式碼可能會失敗，與其直接在此函式處理錯誤，你可以回傳錯誤給呼叫此程式的程式碼，由它們絕對如何處理。這稱之爲*傳播（propagating）*錯誤並讓呼叫者可以有更多的控制權，因爲比起你程式碼當下的內容，回傳的錯誤可能提供更多資訊與邏輯以利處理。

舉例來說，範例 9-6 展示了一個從檔案讀取使用者名稱的函式。如果檔案不存在或無法讀取的話，此函式會回傳該錯誤給呼叫此函式的程式碼。

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">範例 9-6：使用 `match` 回傳錯誤給呼叫者的函式</span>

此函式還能在更簡化，但我們要先繼續手動處理來進一步探討錯誤處理，最後我們會展示最精簡的方式。讓我們先看看此函式的回傳型別 `Result<String, io::Error>`。這代表此函式回傳的型別爲 `Result<T, E>`，而泛型型別 `T` 已經指明爲實際型別 `String` 然後泛型型別 `E` 也已經指明爲實際型別 `io::Error`。如果函式正確無誤的話，程式碼會呼叫此函式並收到擁有 `String` 的 `Ok` 數值。如果程式遇到任何問題的話，呼叫此函式的程式碼就會獲得擁有包含有關問題發生資訊的 `io::Error` 實例的 `Err` 數值。我們選擇 `io::Error` 作爲使函式的回傳值是因爲它正是 `File::open` 函式和 `read_to_string` 方法失敗時的回傳的錯誤型別。

函式本體從呼叫 `File::open` 開始，然後我們使用 `match` 回傳 `Result` 數值，就和範例 9-4 的 `match` 類似，但與其在 `Err` 情形時呼叫 `panic!`，我們儘早回傳 `File::open` 的錯誤型別給呼叫者。如果 `File::open` 成功的話，我們就將檔案句柄賦值給變數 `f` 並繼續執行下去。

接著我們在變數 `s` 建立新的 `String` 並對檔案句柄 `f` 呼叫 `read_to_string` 方法來讀取檔案內容至 `s`。`read_to_string` 也會回傳 `Result` 因爲它也可能失敗，就算 `File::open` 是執行成功的。所以我們需要另一個 `match` 來處理該 `Result`，如果 `read_to_string` 成功的話，我們的函式就是成功的，然後在 `Ok` 回傳 `s` 中該檔案的使用者名稱。如果 `read_to_string` 失敗的話，我們就像處理 `File::open` 的 `match` 一樣回傳錯誤值。不過我們不需要顯式寫出 `return`，因爲這是函式中的最後一個表達式。

呼叫此程式碼的程式就會需要處理包含使用者名稱的 `Ok` 數值以及包含 `io::Error` 的 `Err` 數值。我們不會知道呼叫此程式碼的人會如何處理這些數值。舉例來說，如果呼叫此程式碼而獲得錯誤的話，它可能選擇呼叫 `panic!` 讓程式崩潰，或者使用預設的使用者名稱從檔案以外的地方尋找該使用者。所以我們傳播所有成功或錯誤的資訊給呼叫者，讓它們能妥善處理。

這樣傳播錯誤的模式是非常常見的，所以 Rust 提供了 `?` 來簡化流程。

#### 傳播錯誤的捷徑：`?` 運算子

Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as it had in Listing 9-6, but this implementation uses the
`?` operator.

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">範例 9-7: A function that returns errors to the
calling code using the `?` operator</span>

The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.

There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert errors from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent all
the ways a function might fail, even if parts might fail for many different
reasons. As long as each error type implements the `from` function to define how
to convert itself to the returned error type, the `?` operator takes care of the
conversion automatically.

In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `f`. If an error occurs, the
`?` operator will return early out of the whole function and give any `Err`
value to the calling code. The same thing applies to the `?` at the end of the
`read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">範例 9-8: Chaining method calls after the `?`
operator</span>

We’ve moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn’t changed. Instead of creating a variable `f`, we’ve
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-6 and
Listing 9-7; this is just a different, more ergonomic way to write it.

Speaking of different ways to write this function, Listing 9-9 shows that
there’s a way to make this even shorter.

<span class="filename">檔案名稱：src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">範例 9-9: Using `fs::read_to_string` instead of
opening and then reading the file</span>

Reading a file into a string is a fairly common operation, so Rust provides the
convenient `fs::read_to_string` function that opens the file, creates a new
`String`, reads the contents of the file, puts the contents into that `String`,
and returns it. Of course, using `fs::read_to_string` doesn’t give us the
opportunity to explain all the error handling, so we did it the longer way
first.

#### The `?` Operator Can Be Used in Functions That Return `Result`

The `?` operator can be used in functions that have a return type of
`Result`, because it is defined to work in the same way as the `match`
expression we defined in Listing 9-6. The part of the `match` that requires a
return type of `Result` is `return Err(e)`, so the return type of the function
can be a `Result` to be compatible with this `return`.

Let’s look at what happens if we use the `?` operator in the `main` function,
which you’ll recall has a return type of `()`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/src/main.rs}}
```

When we compile this code, we get the following error message:

```console
{{#include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/output.txt}}
```

This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result` or `Option` or another type that implements
`std::ops::Try`. When you’re writing code in a function
that doesn’t return one of these types, and you want to use `?` when you call other
functions that return `Result<T, E>`, you have two choices to fix this problem.
One technique is to change the return type of your function to be `Result<T,
E>` if you have no restrictions preventing that. The other technique is to use
a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in
whatever way is appropriate.

The `main` function is special, and there are restrictions on what its return
type must be. One valid return type for main is `()`, and conveniently, another
valid return type is `Result<T, E>`, as shown here:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-07-main-returning-result/src/main.rs}}
```

The `Box<dyn Error>` type is called a trait object, which we’ll talk about in
the [“Using Trait Objects that Allow for Values of Different
Types”][trait-objects]<!-- ignore --> section in Chapter 17. For now, you can
read `Box<dyn Error>` to mean “any kind of error.” Using `?` in a `main`
function with this return type is allowed.

Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.

[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
