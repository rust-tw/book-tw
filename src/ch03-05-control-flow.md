## 控制流程

在大多數程式語言中，能夠決定依據某項條件是否爲真來執行些程式碼，以及依據某項條件是否爲真來重複執行些程式碼是非常基本的組成元件。在 Rust 程式碼中能讓你控制執行流程的常見方法有 `if` 表達式以及迴圈。

### `if` 表達式

`if` 能讓你依照條件判斷對你的程式碼產生分支。基本上你提供一個條件然後就像是在說：「如果此條件符合的話，就執行這個程式碼區塊；如果沒有的話，就不要執行這段程式碼。」

請在你的 *projects* 目錄下建立一個新的專案叫做 *branches* 好讓我們來探討 `if` 表達式。接著請在 *src/main.rs* 檔案內輸入以下內容：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

所有的 `if` 表達式都由 `if` 關鍵字開始在加上一個條件。在此例中的條件是判斷變數 `number` 是否小於 5。條件符合時所要執行的程式碼區塊被放在條件之後的大括號裡。與 `if` 表達式條件相關的程式碼段落有時也被稱爲 *arms*，就像我們在第二章[「將猜測的數字與祕密數字做比較」][comparing-the-guess-to-the-secret-number]<!-- ignore -->段落提到的 `match` 表達式的分支一樣。

另外，我們還可以選擇性地加上 `else` 表達式（就像範例寫的），讓條件不符時可以去執行另外一段程式碼。如果你沒有提供 `else` 表達式且條件爲否的話，程式會直接略過 `if` 的程式碼區塊，接著執行後續的程式碼。

請嘗試執行此程式碼，你應該會看到以下輸出結果：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

讓我們來變更 `number` 的值使條件變成 `false`，再來看看會發生什麼事：

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

在跑一次程式，然後看看輸出：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

還有一件值得注意的是程式碼的條件判斷*必須*是 `bool`。如果條件不是 `bool` 的話，我們就會遇到錯誤。比方說，試試以下程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

這次 `if` 條件計算出數值 `3`，然後 Rust 丟出錯誤給我們：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

錯誤訊息告訴我們 Rust 預期收到 `bool` 但是卻拿到整數。這和 Ruby 和 JavaScript 就不同，Rust 不會自動將非布林值型別轉換成布林值。你永遠必須顯式提供布林值給 `if` 作爲它的條件判斷。舉例來說，如果我們希望 `if` 只會在數值爲 `0` 才執行，我們可以將 `if` 表達式改成以下範例：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

執行此程式碼就會印出「number was something other than zero」。

#### 使用 `else if` 處理多重條件

想要實現多重條件的話，你可以將 `if` 和 `else` 組合成 `else if` 表達式。舉例來說：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

程式有四種可能的分支，當你執行它時你應該會看到以下輸出結果：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

當此程式執行時，他會依序檢查每一個 `if` 表達式，並執行第一個符合條件的程式碼段落。注意到雖然 6 的確可以除以 2，但我們沒有看到 `number is divisible by 2`，也沒有看到來自 `else` 那段的 `number is not divisible by 4, 3, or 2`。這是因爲 Rust 只會執行第一個符合條件的區塊，而當它遇到時它就不會在檢查其他條件。

使用太多的 `else if` 表達式很容易讓你的程式碼變得凌亂，所以當你需要用到一個以上，你可能會想要先重構程式碼看看。爲此我們在第六章會介紹一個功能強大的 Rust 條件判斷結構叫做 `match`。

#### 在 `let` 陳述式中使用 `if`

因爲 `if` 是表達式，所以我們可以像範例 3-2 這樣放在 `let` 陳述式的右邊。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">範例 3-2：將 `if` 表達式的結果賦值給變數</span>

變數 `number` 會得到 `if` 表達式運算出的數值。執行此程式看看會發生什麼事：

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

你應該還記得程式碼區塊也可以是表達式且會回傳最後一行的數值，而且數字本身也是表達式。在此例中，`if` 表達式的值取決於哪段程式碼被執行。這代表可能成爲最終結果的每一個 `if` 分支必須要是相同型別。在範例 3-2 中，各分支的型別都是 `i32`。如果型別不一致的話，如以下範例所示，我們會得到錯誤：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

當我們嘗試編譯程式碼時，我們會得到錯誤。`if` 和 `else` 分支的型別並不一致，而且 Rust 還確切指出程式出錯的地方在哪：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` 段落的表達式運算出整數，但 `else` 的區塊卻運算出字串。這樣行不通的原因是變數只能有一個型別。Rust 必須在編譯期間確切知道變數 `number` 的型別，這樣才能驗證它的型別在任何有使用到 `number` 的地方都是有效的。要是 `number` 只能在執行時知道的話，Rust 就沒辦法這樣做了。如果編譯器必須追蹤所有變數多種可能存在的型別，那就會變得非常負責並無法爲程式碼提供足夠的保障。

### 使用迴圈重複執行

重複執行同一段程式碼區塊時常是很有用的。針對這樣的任務，Rust 提供了多種產生 *迴圈（loops）*的方式。一個迴圈會執行一段程式碼區塊，然後在結束時馬上回到區塊起始位置繼續執行。爲了繼續探討迴圈，讓我們再開一個新專案 *loops*。

Rust 提供三種迴圈：`loop`、`while` 和 `for`。讓我們每個都嘗試看看吧。

#### 使用 `loop` 重複執行程式碼

`loop` 關鍵字告訴 Rust 去反覆不停地執行一段程式碼直到你親自告訴它要停下來。

我們用以下範例示範，請修改你 *loops* 目錄下的 *src/main.rs* 檔案成以下程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

當我們執行此程式時，我們會看到 `again!` 一直不停地重複顯示出來，直到我們手動停下程式爲止。大多數的終端機都支援 <span class="keystroke">ctrl-c</span> 這個快捷鍵來中斷一支卡在無限迴圈的程式，你可以自己試試看：

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

`^C` 這個符號表示你按下了 <span class="keystroke">ctrl-c</span>。按照程式收到中斷訊號的時間點，你可能不會看到 `again!` 出現在 `^C` 之後。

幸運的是 Rust 有提供另一個打破迴圈更可靠的方法。你可以在迴圈內加上 `break` 關鍵字告訴程式何時停止執行迴圈。回想一下我們在第二章[「猜對後離開」][quitting-after-a-correct-guess]<!-- ignore -->段落就做過這樣的事，當使用者猜對正確數字而獲勝時就會離開程式。

#### 從迴圈回傳數值

其中一種使用 `loop` 的用途是重試某些你覺得會失敗的動作，像是檢查一個執行緒是否已經完成其任務。不過這樣你可能就會想傳遞任務結果給之後的程式碼。要做到這樣的事，你可以在你要用來停下迴圈的 `break` 表達式內加上一個你想回傳數值，該值就會被停止的迴圈回傳，如以下所示：

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

在迴圈之前，我們宣告了一個變數 `counter` 並初始化爲 `0`，然後我們宣告了另一個變數 `result` 來取的迴圈回傳的值。在迴圈每一次的迭代中，我們將變數 `counter` 加上 `1` 並檢查它是否等於 `10`。如果是的話就用 `break` 關鍵字回傳 `counter * 2`。在迴圈結束後，我們用分號才結束這個賦值給 `result` 的陳述式。最後我們印出 `result`，而結果爲 20。

#### 使用 `while` 做條件迴圈

在程式中用條件判斷迴圈的執行通常是很有用的。當條件爲真時，迴圈就繼續執行。當條件不再符合時，程式就用 `break` 停止迴圈。這樣的循環方法可以用 `loop`、`if`、`else` 和 `break` 組合出來。如果你想嘗試的話，你現在就可以自己寫寫看看。

但是這種模式非常常見，所以 Rust 有提供內建的結構稱爲 `while` 迴圈。範例 3-3 就是使用 `while` 的例子：該程式會循環三次，每次計數都減一，然後在迴圈之後印出訊息並離開。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">範例 3-3：使用 `while` 迴圈，當條件符合就持續執行程式碼</span>

這樣消除了很多使用 `loop`、`if`、`else` 與 `break` 會有的巢狀結構，這樣可以更易閱讀。當條件爲真的，程式碼就執行；不然的話，它就離開迴圈。

#### 使用 `for` 遍歷集合

你可以用 `while` 來遍歷一個集合的元素，像是陣列等等。舉例來說，我們可以看看範例 3-4。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">範例 3-4：使用 `while` 遍歷集合的每個元素</span>

程式在此對陣列的每個元素計數，它先從索引 `0` 開始，然後持續循環直到它抵達最後一個陣列索引爲止（也就是 `index < 5` 不再爲真）。執行此程式會印出陣列裡的每個元素：

```text
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

所有五個元素都如預期顯示在終端機上。儘管 `index` 會在某一刻達到 `5`，但是迴圈會在嘗試取得陣列第六個元素前就停止執行。

但這樣的方式是容易出錯的，我們可能取得錯誤的索引長度造成程式恐慌。這同時也使程式變慢，因爲編譯器得在執行時的程式碼對迴圈中每次迭代的每個元素加上條件檢查。

所以更簡潔的替代方案是，你可以使用 `for` 迴圈來對集合的每個元素執行一些程式碼。`for` 迴圈的樣子就像範例 3-5 寫的這一樣。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">範例 3-5：使用 `for` 迴圈遍歷集合的每個元素</span>

當我們執行此程式時，我們會看到和範例 3-4 一樣的結果。最重要的是，我們增加了程式的安全性，去除了造成程式錯誤的可能性。不會出現超出陣列大小或是讀取長度不足的風險。

比方說在範例 3-4 的程式碼，如果你變更陣列 `a` 的元素爲只有 4 個，但忘記更新條件判斷爲 `while index < 4` 的話，程式就會恐慌。使用 `for` 迴圈的話，我們變更陣列長度時，就不需要去記得更新其他程式碼。

`for` 迴圈的安全性與簡潔程度讓它成爲 Rust 最常被使用的迴圈結構。就算你想執行的是依照次數循環的程式碼，像是範例 3-3 的 `while` 迴圈範例，多數 Rustaceans 還是會選擇 `for` 迴圈。要這麼做的方法是使用 `Range`，這是標準函式庫提供的型別，用來產生一連串的數字序列，從指定一個數字開始一直到另一個數字之前結束。

以下是我們用 `for` 迴圈來計數的另一種方式，它用了一個我們還沒講過的方法 `rev`，這可以用來反轉：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

這樣是不是看起來更好讀許多？

## 總結

你做到了！這的確是篇大章節：你學到了變數、純量與複合資料型別、函式、註解、`if` 表達式以及迴圈！如果你想練習此章的概念，你可以試著打造以下程式：

* 轉換攝氏與華氏溫度。
* 產生第 n 個斐波那契數字。
* 試著用重複的歌詞印出 Christmas carol 的 The Twelve Days of Christmas。

當你準備好後，我們就來探討一個其他語言*不常見*的概念：所有權。

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch03-05-control-flow.md)
> - updated: 2020-09-07