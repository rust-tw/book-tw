## 如何寫測試

測試是一種 Rust 函式來驗證非測試程式碼是否以預期的方式執行。測試函式的本體通常會做三件動作：

1. 設置任何所需要的資料或狀態。
2. 執行你希望測試的程式碼
3. 判定結果是否與你預期的相符。

讓我們看看 Rust 特地提供給測試的功能：包含 `test` 屬性（attribute）、一些巨集以及 `should_panic` 屬性。

### 測試函式剖析

最簡單的形式來看，測試在 Rust 中就是附有 `test` 屬性的函式。屬性（Attributes）是一種關於某段 Rust 程式碼的詮釋資料（metadata），其中一個例子是我們在第五章使用的 `derive` 屬性。要將一個函式轉換成測試函式，在 `fn` 前一行加上 `#[test]` 即可。當你用 `cargo test` 命令來執行你的測試時，Rust 會建構一個測試執行檔並執行標有 `test` 屬性的程式，並回報每個測試函式是否通過或失敗。

當我們用 Cargo 建立新的函式庫專案時，同時會自動建立一個擁有測試函式的測試模組。此模組能協助我們開始寫測試，讓你不必在每次建立新專案時，尋找特定結構體與測試函式的語法。你可以新增多少測試函式與多少測試模組都沒問題！

我們將會透過實驗測試產生的樣板而非實際測試任何程式碼，來探索測試如何運作的每個環節。然後我們會寫些現實世界會寫得測試，呼叫我們寫的程式碼並判定其行為是否正確。

讓我們建立個函式庫專案叫做 `adder`：

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

函式庫專案 `adder` 中的 *src/lib.rs* 檔案內容會長得像範例 11-1 所示。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs:here}}
```

<span class="caption">範例 11-1：透過 `cargo new` 自動產生的測試模組與函式</span>

現在我們先忽略開頭前兩行並專注在函式，看看它執行的。注意到 `fn` 上一行的 `#[test]` 詮釋：此屬性指出這是測試函式，所以測試者會知道此函式是用來測試的。我們也可以在 `tests` 模組中加入非測試函式來協助設置常見場景或是執行常見運算，所以我們需要在想要測試的函式前加上 `#[test]` 屬性。

函式本體使用 `assert_eq!` 巨集來判定 2 + 2 等於 4。此判定是作為典型測試的範例格式。讓我們執行它來看看此測試是否會通過。

`cargo test` 命令會執行專案中的所有測試，如範例 11-2 所示。

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">範例 11-2：執行自動產生的測試的輸出結果</span>

Cargo 會編譯並執行測試。在 `Compiling`、`Finished` 與 `Running` 之後會出現 `running 1 test` 此行。下一行會顯示自動產生的測試函式 `it_works` 以及測試執行的結果 `ok`。再來可以看到整體總結，`test result: ok.` 代表所有測試都有通過，然後 `1 passed; 0 failed` 指出所有測試成功或失敗的數量。

因為我們尚未有任何會忽略的程式碼，所以總結會顯示 `0 ignored`。我們也沒有過濾會執行的測試，所以總結最後顯示 `0 filtered out`。我們會在下個段落 [「控制程式如何執行」][controlling-how-tests-are-run]<!-- ignore --> 來討論忽略與過濾測試。

`0 measured` 的統計數值是指評測效能的效能測試。效能測試（Benchmark tests）在本書撰寫時，仍然僅在 nightly Rust 可用。請查閱[效能測試的技術文件][bench]來瞭解詳情。

[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html

測試輸出結果的下一部分，也就是 `Doc-tests adder`，是指任何技術文件測試的結果。我們還沒有任何技術文件測試，但是 Rust 可以編譯在 API 技術文件中的任何程式碼範例。此功能能幫助我們將技術文件與程式碼保持同步！我們會在第十四章的 [「將技術文件註解作為測試」][doc-comments]<!-- ignore -->段落討論如何寫技術文件測試。現在我們會先忽略 `Doc-tests` 的輸出結果。

讓我們變更程式碼的名稱來看看測試輸出會變成什麼。將 `it_works` 函式變更名稱，像是以下改成 `exploration` 這樣：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs:here}}
```

然後再執行一次 `cargo test`，輸出會顯示 `exploration` 而非 `it_works`：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

讓我們在加上另一個測試，不過這次我們要讓測試失敗！測試會在測試函式恐慌時失敗，每個測試會跑在新的執行緒（thread）上，然後當主執行緒看到測試執行緒死亡時，就會將該測試標記為失敗的。我們有在第九章提及引發恐慌最簡單的辦法，那就是呼叫 `panic!` 巨集。將它寫入新的測試 `another` 中，所以你在 *src/lib.rs* 的檔案中會看到向範例 11-3 這樣。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,panics
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">範例 11-3：新增第二個會失敗的測試，因為我們會呼叫 `panic!` 巨集</span>

使用 `cargo test` 再執行一次測試，輸出結果應該會像範例 11-4 這樣，顯示出我們的 `exploration` 測試通過但 `another` 失敗。

```text
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">範例 11-4：其中一個測試通過，而另一個失敗的輸出結果</span>

`test tests::another` 這行會顯示 `FAILED` 而非 `ok`。在獨立結果與總結之間出現了兩個新的段落，第一個段落會顯示每個測試失敗的原因細節。在此例中，`another` 因為 *src/lib.rs* 檔案中第十行的恐慌 `panicked at '此測試會失敗'` 而失敗。下一個段落則是會列出所有失敗的測試，要是測試很多且失敗測試輸出結果很長的話，此資訊就很實用。我們可以使用失敗測試的名稱來只執行這個測試以便除錯。我們會在[「控制程式如何執行」][controlling-how-tests-are-run]<!-- ignore -->段落討論更多執行測試的方法。

總結會顯示在最後一行，在此例中它表示我們有一個測試結果是 `FAILED`。也就是我們有一個測試通過，一個測試失敗。

現在你知道測試結果在不同場合看起來的樣子，讓我們來看看除了 `panic!` 以外對測試也很有幫助的巨集吧。

### 透過 `assert!` 巨集檢查結果

標準函式庫提供的 `assert!` 巨集可以在你要確保測試中的一些條件評估為 `true` 時使用。我們給予 `assert!` 巨集一個引數來計算出布林值。如果數值為 `true`，`assert!` 不會做任何動作然後測試就會通過。如果數值為 `false`，`assert!` 巨集會呼叫 `panic!` 巨集導致測試失敗。使用 `assert!` 巨集能幫助我們檢查我們的程式碼是否以我們預期的方式運作。

在第五章的範例 5-15，我們有結構體 `Rectangle` 與方法 `can_hold`，我們在範例 11-5 再看一次。讓我們將此程式碼寫入 *src/lib.rs* 檔案中，並寫些對它使用 `assert!` 巨集的測試。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">範例 11-5：第五章中的結構體 `Rectangle` 與其方法 `can_hold`</span>

`can_hold` 方法會回傳布林值，這代表它是 `assert!` 巨集的絕佳展示機會。在範例 11-6 中，我們寫了個測試來練習 `can_hold` 方法，我們建立了一個寬度為 8 長度為 7 的 `Rectangle` 實例，並判定它可以包含另一個寬度為 5 長度為 1 的 `Rectangle` 實例。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">範例 11-6：一支檢查一個大長方形是否能包含一個小長方形的 `can_hold` 測試</span>

注意到我們已經在 `tests` 模組中加了一行 `use super::*;`。`tests` 和一般的模組一樣都遵循我們在第七章[「引用模組項目的路徑」][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->提及的常見能見度規則。因為 `tests` 模組是內部模組，我們需要將外部模組的程式碼引入內部模組的作用域中。我們使用全域運算子（glob）讓外部模組定義的所有程式碼在此 `tests` 模組都可以使用。

我們將我們的測試命名為 `larger_can_hold_smaller`，然後我們建立兩個我們需要用到的 `Rectangle` 實例。然後我們呼叫 `assert!` 巨集並將 `larger.can_hold(&smaller)` 的結果傳給它。此表達式應該要回傳 `true`，所以我們的程式應該會通過。讓我們看看結果吧！

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

它通過了！讓我們再加另一個測試，這是是判定小長方形無法包含大長方形：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

因為函式 `can_hold` 的正確結果在此例為 `false`，我們需要將該結果反轉後才能傳給 `assert!` 巨集。因此我們的測試在 `can_hold` 回傳 `false` 時才會通過：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

兩個測試都過了！現在讓我們看看當我們在程式碼中引入程式錯誤的話，測試結果會為何。讓我們來改變 `can_hold` 方法的實作將比較時的大於符號改成小於符號：

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

執行測試的話現在就會顯示以下結果：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

我們的測試抓到了錯誤！因為 `larger.width` 是 8 而 `smaller.width` 是 5，`can_hold` 比較寬度時現在會回傳 `false`，因為 8 沒有比 5 小。

### 透過 `assert_eq!` 與 `assert_ne!` Macros測試相等

有一種常見的測試程式的方式是將程式碼的結果與你預期程式碼會回傳的數值做比較，檢查它們是否相等。你可以使用 `assert!` 巨集並傳入使用 `==` 運算子的表達式來辦到。不過這種測試方法是很常見的，所以標準函式庫提供了一對巨集 `assert_eq!` 與 `assert_ne!` 來讓你能更方便地測試。這兩個巨集分別比較兩個引數是否相等或不相等。如果判定失敗的話，它們還會印出兩個數值，讓我們能清楚看到*為何*測試失敗。相對地，`assert!` 巨集只會說明它在 `==` 表達式中取得 `false` 值，而不會告訴你導致 `false` 的那兩個值。

在範例 11-7 中，我們寫了個函式叫做 `add_two` 並對參數加上 `2` 然後回傳為結果。然後我們使用 `assert_eq!` 巨集來測試此函式。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs:here}}
```

<span class="caption">範例 11-7：使用 `assert_eq!` 巨集測試函式 `add_two`</span>

讓我們檢查後它的確通過了！

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

我們給予 `assert_eq!` 巨集的第一個引數 `4` 與呼叫 `add_two(2)` 的結果相等。測試的結果為 `test
tests::it_adds_two ... ok` 而 `ok` 就代表我們的測試通過了！

讓我們在我們的程式碼引入個錯誤，看看使使用 `assert_eq!` 的測試失敗時看起來為何。變更函式 `add_two` 的實作改成加 `3`：

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

再執行一次測試：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

我們的測試抓到了錯誤！`it_adds_two` 測試失敗了，並顯示`` assertion failed: `(left == right)` `` 然後接著顯示 `left` 是 `4` 且 `right` 是 `5`。此訊息非常有用，且能幫助我們開始除錯，它代表 `assert_eq!` 的引數 `left` 是 `4` 但是擁有 `add_two(2)` 的引數 `right` 卻是 `5`。

注意到在有些語言或測試框架中，判定兩個數值是否相等的函式的參數會稱作 `expected` 和 `actual`，然後它們會因為指定的引數順序而有差。但在 Rust 中它們被稱為 `left` 和 `right`，且我們預期的值與測試中程式碼產生的值之間的順序沒有任何影響。我們可以在此程式這樣寫判定 `assert_eq!(add_two(2), 4)`，而錯誤訊息就會顯示成 `` assertion failed: `(left == right)` ``，然後 `left` 會是 `5` 而 `right` 會是 `4`。

`assert_ne!` 巨集會在我們給予的兩個值不相等時通過，相等時失敗。此巨集適用於當我們不確定一個數值*會是*什麼樣子，但是我們確定知道如果我們程式如預期執行的話，該數值*不會*是某種樣子。舉例來說，如果我們要測試一個保證會以某種形式更改其輸入的函式，但輸入變更的方式是依照我們執行程式時的當天是星期幾來決定，此時最好的判定方式就是檢查函式的輸出不等於輸入。

`assert_eq!` 和 `assert_ne!` 巨集底下分別使用了 `==` 和 `!=` 運算子。當判定失敗時，巨集會透過除錯格式化資訊來顯示它們的引數，代表要比較的數值必須要實作 `PartialEq` 和 `Debug` 特徵。所有的基本型別與大多數標準函式庫中提供的型別都有實作這些特徵。對於你自己定義的結構體與枚舉，你需要實作 `PartialEq`，這樣該型別的數值才能判定相等或不相等。你需要實作 `Debug` 來顯示判定失敗時的數值。因為這兩個特徵都是可推導的特徵，就像第五章的範例 5-12 所寫的那樣，我們通常只要在你定義的結構體或枚舉前加上 `#[derive(PartialEq, Debug)]` 的詮釋就好。你可以查閱附錄 C [「可推導的特徵」][derivable-traits]<!-- ignore --> 來發現更多可推導的特徵。

### 加入自訂失敗訊息

你可以寫一個一個與失敗訊息一同顯示的自訂訊息，作為 `assert!`、`assert_eq!` 與 `assert_ne!` 巨集的選擇性引數。任何指定在 `assert!` 一個必要引數或 `assert_eq!` 和 `assert_ne!` 兩個必要引數後方的任何引數都會傳給 `format!` 巨集（我們在第八章[「使用 `+` 運算子或 `format!` 巨集串接字串」][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -->的段落討論過），所以你可以傳入一個包含 `{}` 佔位符（placeholder）的格式化字串以及其對應的數值。自訂訊息可以用來紀錄判定的意義，當測試失敗時，你可以更清楚知道程式碼的問題。

舉例來說，假設我們有個函式會以收到的名字向人們打招呼，而且我們希望測試我們傳入的名字有出現在輸出：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs:here}}
```

此函式的要求還沒完全確定，而我們招呼開頭的文字 `Hello` 很可能會在之後改變。我們決定當需求改變時，我們不想要得同時更新測試。所以我們不打算檢查 `greeting` 函式回傳的整個數值，我們只需要判定輸出有沒有包含輸入參數。

讓我們將錯誤引進程式中吧，將 `greeting` 改成不會包含 `name` 然後看看測試會怎麼失敗：

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

執行此程式會產生以下錯誤：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

此結果指出判定失敗以及發生的位置。現在要是錯誤訊息可以提供我們從 `greeting` 函式取得的數值就更好了。讓我們來在測試函式中加入自訂訊息，該訊息會是個格式化字串，並有個佔位符（placeholder）來填入我們從 `greeting` 函式取得的確切數值：

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

現在當我們執行測試，我們能從錯誤訊息得到更多資訊：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

我們可以看到我們實際從測試輸出拿到的數值，這能幫助我們除錯找到實際發生什麼，而不只是預期會是什麼。

### 透過 `should_panic` 檢查恐慌

除了檢查我們的程式碼有沒有回傳我們預期的正確數值，檢查我們的程式碼有沒有如我們預期處理錯誤條件也是很重要的。舉例來說，考慮我們在第九章範例 9-10 建立的 `Guess` 型別。其他使用 `Guess` 的程式碼保證會拿到數值為 1 到 100 的 `Guess` 實例。我們可以寫個會恐慌的程式，嘗試用範圍之外的數字建立 `Guess` 實例。

為此我們可以加上另一個屬性 `should_panic` 到我們的測試函式。此屬性讓函式的程式碼恐慌時才會通過測試，反之如果函式的程式碼沒有恐慌的話測試就會失敗。

範例 11-8 展示一支檢查 `Guess::new` 是否以我們預期的錯誤條件出錯的測試。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs:here}}
```

<span class="caption">範例 11-8：測試造成 `panic!` 的條件</span>

我們將 `#[should_panic]` 屬性置於 `#[test]` 屬性之後與測試函式之前。讓我們看看測試通過的結果：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

看起來不錯！現在讓我們將錯誤引入程式碼中，移除會讓 `new` 函式在數值大於 100 會恐慌的程式碼：

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

當我們執行範例 11-8 的測試，它就會失敗：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

我們在此情況得到的訊息並不是很有用，但是當我們查看測試函式，我們會看到它詮釋了 `#[should_panic]`。這個測試失敗代表測試函式內的程式碼沒有造成恐慌。

使用 `should_panic` 的測試可能會有點模棱兩可，因為它們只代表該程式碼會造成某種恐慌而已。`should_panic` 測試只要是有恐慌都會通過，就算是不同於我們預期發生的恐慌而造成的也一樣。要讓測試 `should_panic` 更精準的話，我們可以加上選擇性的 `expected` 參數到 `should_panic` 中。這樣測試就會確保錯誤訊息會包含我們所寫的文字。舉例來說，範例 11-9 更改了 `Guess` 讓 `new` 函式會依據數值太大或大小而有不同的錯誤訊息。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">範例 11-9：只在造成 `panic!` 的特定錯誤訊息會通過的測試</span>

此測試會通過是因為我們在 `should_panic` 屬性加上的 `expected` 就是 `Guess::new` 函式恐慌時的子字串。我們也可以指定整個恐慌訊息，在此例的話就是 `猜測數字必須小於等於 100，取得的數值是 200。`。你在 `should_panic` 所指定的預期參數取決於該恐慌訊息是獨特或動態的，以及你希望你的測試要多精準。在此例中，恐慌訊息的子訊息就足以確認測試函式中的程式碼會執行 `else if value > 100` 的分支。

為了觀察擁有 `expected` 訊息的 `should_panic` 失敗時會發生什麼事。讓我同樣再次將錯誤引入程式中，將 `if value < 1` 與 `else if value > 100` 的區塊本體對調：

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

這次當我們執行 `should_panic` 測試，它就會失敗：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

錯誤訊息表示此程式碼的確有如我們預期地恐慌，但是恐慌訊息並沒有包含預期的字串 `'猜測數字必須小於等於 100'`。在此例我們的會得到的恐慌訊息為 `猜測數字必須大於等於 1，取得的數值是 200。`這樣我們就能尋找錯誤在哪了！

### 在測試中使用 `Result<T, E>`

目前為止，我們的測試在失敗時就會恐慌。我們也可以寫出使用 `Result<T, E>` 的測試！以下是範例 11-1 的測試，不過重寫成 `Result<T, E>` 的版本並回傳 `Err` 而非恐慌：

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

`it_works` 函式現在有個回傳型別 `Result<(), String>`。在函式本體中，我們不再呼叫 `assert_eq!` 巨集，而是當測試成功時回傳 `Ok(())`，當程式失敗時回傳存有 `String` 的 `Err`。

測試中回傳 `Result<T, E>` 讓你可以在測試本體中使用問號運算子，這樣能方便地寫出任何運算回傳 `Err` 時該失敗的測試。

不過你就不能將 `#[should_panic]` 詮釋用在使用 `Result<T, E>` 的測試。當程式該失敗時，你必須直接回傳 `Err` 數值。

現在你知道了各種寫測試的方法，讓我們看看執行程式時發生了什麼事，並探索我們可以對 `cargo test` 使用的選項。

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#使用--運算子或-format-巨集串接字串
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#控制程式如何執行
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#將技術文件註解作為測試
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch11-01-writing-tests.md)
> - updated: 2020-09-15
