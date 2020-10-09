## 所有能使用模式的地方

模式常出現於 Rust 中數個位置，而你已經不經意使用了很多模式了！此段落會介紹所有模式能有效出現的地方。

### `match` 分支

如同第六章所討論過的，我們可以在 `match` 表達式中的分支使用模式。正式來說，`match` 表達式的定義爲 `match` 關鍵字加上一個要配對的數值，然後會有一或數個包含模式的分支，以及如果數值配對到該分支模式之後要執行的表達式，如以下所示：

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

`match` 表達式有個要求就是它們必須是*徹底的（exhaustive）*，所有 `match` 表達式數值可能的結果都必須涵蓋到。其中一個確保你有考慮到所有可能性的方式是在最後一個分支使用捕獲模式，命名一個能配對任何數值的變數就覺不會失敗，因此可以涵蓋剩餘的情況。

還有一個特定模式 `_` 可以獲取任意可能情況，但它不會綁定到變數中，所以它也很常用在最後的配對分支。舉例來說，`_` 模式就很適合用來忽略任何沒指明的數值。我們會在本章之後的 [「忽略模式中的數值」][ignoring-values-in-a-pattern]<!-- ignore -->段落談到更多 `_` 的細節。

### `if let` 條件表達式

在第六章中我們介紹了如何使用 `if let` 表達式，它等同於只配對一種情況的 `match` 表達式，主要作爲更簡潔的語法。此外，`if let` 可以再加上 `else` 來包含如果 `if let` 模式不符的話能執行的程式碼。

範例 18-1 展示了我們能夠混合並配對 `if let`、`else if` 與 `else if let` 表達式。這樣做可以比 `match` 表達式還來得有彈性，因爲 `match` 只能有一個數值與模式們配對。另外，`if let`、`else if` 與 `else if let` 分支之間的條件彼此並不需要有關聯。

範例 18-1 的程式碼顯示了一系列的條件檢查來決定背景顏色該爲何。在此例中，我們建立一個寫死的變數數值，在實際程式中應該會由使用者輸入。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-01/src/main.rs}}
```

<span class="caption">範例 18-1：混合 `if let`、`else if`、`else if let` 與 `else`</span>

如果使用者指定的最喜歡的顏色，該顏色就是背景顏色。如果今天是星期二，背景顏色就是綠色。如果使用者用字串指定他們的年齡且可以成功轉換成數字的話，背景顏色依據數字結果就是紫色或橘色。如果以上條件都不符合的話，背景顏色就是藍色。

這樣的條件結構讓我們可以職員複雜的需求。透過我們在此寫死的數值，此例會印出 `Using purple as the background color`。

你可以看到 `if let` 也能如同 `match` 的分支一樣遮蔽變數，`if let Ok(age) = age` 這行就產生了新的遮蔽變數 `age` 來包含 `Ok` 變體內的數值。這意味著我們需要將 `if age > 30` 的條件方在區塊內，我們不能組合這兩個條件成 `if let Ok(age) = age && age > 30`。遮蔽的 `age` 在大括號開始之後的新作用域才有效，此時才能與 30 做比較。

使用 `if let` 表達式的缺點是編譯器不會徹底檢查，而 `match` 表達式則會。如果我們省略最後一個 `else` 區塊而因此忘了處理一些情況，編譯器不會警告我們這種可能的邏輯錯誤。

### `while let` 條件迴圈

與 `if let` 的結構類似，`while let` 條件迴圈允許 `while` 迴圈只要在模式持續配對符合的情況下一直執行。範例 18-2 的例子展示一個 `while let` 迴圈使用向量最爲堆疊，並以數值被插入向量時相反的順序印出它們。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-02/src/main.rs:here}}
```

<span class="caption">範例 18-2：使用 `while let` 迴圈，且只要 `stack.pop()` 有回傳 `Some` 就持續印出數值</span>

此範例會依序顯示 3、2 然後是 1。`pop` 方法會取得向量最後一個數值並回傳 `Some(value)`。如果向量是空的，`pop` 就回傳 `None`。只要 `pop` 有回傳 `Some`，`while` 迴圈就會持續執行其區塊中的程式碼。當 `pop` 回傳 `None` 時，迴圈就會結束。我們可以使用 `while let` 來取得堆疊彈出的每個數值。

### `for` 迴圈

在第三章中，我們提到 `for` 迴圈是 Rust 程式碼中最常見的迴圈結構，但我們尚未介紹要如何在  `for` 中使用模式。在 `for` 迴圈中，`for` 關鍵字之後的數值就是模式，所以在 `for x in y` 中 `x` 就是模式。

範例 19-3 展示了如何在 `for` 迴圈使用模式來解構或拆開一個 `for` 迴圈中的元組。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-03/src/main.rs:here}}
```

<span class="caption">範例 18-3：使用模式來解構 `for` 迴圈中的元組</span>

範例 18-3 的程式碼會顯示以下結果：

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-03/output.txt}}
```

我們使用 `enumerate` 方法來配接一個疊代器來產生一個數值與該數值在疊代器中的索引，並放入元組中。第一次呼叫 `enumerate` 會產生元組 `(0, 'a')`。當此數值配對到 `(index, value)` 模式時，`index` 會是 `0` 而 `value` 會是 `'a'`，並印出第一行的輸出。

### `let` 陳述式

在本章節之前，我們只有告訴你模式能用在 `match` 和 `if let`，但事實上我們在其他地方也早就使用過模式了，這包含 `let` 陳述式。舉例來說，請看看以下這個使用 `let` 賦值變數的直白例子：

```rust
let x = 5;
```

在整本書中，我們已經像這樣使用 `let` 無數次，而雖然你還沒有察覺到，但你已經使用過模式了！所以更正式地來說，`let` 陳述式是這樣定義的：

```text
let PATTERN = EXPRESSION;
```

像 `let x = 5;` 這樣的陳述式中變數名稱會位於 `PATTERN` 的位置，變數名稱恰好是種特別簡單的模式。Rust 會將表達式與模式做比較，並賦值給它找到的任何名稱。所以在 `let x = 5;` 的範例中，`x` 是個模式並表示「將配對到的數值綁定給變數 `x`」。因爲名稱 `x` 就是整個模式，此模式實際上等同於「將任何數值綁定給變數 `x`，無論該數值爲何」。

爲了更清楚理解 `let` 怎麼使用模式配對，請參考範例 18-4，這對 `let` 使用模式來解構一個元組。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-04/src/main.rs:here}}
```

<span class="caption">範例 18-4：使用模式來解構元組，並同時建立三個變數</span>

我們在此用一個元組來配對一個模式。Rust 會將數值 `(1, 2, 3)` 與模式 `(x, y, z)` 做比較，並看出數值能配對到模式中，所以 Rust 將 `1` 綁定給 `x`、`2` 給 `y` 然後 `3` 給 `z`。你可以把此元組模式想成是三個獨立的變數模式組合在一起。

如果模式中的元素數量與元組中的元素數量不符合的話，整體型別就無法配對，所以我們會得到編譯錯誤。舉例來說，範例 18-5 嘗試將有三個元素的元組解構到兩個變數中，這樣就無法成功。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-05/src/main.rs:here}}
```

<span class="caption">範例 18-5：錯誤的模式結構，因爲變數數量與元組元素數量不符</span>

嘗試編譯此程式碼的話，會得到此型別錯誤：

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-05/output.txt}}
```

如果我們在元組中想要忽略一或數個數值的話，我們可以使用 `_` 或 `..`，你會在[「忽略模式中的數值」][ignoring-values-in-a-pattern]<!-- ignore -->段落中瞭解更多詳情。如果問題出在於我們模式中有太多變數的話，解決辦法就是移除些變數使變數數量等同於元組元素數量，讓型別可以配對。

### 函式參數

函式參數也可以是模式。範例 18-6 的程式碼宣告了一個函式叫做 `foo` 來接收一個參數叫做 `x` 其型別爲 `i32`，現在這看起來你應該都還很熟悉。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-06/src/main.rs:here}}
```

<span class="caption">範例 18-6：在參數中使用模式的函式簽名</span>

`x` 的部分就是模式！就如同我們在 `let` 所做的一樣，我們可以在函式引數中使用模式來配對元組，範例 18-7 將傳遞給函式的元組拆爲不同數值。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-07/src/main.rs}}
```

<span class="caption">範例 18-7：函式透過參數來解構元組</span>

此程式碼會顯示 `Current location: (3, 5)`。數值 `&(3, 5)` 能配對到模式 `&(x, y)`，所以 `x` 會是數值 `3` 而 `y` 會是數值 `5`。

我們還可以在閉包參數列表中像函式參數列表這樣使用模式，因爲第十三章就提過閉包類似於函式。

到目前爲止，你已經見過許多使用模式的方式，但模式在我們能使用的地方並不都會有相同的行爲。在某些地方，模式必須是不可反駁的（irrefutable），而在其他場合它們則是可反駁的（refutable）。接下來我們會來討論這兩個概念。

[ignoring-values-in-a-pattern]:
ch18-03-pattern-syntax.html#忽略模式中的數值

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch18-01-all-the-places-for-patterns.md)
> - updated: 2020-09-25
