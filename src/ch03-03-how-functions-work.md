## 函式

函式在 Rust 程式碼中無所不在。你已經見過一個語言最重要的函式了：`main` 函式是許多程式的入口點。此外你也看到了 `fn` 關鍵字能讓你宣告新的函式。

Rust 程式碼使用 *snake case* 作爲函式與變數名稱的慣例風格。在 snake case 中，所有的字母都是小寫，並用底線區隔單字。以下是一支包含函式定義範例的程式：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust 的函式定義從 `fn` 開始且在函式名稱後會有一組括號，大括號告訴編譯器函式本體的開始與結束位置。

我們可以輸入函式的名稱並加上括號來呼叫任何我們定義過的函式。因爲 `another_function` 已經在程式中定義了，他就可以在 `main` 函式中呼叫。注意到我們是在原始碼中的 `main` 函式*之後*定義 `another_function` 的，我們當然也可以把它定義在前面。Rust 不在乎你的函式是在哪裡定義的，只需要知道它在某處有定義就好。

讓我們開啟一個新的專案叫做 *functions* 來進一步探索。請將 `another_function` 範例放入 *src/main.rs* 然後執行它。你應該會看到以下輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

程式碼會按照 `main` 函式中的順序執行。首先，「Hello, world!」的訊息會先顯示出來，再來才會呼叫 `another_function` 並印出它的訊息。

### 函式參數

函式也可以被定義成擁有*參數（parameters）*的，這是函式簽名（signatures）中特殊的變數。當函式有參數時，你可以提供那些參數的確切數值。嚴格上來說，我們傳遞的數值會叫做*引數（arguments）*。但爲了方便起見，通常大家不太會去在意兩者的區別。雖然函式定義時才叫*參數*，傳遞數值時叫做*引數*，但很多時候會被人們拿來交互使用。

以下是加上參數後重新寫過的 `another_function` 範例：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

嘗試執行程式的話，你應該會看到以下輸出結果：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

宣告 `another_function` 時有一個參數叫做 `x`，而 `x` 的型別被指定爲 `i32`。當我們傳遞 `5` 給 `another_function` 時，`println!` 巨集會將 `5` 置於格式化字串中的大括號的位置。

在函式簽名中，你*必須*宣告每個參數的型別，這是 Rust 謹慎考慮後的設計決定：在函式定義中要求型別詮釋，代表編譯器幾乎不需要你在其他地方再提供資訊才能知道你要做什麼。

如果你希望函式擁有數個參數，你可以用逗號區隔開來，像這樣：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

此範例建立了一個有兩個參數的函式，兩個都是 `i32` 型別。接著函式在印出兩個參數的數值，注意參數不必得是相同的形態，這只是我們在此範例這樣寫而已。

讓我們試著執行此程式碼，請覆蓋你的專案 *functions* 內的 *src/main.rs* 檔案內容爲以上範例，然後用 `cargo run` 執行程式：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

因爲我們呼叫函式時，將 `5` 給了  `x` 且將 `6` 給了 `y`，字串就會印出這些數值。

### 函式本體包含陳述式與表達式

函式本體是由一系列的陳述式（statements）並在最後可以選擇加上表達式（expression）來組成。目前我們只講了沒有用到表達式做結尾的函式。由於 Rust 是門基於表達式（expression-based）的語言，知道這樣的區別是很重要的。其他語言通常沒有這樣的區別，所以現在讓我們來看看陳述式和表達式有什麼不同，以及它們怎麼影響函式本體。

我們其實已經使用了很多次陳述式與表達式。*陳述式（Statements）*是進行一些動作的指令，且不回傳任何數值。*表達式（Expressions）*則是計算並產生數值。讓我們來看一些範例：

建立一個變數然後用 `let` 關鍵字賦值給它就是一道陳述式。在範例 3-1 中的 `let y = 6;` 就是個陳述式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">範例 3-1：包含一道陳述式的 `main` 函式宣告</span>

此函式定義也是陳述式，整個範例就是本身就是一個陳述式。

陳述式不會回傳數值，因此你無法將 `let` 陳述式賦值給其他變數。如同以下程式碼所做的，你將會得到一個錯誤：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

當你執行此程式時，你就會看到這樣的錯誤訊息：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` 陳述式不回傳數值，所以 `x` 得不到任何數值。這就和其他語言有所不同，像是 C 或 Ruby，通常它們的賦值仍能回傳所得到的值。在那些語言，你可以寫 `x = y = 6` 同時讓 `x` 與 `y` 都取得 `6`，但在 Rust 就不行。

表達式則會運算出些東西，並組合成你大部分所寫的 Rust 程式。先想想看一個簡單的數學運算比如 `5 + 6`，這就是個會算出 `11` 的表達式。表達式可以是陳述式的一部分：在範例 3-1 中 `let y = 6;` 的 `6` 其是就是個算出 `6` 的表達式。呼叫函式也可以是表達式、呼叫巨集也是表達、我們用 `{}` 產生的作用域也是表達式。舉例來說：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

此表達式：

```rust,ignore
{
    let x = 3;
    x + 1
}
```

就是一個會回傳 `4` 的區塊，此值再用 `let` 陳述式賦值給 `y`。請注意到 `x + 1` 這行沒有加上分號，它和你目前看到的寫法有點不同，因爲表達式結尾不會加上分號。如果你在此表達式加上分號的話，它就不會回傳數值。在我們繼續探討函式回傳值與表達式的同時請記住這一點。

### 函式回傳值

函式可以回傳數值給呼叫它們的程式碼，我們不會爲回傳值命名，但我們會用箭頭（`->`）來宣告它們的型別。在 Rust 中，回傳值其實就是函式本體最後一行的表達式。你可以用 `return` 關鍵字加上一個數值來提早回傳函式，但多數函式都能用最後一行的表達式作爲數值回傳。以下是一個有回傳數值的函式範例：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

在 `five` 函式中沒有任何函式呼叫、巨集甚至是 `let` 陳述式，只有一個 `5`。這在 Rust 中完全是合理的函式。請注意到函式的回傳型別也有指明，就是 `-> i32`。嘗試執行此程式的話，輸出結果就會像是這樣：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`five` 中的 `5` 就是函式的回傳值，這就是爲何回傳型別是 `i32`。讓我們進一步研究細節，這邊有兩個重要的地方：首先這行 `let x = five();` 顯示了我們用函式的回傳值作爲變數的初始值。因爲函式 `five` 回傳 `5`，所以這行和以下程式碼相同：

```rust
let x = 5;
```

再來，`five` 函式沒有參數但有定義回傳值的型別。所以函式本體只需有一個 `5` 就好，不需加上分號，這樣就能當做表達式回傳我們想要的數值。

讓我們在看另一個例子：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

執行此程式會顯示 `The value of x is: 6`，但如果我們在最後一行 `x + 1` 加上分號的話，就會將它從表達式變爲陳述式。我們就會得到錯誤。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

編譯此程式就會產生以下錯誤：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

錯誤訊息「mismatched types」就告訴了我們此程式碼的核心問題。`plus_one` 的函式定義說它會回傳 `i32` 但是陳述式不會回傳任何數值。我們用空元組 `()` 表示不會回傳任何值。因此沒有任何值被回傳，這和函式定義相牴觸，最後產生錯誤。在此輸出結果，Rust 提供了一道訊息來協助解決問題：它建議移除分號，這樣就能修正錯誤。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch03-03-how-functions-work.md)
> - updated: 2020-09-05