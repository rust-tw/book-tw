## 閉包：獲取其環境的匿名函式

Rust 的閉包（closures）是個你能賦值給變數或作為其他函式引數的匿名函式。你可以在某處建立閉包，然後在不同的地方呼叫閉包並執行它。而且不像函式，閉包可以從它們所定義的作用域中獲取數值。我們將會解釋這些閉包功能如何允許程式碼重用以及自訂行為。

### 透過閉包獲取環境

我們首先會來研究我們如何用閉包來獲取定義在環境的數值並在之後使用。讓我們考慮以下假設情境：每隔一段時間，我們的襯衫公司會送出獨家限量版襯衫給郵寄清單的某位顧客來作為宣傳手段。郵寄清單的顧客可以在他們的設定中加入他們最愛的顏色。如果被選中的人有設定最愛顏色的話，他們就會獲得該顏色的襯衫。如果他們沒有指定任何最愛顏色的話，公司就會選擇目前顏色最多的選項。

要實作的方式有很多種。舉例來說，我們可以使用一個列舉叫做 `ShirtColor` 然後其變體有 `Red` 和 `Blue`（為了簡潔我們限制顏色的種類）。我們用 `Inventory` 來代表公司的庫存，然後用 `shirts` 欄位來包含 `Vec<ShirtColor>` 來代表目前庫存有的襯衫顏色。定義在 `Inventory` 的 `giveaway` 方法會取得免費襯衫得主的選擇性襯衫顏色偏好，然後回傳他們會拿到的襯衫顏色。如範例 13-1 所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">範例 13-1：襯衫公司送禮的情境</span>

定義在 `main` 中的 `store` 在這次的限量版宣傳中的庫存有兩件藍色襯衫與一件紅色襯衫。我們呼叫了 `giveaway` 方法兩次，一次是給偏好紅色襯衫的使用者，另一次則是給無任何偏好的使用者。

再次強調這可以用各種方式實作，只是在此我們想專注在閉包，所以除了用到我們已經學過的概念以外，`giveaway` 方法中還使用了閉包。在 `giveaway` 方法中，我們從參數型別 `Option<ShirtColor>` 取得使用者偏好，然後對 `user_preference` 呼叫 `unwrap_or_else` 方法。[`Option<T>` 的 `unwrap_or_else` 方法][unwrap-or-else]<!-- ignore -->定義在標準函式庫中。它接收一個引數：一個沒有任何引數的閉包然後會回傳數值 `T`（該型別為 `Option<T>` 的 `Some` 儲存的型別，在此例中就是 `ShirtColor`）。如果 `Option<T>` 是 `Some` 變體，`unwrap_or_else` 就會回傳 `Some` 裡的數值。如果 `Option<T>` 是 `None` 變體，`unwrap_or_else` 會呼叫閉包並回傳閉包回傳的數值。

我們寫上閉包表達式 `|| self.most_stocked()` 作為 `unwrap_or_else` 的引數。這是個沒有任何參數的閉包（如果閉包有參數的話，它們會出現在兩條直線中間）。閉包本體會呼叫 `self.most_stocked()`。我們直接在此定義閉包，然後 `unwrap_or_else` 的實作就會在需要結果時執行閉包。

執行此程式的話就會印出：

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

這裡值得注意的是我們對當前 `Inventory` 實例傳入的是一個呼叫 `self.most_stocked()` 的閉包。標準函式庫不需要知道我們定義的任何型別像  `Inventory` 與 `ShirtColor`，或是在此情境中我們需要使用的任何邏輯，閉包就會獲取 `Inventory` 實例的不可變參考 `self`，然後傳給我們在 `unwrap_or_else` 方法中指定的程式碼。反之，函式就無法像這樣獲取它們周圍的環境。

### 閉包型別推導與詮釋

函式與閉包還有更多不同的地方。閉包通常不必像 `fn` 函式那樣要求你要詮釋參數或回傳值的型別。函式需要型別詮釋是因為它們是顯式公開給使用者的介面。嚴格定義此介面是很重要的，這能確保每個人同意函式使用或回傳的數值型別為何。但是閉包並不是為了對外公開使用，它們儲存在變數且沒有名稱能公開給我們函式庫的使用者。

閉包通常很短，而且只與小範圍內的程式碼有關，而非適用於任何場合。有了這樣限制的環境，編譯器能可靠地推導出參數與回傳值的型別，如同其如何推導出大部分的變數型別一樣。（但在有些例外情形下編譯器還是需要閉包的型別詮釋）

至於變數的話，雖然不是必要的，但如果我們希望能夠增加閱讀性與清楚程度，我們還是可以加上型別詮釋。要在閉包詮釋型別的話，就會如範例 13-2 的定義所示。在此範例中，我們定義一個閉包並儲存至一個變數中，而非像範例 13-1 我們將閉包作為引數傳入。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">範例 13-2：對閉包加上選擇性的參數與回傳值型別詮釋</span>

加上型別詮釋後，閉包的語法看起來就更像函式的語法了。我們在此定義了一個對參數加 1 的函式，以及一個有相同行為的閉包做為比較。我們加了一些空格來對齊相對應的部分。這顯示了閉包語法和函式語法有多類似，只是改用直線以及有些語法是選擇性的。

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行顯示的是函式定義，而第二行則顯示有完成型別詮釋的閉包定義。在第三行我們移除了閉包定義的型別詮釋，然後在第四行我們移除了大括號，因為閉包本體只有一個表達式，所以這是選擇性的。這些都是有效的定義，並會在被呼叫時產生相同行為。而 `add_one_v3` 和 `add_one_v4` 一定要被呼叫，這樣編譯器才能從它們的使用方式中推導出型別。這就像 `let v = Vec::new();` 需要型別詮釋，或是有某種型別的數值插入 `Vec` 中，Rust 才能推導出型別。

對於閉包定義，編譯器會對每個參數與它們的回傳值推導出一個實際型別。舉例來說，範例 13-3 展示一支只會將收到的參數作為回傳值的閉包定義。此閉包並沒有什麼意義，純粹作為範例解釋。注意到我們沒有在定義中加上任何型別詮釋。由於沒有型別詮釋，我們可以用任何型別來呼叫閉包，像我們第一次呼叫就用 `String`。如果我們接著嘗試用整數呼叫 `example_closure`，我們就會得到錯誤。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">範例 13-3：嘗試呼叫被推導出兩個不同型別的閉包</span>

編譯器會給我們以下錯誤：

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

當我們第一次使用 `String` 數值呼叫 `example_closure` 時，編譯器會推導 `x` 與閉包回傳值的型別為 `String`。這樣 `example_closure` 閉包內的型別就會鎖定，然後我們如果對同樣的閉包嘗試使用不同的型別的話，我們就會得到型別錯誤。

### 獲取參考或移動所有權

閉包要從它們周圍環境取得數值有三種方式，這能直接對應於函式取得參數的三種方式：不可變借用、可變借用，與取得所有權。閉包會依照函式本體如何使用獲取的數值，來決定要用哪種方式。

在範例 13-4 中，我們定義一個閉包來獲取 `list` 向量的不可變參考，因為它只需要不可變參考就能印出數值：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">範例 13-4：定義並呼叫會獲取不可變參考的閉包</span>

此範例還示範了變數能綁定閉包的定義，然後我們之後就可以用變數名稱加上括號來呼叫閉包，這樣變數名稱就像函式名稱一樣。

由於我們可以同時擁有 `list` 的多重不可變參考，`list` 在閉包定義前、在閉包定義後閉包呼叫前以及閉包呼叫時的程式碼中都是能使用的。此程式碼就會編譯、執行並印出：

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

接著在範例 13-5 中我們改變閉包本體，對 `list` 向量加上一個元素。這樣閉包現在就會獲取可變參考：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">範例 13-11：定義並呼叫會獲取可變參考的閉包</span>

此程式碼會編譯、執行並印出：

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

注意到在 `borrows_mutably` 閉包的定義與呼叫之間的 `println!` 不見了：當 `borrows_mutably` 定義時，它會獲取 `list` 的可變參考。我們在閉包呼叫之後沒有再使用閉包，所以可變參考就結束。在閉包定義與呼叫之間，利用不可變參考印出輸出是不允許的，因為在可變參考期間不能再有其他參考。你可以試試看在那加上 `println!` 然後看看會收到什麼錯誤訊息！

如果你想要強迫閉包取得周圍環境數值的所有權的話，你可以在參數列表前使用 `move` 關鍵字。

此技巧適用於將閉包傳給新執行緒來移動資料，讓新的執行緒能擁有該資料。我們會在第十六章討論並行時，介紹為何你會想使用它們。但現在讓我們簡單探索怎麼在閉包使用 `move` 關鍵字開個新的執行緒就好。範例 13-6 更改了範例 13-4 讓向量在新的執行緒印出而非原本的主執行緒：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<span class="caption">範例 13-6：使用 `move` 來迫使執行緒的閉包取得 `list` 的所有權</span>

我們開了一個新的執行緒，將閉包作為引數傳入，閉包本體會印出 `list`。在範例 13-4 中，閉包只用不可變參考獲取 `list`，因為要印出 `list` 的需求只要這樣就好。而在此例中，儘管閉包本體仍然只需要不可變參考就好，我們在閉包定義時想要指定 `list` 應該要透過 `move` 關鍵字移入閉包。新的執行緒可能會在主執行緒之前結束，或者主執行緒也有可能會先結束。如果主執行緒持有 `list` 的所有權卻在新執行緒之前結束並釋放 `list` 的話，執行緒拿到的不可變參考就會無效了。因此編譯器會要求 `list` 移入新執行緒的閉包中，這樣參考才會有效。嘗試看看將 `move` 關鍵字刪掉，或是在主執行緒的閉包定義之後使用 `list`，看看你會收到什麼編譯器錯誤訊息！

### `Fn` 特徵以及將獲取的數值移出閉包

一但閉包從其定義的周圍環境獲取了數值的參考或所有權（也就是說被**移入**閉包中），閉包本體的程式碼會定義閉包在之後執行結束後要對參考或數值做什麼事情（也就是說被**移出**閉包）。閉包本體可以做以下的事情：將獲取的數值移出閉包、改變獲取的數值、不改變且不移動數值，或是一開始就不從環境獲取任何值。

閉包從周圍環境獲取並處理數值的方式會影響閉包會實作哪種特徵，而這些特徵能讓函式與結構體決定它們想使用哪種閉包。閉包會依照閉包本體處理數值的方式，自動實作一種或多種 `Fn` 特徵：

1. `FnOnce` 適用於可以呼叫一次的閉包。所有閉包至少都會有此特徵，因為所有閉包都能被呼叫。會將獲取的數值移出本體的閉包只會實作 `FnOnce` 而不會再實作其他 `Fn` 特徵，因為這樣它只能被呼叫一次。
2. `FnMut` 適用於不會將獲取數值移出本體，而且可能會改變獲取數值的閉包。這種閉包可以被呼叫多次。
3. `Fn` 適用於不會將獲取數值移出本體，而且不會改變獲取數值或是甚至不從環境獲取數值的閉包。這種閉包可以被呼叫多次，而且不會改變周圍環境，這對於並行呼叫閉包多次來說非常重要。

讓我們來觀察範例 13-1 中 `Option<T>` 用到的 `unwrap_or_else` 方法定義：

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

回想一下 `T` 是一個泛型型別，代表著 `Option` 的 `Some` 變體內的數值型別。型別 `T` 同時也是函式 `unwrap_or_else` 的回傳型別：比如說對 `Option<String>` 呼叫  `unwrap_or_else` 的話就會取得 `String`。

接著注意到函式 `unwrap_or_else` 有個額外的泛型型別參數 `F`。型別 `F` 是參數 `f` 的型別，也正是當我們呼叫 `unwrap_or_else` 時的閉包。

泛型型別 `F` 指定的特徵界限是 `FnOnce() -> T`，也就是說 `F` 必須要能夠呼叫一次、不帶任何引數然後回傳 `T`。在特徵界限中使用 `FnOnce` 限制了 `unwrap_or_else` 只能呼叫 `f` 最多一次。在 `unwrap_or_else` 本體中，如果 `Option` 是 `Some` 的話，`f` 就不會被呼叫。如果 `Option` 是 `None` 的話，`f` 就會被呼叫一次。由於所有閉包都有實作 `FnOnce`，`unwrap_or_else` 能接受大多數各種不同的閉包，讓它的用途非常彈性。

> 注意：函式也可以實作這三種 `Fn` 特徵。如果我們不必獲取環境數值，在我們需要有實作其中一種 `Fn` 特徵的項目時，我們可以使用函式名稱而不必用到閉包。舉例來說，對於 `Option<Vec<T>>` 的數值，我們可以呼叫 `unwrap_or_else(Vec::new)` 在數值為 `None` 時取得新的空向量。

現在讓我們來看看標準函式庫中切片定義的 `sort_by_key` 方法，來觀察它和 `unwrap_or_else` 有什麼不同，以及為何 `sort_by_key` 的特徵界限使用的是 `FnMut` 而不是 `FnOnce`。閉包會取得一個引數，這會是該切片當下項目的參考，然後回傳型別 `K` 的數值以供排序。當你想透過切片項目的特定屬性做排序時，此函式會很實用。在範例 13-7 中，我們有個 `Rectangle` 實例的列表，然後我們使用 `sort_by_key` 透過 `width` 屬性由低至高排序它們：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">範例 13-7：使用 `sort_by_key` 依據寬度來排序長方形</span>

此程式碼會印出：

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key` 的定義會需要 `FnMut` 閉包的原因是因為它得呼叫閉包好幾次，對切片的每個項目都要呼叫一次。閉包 `|r| r.width` 沒有獲取、改變或移動周圍環境的任何值，所以它符合特徵界限的要求。

反之，範例 13-8 示範了一個只實作 `FnOnce` 特徵的閉包，因為它有將數值移出環境。編譯器不會允許我們將此閉包用在 `sort_by_key`：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">範例 13-8：嘗試在 `sort_by_key` 使用 `FnOnce` 閉包</span>

這裡嘗試用很糟糕且令人費解的方式計算 `list` 在排序時 `sort_by_key` 被呼叫了幾次。此程式碼嘗試計數的方式是把閉包周圍環境中型別為 `String` 的 `value` 變數放入 `sort_operations` 向量中。閉包會獲取 `value`，然後將 `value` 移出閉包，也就是將 `value` 的所有權轉移到 `sort_operations` 向量裡。此向量只能呼叫一次，嘗試呼叫第二次是無法成功的，因為 `value` 已經不存在於環境中了，無法再次放入 `sort_operations`！因此，此閉包僅實作了 `FnOnce`。當我們嘗試編譯此程式碼時，我們會收到錯誤訊息說明 `value` 無法移出閉包，因為閉包必須實作 `FnMut`：

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

錯誤訊息指出閉包本體將 `value` 移出環境的地方。要修正此問題的話，我們需要改變閉包本體，讓它不再將數值移出環境。要計算 `sort_by_key` 呼叫次數的話，在環境中放置一個計數器，然後在閉包本體增加其值是更直觀的計算方法。範例 13-9 的閉包就能用在 `sort_by_key`，因為它只獲取了 `num_sort_operations` 計數器的可變參考，因此可以被呼叫不只一次：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">範例 13-9：在 `sort_by_key` 使用 `FnMut` 閉包是允許的</span>

當我們要在函式或型別中定義與使用閉包時，`Fn` 特徵是很重要的。在下個段落中，我們將討論疊代器。疊代器有許多方法都需要閉包引數，所以隨著我們繼續下去別忘了複習閉包的用法！

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else