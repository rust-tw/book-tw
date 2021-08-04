## 變數與可變性

如同[「透過變數儲存數值」][storing-values-with-variables]<!-- ignore -->提到的，變數預設是不可變的。這是 Rust 推動你能充分利用 Rust 提供的安全性和簡易並行性來寫程式的許多方法之一。不過，你還是有辦法能讓你的變數成為可變的。讓我們來探討為何 Rust 鼓勵你多多使用不可變，以及何時你會想要改為可變的。

當一個變數是不可變的，只要有數值綁定在一個名字上，你就無法改變其值。為了方便說明，讓我們使用 `cargo new variables` 在 *projects* 目錄下產生一個新專案叫做 *variables*。

再來在你的 *variables* 目錄下開啟 *src/main.rs* 然後覆蓋程式碼為以下內容，這是段還無法編譯的程式碼：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

儲存然後使用 `cargo run` 執行程式。你應該會收到一則錯誤訊息，如下所示：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

此範例顯示了編譯器如何協助你找到你程式碼的錯誤。雖然看到編譯器錯誤訊息總是令人感到沮喪，但這通常是為了讓你知道你的程式無法安全地完成你想讓它完成的任務。它們**不代表**你不是個優秀的程式設計師！有經驗的 Rustaceans 時常會與編譯器錯誤訊息打交道。

這則錯誤訊息表示錯誤發生的原因：「cannot assign twice to immutable variable `x`」，因為你嘗試第二次賦值給 `x` 變數。

當我們嘗試改變一個原先設計為不可變的變數時，能夠產生編譯時錯誤是很重要的。因為這樣的情況很容易導致程式錯誤。如果我們有一部分的程式碼在執行時認為某個數值絕對不會改變，但另一部分的程式碼卻更改了其值，那麼這就有可能讓前一部分的程式碼就可能以無法預測的方式運行。這樣的程式錯誤的起因是很難追蹤的，尤其是當第二部分的程式碼**偶而**才會改變其值。

在 Rust 中，編譯器會保證當你宣告一個數值不會被改變時，它就絕對不會被改變。這代表當你讀寫程式碼時，你不需要去追蹤該值可能會被改變，讓你的程式碼更容易推導。

但同時可變性也是非常有用的，變數只有預設是不可變的，就如同第二章一樣你可以在變數名稱前面加上 `mut` 讓它們可以成為可變的。除了允許改變其值之外，`mut` 向未來的讀取者表明了其他部分的程式碼將會改變此變數的數值。

舉例來說，讓我們改變 *src/main.rs*  成以下程式碼：

<span class="filename">檔案內容：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

當你執行程式的話，我們會得到：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

當使用 `mut` 時，我們可以將 `x` 的數值從 `5` 改變為 `6`。有時候比起只有不可變變數，你會想要將某些變數改為可變的，讓它更容易編寫。

當然除了防止程式錯誤以外，這還有很多權衡取捨。舉例來說，當你擁有一個大型資料結構時，變更其值通常會比複製然後返回重新分配的實例還來的快。不過在比較小的資料結構，用函式程式語言的風格產生新的實例會比較容易思考，所以損失一些效能會比損失閱讀性來得好。

### 變數與常數的差異

不能夠變更數值的情況可能會讓你聯想到其他程式語言都有的概念：**常數（constants）**。和不可變變數一樣，常數會讓數值與名稱綁定且不允許被改變，但是不可變變數與常數還是有些差異。

首先，你無法在使用常數使用 `mut`，常數不是預設不可變，它們永遠都不可變。

如果你使用 `const` 宣告而非 `let` 的話，你**必須**指明型別。我們會在下一章[「資料型別」][data-types]<!-- ignore -->詳細解釋型別與型別詮釋，所以現在先別擔心細節。你只需要先知道你永遠必須先詮釋常數的型別。

常數可以被定義在任一有效範圍，包含全域有效範圍。這讓它們非常有用，讓許多部分的程式碼都能夠知道它們。

最後一個差別是常數只能被常數表達式設置，而不能用任一在運行時產生的其他數值設置。

以下為一個常數名稱被宣告為 `THREE_HOURS_IN_SECONDS` 的範例，它的數值被設為 60（一分鐘有多少秒）乘上 60（一小時有多少分鐘）乘上 3（此程式想要計算的小時數量）：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust 的常數命名規則為使用全部英文大寫並用底寫區隔每個單字。編譯器能夠在編譯時用特定限制集合內的操作進行運算，讓我們能用易於理解且驗證的方式寫出此數值，而不用將常數設爲 10,800。你可以查閱 Rust Reference 的 [constant evaluation 段落][const-eval]來瞭解哪些操作可以在宣告常數時使用。

在整支程式運行時，常數在它們的範圍內都是有效的。這樣的性質讓常數在處理應用程式中需要被許多程式碼部份所知道的數值的情況下是非常好的選擇，像是一款遊戲中玩家能夠得到的最高分數或者光速的數值。

將會擴散到所有程式碼的數值定義為常數，對於幫助未來程式碼的維護者理解是非常好的選擇。這也讓未來需要更新數值的話，你知道需要修改寫死的地方就好。

### 遮蔽（Shadowing）

如同你在猜謎遊戲教學所看到的，在第二章[「將猜測的數字與祕密數字做比較」][comparing-the-guess-to-the-secret-number]<!-- ignore -->你可以用之前的變數再次宣告新的變數。Rustaceans 會說第一個變數被第二個變數所**遮蔽**了，這代表該變數被使用時會拿到第二個變數的數值。我們可以用 `let` 關鍵字來重複宣告相同的變數名稱來遮蔽一個變數：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

此程式首先將 `x` 給予 `5`，然後它用 `let x =` 遮蔽了 `x` 變數取代了原本的變數變為 `6`。然後內部範圍內，第三次的 `let` 陳述式一樣遮蔽了 `x` 讓它將原本的值乘與 `2`，讓 `x` 數值為 `12`。當該範圍結束時，內部的遮蔽也結束，所以 `x` 就回到原本的 `6`。當我們運行此程式時，當會輸出以下結果：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

遮蔽與標記變數為 `mut` 是不一樣的，因為如果我們不小心重新賦值而沒有加上 `let` 關鍵字的話，是會產生編譯期錯誤的。使用 `let` 的話，我們可以作出一些改變，然後在這之後該變數仍然是不可變的。

另一個 `mut` 與遮蔽不同的地方是，我們能有效地再次運用 `let` 產生新的變數，可以在重新運用相同名稱時改變它的型別。舉例來說，當我們希望程式要求使用者顯示出字串間應該顯示多少空格，但同時我們又希望它被存為一個數字時，我們可以這樣做：

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

這個範例是被允許的是因為第一次宣告 `spaces` 的變數雖然是一個字串型別，但在第二次宣告儘管用了同樣的名稱，但是我們卻能遮蔽成數字型別。遮蔽這項功能讓我們不必去宣告像是 `spaces_str` 與 `spaces_num`，我們可以重複使用 `spaces` 這個變數名稱。不過，可變變數仍然是無法變更變數型別的，如果這樣做的話我們就會拿到編譯期錯誤：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

此錯誤訊息告訴我們我們不允許改變變數的型別：

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

現在我們講完變數了，讓我們看看它們可以擁有的資料型別吧。

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#將猜測的數字與祕密數字做比較
[data-types]: ch03-02-data-types.html#資料型別
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#透過變數儲存數值
[const-eval]: https://doc.rust-lang.org/stable/reference/const_eval.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch03-01-variables-and-mutability.md)
> - updated: 2020-09-05
