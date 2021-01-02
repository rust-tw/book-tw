## 函式語言功能：疊代器與閉包

Rust 的閉包（closures）是個你能賦值給變數或作為其他函式引數的匿名函式。你可以在某處建立閉包，然後在不同的地方呼叫閉包並執行它。而且不像函式，閉包可以從它們所定義的作用域中獲取數值。我們將會解釋這些閉包功能如何允許程式碼重用以及自訂行為。

### 透過閉包建立抽象行為

讓我們處理一個範例是當儲存閉包並在之後才執行是很實況的情況。在過程中，我們會討論到閉包語法、型別推導以及特徵。

讓我們考慮以下假設情境：我們在一家新創公司上班並正在推出一支會產生自訂重訓方案的應用程式。其後端就是用 Rust 寫的，且產生重訓方案的演算法有很多因素要考量，像是使用者的年齡、身高體重指數、健身喜好、最近鍛鍊的項目以及他們指定的重訓強度。此例中實際使用的演算法並不重要，重要的是此運算會花費數秒鐘。我們只想要在我們需要時呼叫此演算法並只會呼叫一次，讓使用者不會等待太久。

我們會模擬這個假設的演算法為函式 `simulated_expensive_calculation`，如範例 13-1 所示，他會印出 `緩慢計算中...`、等待兩秒鐘，然後回傳我們傳入的數值。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs:here}}
```

<span class="caption">範例 13-1：一支作為假想需要運算 2 秒鐘的函式</span>

接下來 `main` 函式會包含此健身應用程式中最重要的部分。此函式代表當使用者請求健身方案時應用程式會呼叫的程式碼。由於應用程式的前端與我們的閉包使用並沒有任何關聯，我們將會用寫死的數值來代表我們程式的輸入並印出輸出結果。

必要的輸入如以下所示：

* 使用者想要的重訓強度，用來指明他們想要的訓練是低強度訓練或是高強度訓練
* 一個用來產生重訓方案變化的隨機數值

輸出結果會是建議的重訓方案，範例 13-2 展示了我們使用的 `main` 函式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">範例 13-2：一支有寫死的數值來模擬使用者輸入與隨機生成數字的 `main` 函式</span>

為了方便我們將變數 `simulated_user_specified_value` 寫死為 10 且變數 `simulated_random_number` 寫死為 7。在實際程式中，我們會從應用程式前端取得強度數字，並用 `rand` crate 來產生隨機數字，如同第二章猜謎遊戲所做的一樣。`main` 函式會用模擬的輸入數值呼叫 `generate_workout` 函式。

現在我們有了內容，讓我們看看演算法吧。範例 13-3 的函式 `generate_workout` 包含了應用程式的業務邏輯，也就是我們在此例最在意的地方。此例中接下來的程式碼都在此函式中進行：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">範例 13-3：依據輸入印出重訓方案並呼叫函式 `simulated_expensive_calculation` 的業務邏輯</span>

範例 13-3 的程式碼會多次呼叫計算緩慢的函式。第一個 `if` 區塊會呼叫 `simulated_expensive_calculation` 兩次，然後在 `else` 區塊內的 `if` 不會呼叫它，然後第二個 `else` 會呼叫它一次。

`generate_workout` 函式預期的行為是先檢查使用者想要低強度的重訓方案（也就是強度低於 25）或者高強度的方案（大於等於 25）。

低強度重訓方案會依據我們麼體的複雜演算法來建議一些伏地挺身和仰臥起坐。

如果使用者想要高強度重用，就會有額外的邏輯：如果應用程式產生的隨機數字是 3 的話，應用程式會建議休息並多喝水；如果不是的話，使用者會依據複雜演算法得到數分鐘的跑步訓練。

此程式碼能夠應付業務邏輯了，但是假設未來資料科學團隊決定要求我們需要更改我們呼叫 `simulated_expensive_calculation` 函式的方式。為了簡化這些更新步驟，我們想要重構此程式碼好讓 `simulated_expensive_calculation` 只會呼叫一次。同時我們也想要去掉我們目前呼叫兩次的多餘程式碼，我們不希望再對此程序加上更多的函式呼叫。也就是說，我們不希望在沒有需要取得結果時呼叫程式碼，且我們希望它只會被呼叫一次。

#### 透過函式重構

我們可以用許多方式重構此重訓程式。首先我們先將重複呼叫 `simulated_expensive_calculation` 的地方改成變數，如範例 13-4 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs:here}}
```

<span class="caption">範例 13-4：提取 `simulated_expensive_calculation` 的呼叫到同個位置並用變數 `expensive_result` 儲存結果</span>

此變更統一了所有 `simulated_expensive_calculation` 的呼叫並解決第一個 `if` 區塊重複呼叫函式兩次的問題。不幸的是，現在我們一定得呼叫此函式並在所有情形下都得等待，這包含沒有使用到此結果的 `if` 區塊。

我們想在程式某處定義程式碼，並在我們確實需要它時**執行**程式碼就好。這就是閉包能使用的場合！

#### 透過閉包重構來儲存程式碼

與其在 `if` 區塊之前就呼叫 `simulated_expensive_calculation` 函式，我們可以定義一個閉包並將**閉包**存入變數中，而不是儲存函式呼叫的結果，如範例 13-5 所示。我們可以將 `simulated_expensive_calculation` 的本體移入這個閉包中。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs:here}}
```

<span class="caption">範例 13-5：定義閉包並存入 `expensive_closure` 變數中</span>

閉包定義位於 `expensive_closure` 賦值時 `=` 後面的部分。要定義閉包，我們先從一對直線（`|`）開始，其內我們會指定閉包的參數，選擇此語法的原因是因為這與 Smalltalk 和 Ruby 的閉包定義類似。此閉包有一個參數 `num`，如果我們想要不止一個的話，我們可以用逗號來分隔，像是這樣 `|param1, param2|`。

在參數之後，我們加上大括號來作為閉包的本體，不過如果閉包本體只是一個表達式的話就不必這樣寫。在閉包結束後，也就是大括號之後，我們要加上分號才能完成 `let` 陳述式的動作。在閉包本體最後一行的回傳數值就會是當閉包被呼叫時的回傳數值，因為該行沒有以分號做結尾，就像函式本體一樣。

注意到此 `let` 陳述式代表 `expensive_closure` 包含了匿名函式的**定義**，而不是呼叫匿名函式的**回傳數值**。回想一下我們使用閉包是為了讓我們能在某處定義程式碼、儲存這段程式碼然後在之後別的地方呼叫它。我們想呼叫的程式碼現在儲存在 `expensive_closure` 中。

有了閉包定義，我們就可以變更 `if` 區塊內的程式碼呼叫閉包來執行其程式碼並取得結果數值。我們呼叫閉包的方式與呼叫函式一樣：我們指定握有閉包定義的變數名稱，然後在括號內加上我們想使用的引數數值，如範例 13-6 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs:here}}
```

<span class="caption">範例 13-6：呼叫我們定義的 `expensive_closure`</span>

現在耗時的計算只會在一處呼叫了，而且我們只會在需要結果時才會執行該程式碼。

然而我們又重新引入了範例 13-3 其中一個問題，我們仍然會在第一個 `if` 區塊呼叫閉包兩次，也就是會呼叫耗時的程式碼兩次，讓使用者需要多等一倍的時間。我們可以透過在 `if` 區塊內建立變數並取得呼叫閉包的結果來修正這個問題，但是閉包還能提供我們另一種解決辦法。我們稍後會介紹這個解決辦法。但在這之前讓我們先討論為何閉包定義中不用型別詮釋，以及與閉包相關的特徵。

### 閉包型別推導與詮釋

閉包不必像 `fn` 函式那樣要求你要詮釋參數或回傳值的型別。函式需要型別詮釋是因為它們是顯式公開給使用者的介面。嚴格定義此介面是很重要的，這能確保每個人同意函式使用或回傳的數值型別為何。但是閉包並不是為了對外公開使用，它們儲存在變數且沒有名稱能公開給我們函式庫的使用者。

閉包通常很短，而且只與小範圍內的程式碼有關，而非適用於任何場合。有了這樣限制的環境，編譯器能可靠地推導出參數與回傳值的型別，如同其如何推導出大部分的變數型別一樣。

要求開發者得為這些小小的匿名函式詮釋型別的話會變得很惱人且非常多餘，因為編譯器早就有足夠的資訊能推導出來了。

至於變數的話，雖然不是必要的，但如果我們希望能夠增加閱讀性與清楚程度，我們還是可以加上型別詮釋。要對我們在範例 13-5 定義的閉包詮釋型別的話，會如以下範例 13-7 所定義的所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs:here}}
```

<span class="caption">範例 13-7：對閉包加上選擇性的參數與回傳值型別詮釋</span>

加上型別詮釋後，閉包的語法看起來就更像函式的語法了。以下對一個參數加 1 的函式定義語法與有相同行為的閉包的比較表。我們加了一些空格來對齊相對應的部分。這顯示了閉包語法和函式語法有多類似，只是改用直線以及有些語法是選擇性的。

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行顯示的是函式定義，而第二行則顯示有完成型別詮釋的閉包定義。第三行移除了閉包定義的型別詮釋，然後第四行移除了大括號，因為閉包本體只有一個表達式，所以這是選擇性的。這些都是有效的定義，並會在被呼叫時產生相同行為。而 `add_one_v3` 和 `add_one_v4` 一定要被呼叫，這樣編譯器才能從它們的使用方式中推導出型別。

閉包定義會對每個參數與它們的回傳值推導出一個實際型別。舉例來說，範例 13-8 展示一支只會將收到的參數作為回傳值的閉包定義。此閉包並沒有什麼意義，純粹作為範例解釋。注意到我們沒有在定義中加上任何型別詮釋。如果我們嘗試呼叫閉包兩次，一次使用 `String` 作為引數，而另一次使用 `u32` 的話，我們就會得到錯誤。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs:here}}
```

<span class="caption">範例 13-8：嘗試呼叫被推導出兩個不同型別的閉包</span>

編譯器會給我們以下錯誤：

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

當我們第一次使用 `String` 數值呼叫 `example_closure` 時，編譯器會推導 `x` 與閉包回傳值的型別為 `String`。這樣 `example_closure` 閉包內的型別就會鎖定，然後我們如果對同樣的閉包嘗試使用不同的型別的話，我們就會得到型別錯誤。

### 透過泛型參數與 `Fn` 特徵儲存閉包

讓我們回到我們的重訓生成應用程式。在範例 13-6 中，我們的程式碼仍然會呼叫耗時的閉包不止一次。其中一個解決此問題的選項是將耗時閉包的結果存入變數中，並在我們需要結果的地方使用該變數，而不是再呼叫閉包一次。不過此方法可能會增加很多重複的程式碼。

幸運的是我們還有另一個解決辦法。我們可以建立一個結構體來儲存閉包以及呼叫閉包的結果數值。此結構體只會在我們需要結果數值時執行閉包，然後它會獲取結果數值，所以我們的程式碼就不必負責儲存要重複使用的結果。你可能會聽過這種模式叫做**記憶化（memoization）**或**惰性求值（lazy evaluation）**。

要定義一個結構體儲存一個閉包，我們需要指定閉包的型別，因為結構體定義需要知道它每個欄位的型別。每個閉包實例都有自己獨特的匿名型別，也就是說就算有兩個閉包的簽名一模一樣，它們的型別還是會被視為不同的。要定義有使用閉包的結構體、枚舉或函式參數的話，我們可以使用在第十章所提到的泛型與特徵界限。

標準函式庫有提供 `Fn` 特徵，所有閉包都有實作至少以下一種特徵：`Fn`、`FnMut` 或 `FnOnce`。我們會在[「透過閉包獲取環境」](#透過閉包獲取環境)<!-- ignore -->段落中討論這些特徵的不同。在此例中，我們可以使用 `Fn` 特徵。

我們在 `Fn` 特徵界限加上了型別來表示閉包參數與回傳值必須擁有的型別。在此例中，我們的閉包參數型別為 `u32` 且回傳 `u32`，所以我們指定的特徵界限為 `Fn(u32) -> u32`。

範例 13-9 顯示了擁有一個閉包與一個 Option 結果數值的 `Cacher` 結構體定義。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs:here}}
```

<span class="caption">範例 13-9：定義結構體 `Cacher` 而 `calculation` 會存有閉包且 `value` 存放  Option 結果</span>

`Cacher` 結構體有個欄位 `calculation` 其泛型型別為 `T`。`T` 的特徵界限指定這是一個使用 `Fn` 特徵的閉包。任何我們想存入的 `calculation` 欄位的閉包都必須只有一個 `u32` 參數（在 `Fn` 後方的括號內指定）以及回傳一個 `u32`（在 `->` 之後指定）。

> 注意：函式也會實作這三個 `Fn` 特徵。如果我們想做的事情不需要獲取環境數值，我們可以使用實現 `Fn` 特徵的函式而非閉包。

`value` 欄位型別為 `Option<u32>`。在我們執行閉包前，`value` 會是 `None`。當有程式碼使用 `Cacher` 要求取得閉包**結果**時，`Cacher` 就會在那時候執行閉包並以 `Some` 變體儲存結果到 `value` 欄位。然後如果有程式碼再次要求閉包結果時，我們就不必再執行閉包一次，可以靠 `Cacher` 回傳 `Some` 變體內的結果。

我們討論這個有關 `value` 欄位的邏輯定義就如範例 13-10 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

<span class="caption">範例 13-10：`Cacher` 的快取（Caching）邏輯</span>

我們想要 `Cacher` 來管理結構體的欄位數值，而不是讓呼叫者有機會直接改變這些欄位的數值，所以這些欄位是私有的。

`Cacher::new` 函式接收一個泛型參數 `T`，其特徵界限與我們在 `Cacher` 結構體定義的是相同的。接著 `Cacher::new` 回傳一個 `Cacher` 實例，其 `calculation` 欄位擁有指定的閉包而 `value` 欄位則是 `None`，因為我們還沒有執行閉包。

當呼叫者需要閉包計算的結果時，不是直接呼叫閉包，而是呼叫 `value` 方法。此方法會檢查我們的 `self.value` 是否已經有個結果數值在 `Some` 內。如果有的話，它會回傳 `Some` 內的數值而不用再次執行閉包。

如果 `self.value` 是 `None`，程式碼會呼叫存在 `self.calculation` 的閉包、儲存結果到 `self.value` 以便未來使用，並回傳數值。

範例 13-11 展示我們如何在範例 13-6 的 `generate_workout` 函式中使用此 `Cacher` 結構體。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<span class="caption">範例 13-11：在函式 `generate_workout` 使用 `Cacher` 來抽象化快取邏輯</span>

不同於將閉包儲存給變數，我們建立一個新的 `Cacher` 實例來儲存閉包。然後在每個我們需要結果的地方，我們呼叫 `Cacher` 實例的 `value` 方法。我們要呼叫 `value` 方法幾次都行，或者不叫也行，無論如何耗時計算最多就只會被執行一次。

請嘗試從範例 13-2 的 `main` 函式執行此程式。變更 `simulated_user_specified_value` 與 `simulated_random_number` 的數值來驗證看看在所有情況下與數個 `if` 和 `else` 區塊中，`緩慢計算中...` 只會出現一次且只有在需要時才會出現。`Cacher` 負責確保我們不會呼叫超過耗時計算所需的邏輯，讓 `generate_workout` 可以專注在業務邏輯。

### `Cacher` 實作的限制

快取數值是個廣泛實用的行為，我們可能會希望在程式碼中的其他不同閉包也使用到。然而目前`Cacher` 的實作有兩個問題可能會在不同場合重複使用變得有點困難。

第一個問題是 `Cacher` 實例假設它永遠會從方法 `value` 的參數 `arg` 中取得相同數值，所以說以下 `Cacher` 的測試就會失敗：

```rust,ignore,panics
{{#rustdoc_include ../listings/ch13-functional-features/no-listing-01-failing-cacher-test/src/lib.rs:here}}
```

此測試透過一個回傳傳入值的閉包建立一個新的 `Cacher` 實例。我們透過一個 `arg` 數值 1 與另一個 `arg` 數值 2 來呼叫此 `Cacher` 實例的 `value` 方法兩次，且我們預期 `arg` 為 2 的 `value` 會回傳 2。

使用範例 13-9 和範例 13-10 的 `Cacher` 實作執行此測試的話，測試會在 `assert_eq!` 失敗並附上此訊息：

```console
{{#include ../listings/ch13-functional-features/no-listing-01-failing-cacher-test/output.txt}}
```

問題在於我們第一次會使用 1 呼叫 `c.value`，`Cacher` 實例會儲存 `Some(1)` 給 `self.value`。因此無論我們再傳入任何值給 `value` 方法，它永遠只會回傳 1。

我們可以嘗試將 `Cacher` 改成儲存雜湊映射（hash map）而非單一數值。雜湊映射的鍵會是傳入的 `arg` 數值，而雜湊映射的值則是用該鍵呼叫閉包的結果。所以不同於查看 `self.value` 是 `Some` 還是 `None` 值， `value` 函式將會查看 `arg` 有沒有在雜湊映射內，而如果有的話就會傳對應數值。如果沒有的話，`Cacher` 會呼叫閉包並儲存 `arg` 數值與對應的結果數值到雜湊映射中。

第二個問題是目前的 `Cacher` 實作只會接受參數型別為 `u32` 並回傳 `u32` 的閉包。舉例來說，我們可能會想要快取給予字串並回傳 `usize` 的閉包結果數值。要修正此問題，你可以嘗試加上更多泛型參數來增加 `Cacher` 功能的彈性。

### 透過閉包獲取環境

在重訓生成範例中，我們只將閉包作為行內匿名函式。但是閉包還有個函式所沒有的能力：它們可以獲取它們的環境並取得在它們所定義的作用域內的變數。

範例 13-12 有一個儲存在變數 `equal_to_x` 的閉包，其使用變數 `x` 來取得閉包周圍的環境。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/main.rs}}
```

<span class="caption">範例 13-12：引用周圍作用域中變數的閉包範例</span>

`x` 在此雖然不是 `equal_to_x` 的參數，`equal_to_x` 閉包卻允許使用變數 `x`，因為它與 `equal_to_x` 都定義在同個作用域。

我們用函式就做不到，如果我們嘗試執行以下範例，我們的程式碼會無法編譯：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/no-listing-02-functions-cant-capture/src/main.rs}}
```

我們得到以下錯誤：

```console
{{#include ../listings/ch13-functional-features/no-listing-02-functions-cant-capture/output.txt}}
```

編譯器甚至會提醒我們這只有閉包才能做到！

當閉包從它的環境獲取數值時，它會在閉包本體中使用記憶體來儲存這個數值。這種儲存記憶體的方式會產生額外開銷。在更常見的場合中，也就是不需要獲取程式碼的環境時，我們並不希望產生這種開銷。因為函式並不允許獲取它們的環境，定義與使用函式就不會產生這種開銷。

閉包可以用三種方式獲取它們的環境，這剛好能對應到函式取得參數的三種方式：取得所有權、可變借用與不可變借用。這就被定義成以下三種 `Fn` 特徵：

* `FnOnce` 會消耗周圍作用域中，也就是閉包的**環境**，所獲取變數。要消耗掉所獲取的變數，閉包必須取得這些變數的所有權並在定義時將它們移入閉包中。特徵名稱中的 `Once` 指的是因為閉包無法取得相同變數的所有權一次以上，所以它只能被呼叫一次。
* `FnMut` 可以改變環境，因為它取得的是可變的借用數值。
* `Fn` 則取得環境中不可變的借用數值。

當你建立閉包時，Rust 會依據閉包如何使用環境的數值來推導該使用何種特徵。所有的閉包都會實作 `FnOnce` 因為它們都可以至少被呼叫一次。不會移動獲取變數的閉包還會實作 `FnMut`，最後不需要向獲取變數取得可變引用的閉包會再實作 `Fn`。在範例 13-12 中，`equal_to_x` 閉包會取得 `x` 的不可變借用（所以 `equal_to_x` 擁有 `Fn` 特徵），因為閉包本體只會讀取 `x` 數值。

如果你希望強制閉包會取得周圍環境數值的所有權，你可以在參數列表前使用 `move` 關鍵字。此技巧在要將閉包傳給新的執行緒以便將資料移動到新執行緒時會很實用。

當我們在第十六章討論並行的時候，我們會遇到更多 `move` 閉包的範例。現在的話可以先看看範例 13-12 怎麼使用 `move` 關鍵字到閉包定義中 ，並使用向量而非整數，因為整數可以被拷貝而不是移動。注意此程式還不能編譯過。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/no-listing-03-move-closures/src/main.rs}}
```

我們會獲得以下錯誤：

```console
{{#include ../listings/ch13-functional-features/no-listing-03-move-closures/output.txt}}
```

當閉包定義時，數值 `x` 會移入閉包中，因為我們加上了 `move` 關鍵字。閉包因此取得 `x` 的所有權，然後 `main` 就會不允許 `x` 在 `println!` 陳述式中使用。移除此例的 `println!` 就能修正問題。

大多數要指定 `Fn` 特徵界限時，你可以先從 `Fn` 開始，然後編譯器會依據閉包本體的使用情況來告訴你該使用 `FnMut` 或 `FnOnce`。


接下來為了講解閉包獲取環境的行為很適合用於函式參數的情形，讓我們移至下個主題：疊代器。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch13-01-closures.md)
> - updated: 2020-09-17
