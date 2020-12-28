## 切片型別

另一種沒有所有權的資料型別是**切片（slice）**。切片讓你可以引用一串集合中的元素序列，而並非引用整個集合。

以下是個小小的程式問題：寫一支接收字串的函式並回傳第一個找到的單字，如果函式沒有在字串找到空格的話，就代表整個字串就是一個單字，所以就回傳整個字串。

先來想想看函式簽名該長怎樣：

```rust,ignore
fn first_word(s: &String) -> ?
```

此函式 `first_word` 有一個參數 `&String`。我們不需要取得所有權，所以這是合理的。但我們該回傳啥呢？我們目前還沒有方法能夠描述一個字串的**其中一部分**。不過我們可以回傳該單字的最後一個索引。讓我們像範例 4-7 這樣試試看。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<span class="caption">範例 4-7：函式 `first_word` 回傳參數 `String` 第一個單字最後的索引</span>

因為我們需要遍歷 `String` 的每個元素並檢查該值是否為空格，我們要用 `as_bytes` 方法將 `String` 轉換成一個位元組陣列：

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

接下來我們使用 `iter` 方法對位元組陣列建立一個疊代器（iterator）：

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

我們會在第十三章討論疊代器的細節。現在我們只需要知道 `iter` 是個能夠回傳集合中每個元素的方法，然後 `enumerate` 會將 `iter` 的結果包裝起來回傳成元組。`enumerate` 回傳的元組中的第一個元素是索引，第二個才是元素的引用。這樣比我們自己計算索引還來的方便。

既然 `enumerate` 回傳的是元組，我們可以用模式配對來解構元組，就像在 Rust 其他地方使用的方式一樣。所以在 `for` 迴圈中，我們指定了一個模式讓 `i` 取得索引然後 `&item` 取得元組中的位元組。因為我們從用 `.iter().enumerate()` 取得引用的，所以在模式中我們用的是 `&` 來獲取。

在 `for` 迴圈裡面我們使用字串字面值的語法搜尋位元組是不是空格。如果我們找到空格的話，我們就回傳該位置。不然我們就用 `s.len()` 回傳整個字串的長度：

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

我們現在有了一個能夠找到字串第一個單字結尾索引的辦法，但還有一個問題。我們回傳的是一個獨立的 `usize`，它套用在 `&String` 身上才有意義。換句話說，因為它是個與 `String` 沒有直接關係的數值，我們無法保證它在未來還是有效的。參考一下使用了範例 4-7 中函式 `first_word` 的範例 4-8：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

<span class="caption">範例 4-8：先儲存呼叫函式 `first_word`的結果再變更 `String` 的內容</span>

此程式可以成功編譯沒有任何錯誤，而且我們在呼叫 `s.clear()` 後仍然能使用 `word`。因為 `word` 和 `s` 並沒有直接的關係，`word` 在之後仍能繼續保留 `5`。我們可以用 `s` 取得 `5` 並嘗試取得第一個單字。但這樣就會是程式錯誤了，因為 `s` 的內容自從我們賦值 `5` 給 `word` 之後的內容已經被改變了。

要隨時留意 `word` 會不會與 `s` 的資料脫鉤是很煩瑣的且容易出錯！要是我們又寫了個函式 `second_word`，管理這些索引會變得非常難以管控！我們會不得不將函式簽名改成這樣：

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

現在我們得同時紀錄起始**與**結束的索引，而且我們還產生了更多與原本數值沒辦法直接相關的計算結果。我們現在有三個非直接相關的變數需要保持同步。

幸運的是 Rust 為此提供了一個解決辦法：字串切片（String slice）。

### 字串切片

**字串切片**是 `String` 其中一部分的引用，它長得像這樣：

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

這和取得整個 `String` 的引用相似，但是加上了 `[0..5]`。所以與其引用整個 `String`，這個只引用了一部分的`String`。

我們可以像這樣 `[起始索引..結束索引]` 用中括號加上一個範圍來建立切片。`起始索引` 是切片的第一個位置，而 `結束索引` 在索引結尾之後的位置（所以不包含此值）。在內部的切片資料結構會儲存起始位置，以及 `結束索引` 與 `起始索引` 相減後的長度。所以用 `let world = &s[6..11];` 作為例子的話， `world` 就會是個切片，包含一個指標指向 `s` 第七個位元組和一個長度數值 `5`。

圖示 4-6 就是此例的示意圖。

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">圖示 4-6：指向部分 `String` 的字串切片</span>

要是你想用 Rust 指定範圍的語法 `..` 從第一個索引（也就是零）開始的話，你可以省略兩個句點之前的值。換句話說，以下兩個是相等的：

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

同樣地，如果你的切片包含 `String` 的最後一個位元組的話，你同樣能省略最後一個數值。這代表以下都是相等的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

如果你要獲取整個字串的切片，你甚至能省略兩者的數值，以下都是相等的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> 注意：字串切片的索引範圍必須是有效的 UTF-8 字元界限。如果你嘗試從一個多位元組字元（multibyte character）中產生字串切片，你的程式就會回傳錯誤。為了方便介紹字串切片，本章只使用了 ASCII 字元而已。
我們會在第八章的[「使用 String 儲存 UTF-8 編碼的文字」][strings]<!-- ignore -->做更詳盡的討論。

有了這些資訊，讓我們用切片來重寫 `first_word` 吧。對於「字串字面值」的的回傳型別我們會寫 `&str`：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

我們如同範例 4-7 一樣用判斷第一個空格取得了單字結尾的索引。當我們找到第一個空格，我們用字串的初始索引與當前空格的索引作為初始與結束索引來回傳字串切片。

現在當我們呼叫 `first_word`，我們就會取得一個與原本資料有直接相關的數值。此數值是由切片的起始位置即切片中的元素個數組成。

這樣函式 `second_word`  一樣也可以回傳切片：

```rust,ignore
fn second_word(s: &String) -> &str {
```

我們現在有個不可能出錯且更直觀的 API，因為編譯器會確保 `String` 的引用會是有效的。還記得我們在範例 4-8 的錯誤嗎？就是那個當我們取得單字結尾索引，但字串卻已清空變成無效的錯誤。那段程式碼邏輯是錯誤的，卻不會馬上顯示錯誤。要是我們持續嘗試用該索引存取空字串的話，問題才會浮現。切片可以讓這樣的程式錯誤無所遁形，並及早讓我們知道我們程式碼有問題。使用切片版本 `first_word` 的程式碼的話就會出現編譯期錯誤：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

以下是錯誤訊息：

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

回憶一下借用規則，要是我們有不可變引用的話，我們就不能取得可變引用。因為 `clear` 會縮減 `String`，它必須是可變引用。這樣一來 Rust 就不允許，並讓編譯失敗。Rust 不僅讓我們的 API 更容易使用，還想辦法讓所有錯誤在編譯期就消除！

#### 字串字面值就是切片

回想一下我們講說字串字面值是怎麼存在二進制檔案的。現在既然我們已經知道切片，我們就能知道更清楚理解字串字面值：

```rust
let s = "Hello, world!";
```

此處 `s` 的型別是 `&str`：它是指向二進制檔案某部份的切片。這也是為何字串字面值是不可變的，`&str` 是個不可變引用。

#### 字串切片作為參數

知道你可以取得字面值的切片與 `String` 數值後，我們可以再改善一次 `first_word`。也就是它的簽名表現：

```rust,ignore
fn first_word(s: &String) -> &str {
```

較富有經驗的 Rustacean 會用範例 4-9 的方式編寫函式簽名，因為這讓該函式可以同時接受 `&String` 和 `&str` 的數值。

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<span class="caption">範例 4-9：使用字串切片作為參數 `s` 來改善函式 `first_word`</span>

如果我們有字串字面值的話，我們可以直接傳遞。如果我們有 `String` 的話，我可以們傳遞整個 `String` 的切片。定義函式的參數為字串字面值而非 `String` 可以讓我們的 API 更通用且不會失去去任何功能：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

### 其他切片

字串切片如你所想的一樣是特別針對字串的。但是我們還有更通用的切片型別。請考慮以下陣列：

```rust
let a = [1, 2, 3, 4, 5];
```

就像我們引用一部分的字串一樣，我們可以這樣引用一部分的字串：

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

此切片的型別為 `&[i32]`，它和字串運作的方式一樣，儲存了切片的第一個元素以及總長度。你以後會對其他集合也使用這樣的切片。我們會在第八章討論這些集合的更多細節。

## 總結

所有權、借用與切片的概念讓 Rust 可以在編譯時期就確保記憶體安全。Rust 程式語言讓你和其他程式語言一樣控制你的記憶體使用方式，但是會在擁有者離開作用域時自動清除擁有的資料，讓你不必在編寫或除錯額外的程式碼。

所有權影響了 Rust 很多其它部分執行的方式，所以我們在書中之後討論這些概念。讓我們繼續到第五章，看看如何用 `struct` 將資料組合在一起。

[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch04-03-slices.md)
> - updated: 2020-09-08