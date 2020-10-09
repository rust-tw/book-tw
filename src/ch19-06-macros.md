## 巨集

在本書中，我們到處使用像 `println!` 這類的巨集（macro），但尚未完全探索巨集究竟是何物，以及該如何駕馭。**巨集**指的是一整家族的 Rust 功能集合：使用 `macro_rules!` 的**宣告式（declarative）巨集**，以及另外三種**程序式（procedural）巨集**：

* 客製化 `#[derive]` 巨集，可以將指定的程式碼加在使用 `derive` 屬性的結構體和枚舉
* 類屬性巨集，定義客製化，可以用在任何項目（item）的屬性
* 類函式巨集，看起來在呼叫函式但實際上將標記（token）當作引數來處理

我們將會按照順序聊聊每種巨集，但首先，來看看為什麼我們已經有了函式，仍需要巨集呢？

### 巨集與函式的差異

基本上，巨集是一種透過寫程式碼來產生其他程式碼的手段，又稱作**超程式設計（metaprogramming）**。像是在附錄 C，我們探討的 `derive` 屬性，這個屬性會替你產生多種特徵的實作。還有在整本書中到處使用 `println!` 和 `vec!` 兩巨集。以上這些巨集都會**展開**來，產生比你自己手寫的還要多的程式碼。

超程式設計對減少撰寫和維護的程式碼量非常有幫助，這和函式扮演的角色相同，然而，巨集具有函式沒有的特殊本事。

一個函式簽名必須宣告該函式需要的參數型別與數量。反觀巨集可以接收變動數量的參數：我們可以用一個參數呼叫 `println!("hello")` ，也可以是兩個參數的 `println!("hello {}", name)`。另外，巨集會在編譯器開始翻譯程式碼的意義之前展開。例如可以使用巨集實作一個特徵。這種事函式便無法做到，因為函式會在執行期呼叫，而特徵需要在編譯期就實作。

選擇實作巨集而不用函式也有缺點，巨集的定義比函式更加複雜，因為你是在寫寫 Rust 程式碼的 Rust 程式碼。就是因為這種間接迂迴的關係，一般情況下，相較於函式來說巨集的定義都更加難以閱讀、理解與維護。

另一個巨集和函式之間的重要的的差異，在一個檔案中想呼叫巨集，必須在作用域（scope）內定義或是將巨集帶到這個作用域，而反過來函式可以在任何地方定義與呼叫。

### 使用 `macro_rules!` 宣告式巨集做普通的超程式設計

Rust 中最廣泛使用的巨集形式非**宣告式巨集**莫屬。這種巨集有時也稱為「巨集為例（macros by example）」、「`macro_rules!`」，或是直白的「巨集」。宣告式巨集的核心就是賦予你寫些類似 Rust `match` 表達式的東西。在第六章我們聊了 `match` 表達式是一種流程控制結構，會拿一個表達式，將其結果值與其他模式作比較，並執行匹配模式對應的程式碼。巨集同樣會拿一個值，與模式相比較，而這個模式又與特定程式碼相關聯：這種情況會是，傳入巨集的值就是一字一字刻出來 Rust 原始碼，而所謂模式則是比較原始碼的結構，當原始碼與模式相匹配，就會帶入與模式關聯的這段特定程式碼，取代原先傳入巨集的原始碼。這些都發生在編譯的期間。

你可以透過 `macro_rules!` 定義一個巨集。讓我們藉著閱讀 `vec!` 的定義來探索如何使用 `macro_rules!`。第八章我們介紹了如何使用 `vec!` 巨集來建立含有特定值的向量。例如，下面的巨集會建立帶著三個整數的新向量：

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

我們也可利用 `vec!` 巨集產生兩個整數的向量或是五個字串的 slice。因為不能預先得知這些值的數量，所以我們無法透過函式做到這件事。

範例 19-28 展示了稍微簡化過的 `vec!` 巨集定義。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-28/src/lib.rs}}
```

<span class="caption">範例 19-28：`vec!` 巨集定義簡化版</span>

> 注意：在標準函式庫中真實的 `vec!` 巨集定義有預先配置正確記憶體用量的程式碼，因為這段程式碼是一種最佳化手段，為了簡化範例，並無將之包含其中。

這個 `#[macro_export]` 標註（annotation）用來指明只要這個 crate 有在程式碼可見作用域中，就可以使用該巨集。若沒有這個標註，巨集就不能帶入該作用域內。

我們的巨集定義從 `macro_rules!` 和我們欲定義的巨集名稱**去除**驚嘆號開始。這個名稱，在我們例子裡是 `vec`，的後面接著花括號表示巨集定義的本體。

這個 `vec!` 本體的結構和 `match` 表達式的結構相似。這裡我們有一個 match 分支，帶著模式 `( $( $x:expr ),* )`，並接著 `=>` 後面與該模式相關聯的程式碼區塊。這個分支是此巨集唯一一個模式，所以只有一個合法匹配方式；任何其他模式都會產生錯誤。更複雜的巨集會有多於一個分支。

合法的巨集定義模式語法和在第十八章的模式語法並不相同，巨集的模式並不跟值比較，而是與 Rust 程式碼的結構相互匹配。在範例 19-28 我們會走過一次這些模式的意義，至於完整的巨集模式語法，請閱讀[參考手冊]。

[參考手冊]: ../reference/macros-by-example.html

首先，一對圓括號包圍整個模式。在括號後面的錢字號（`$`）捕獲了在括號內匹配該模式的值，用來取代該段程式碼。在 `$()` 內的 `$x:expr` 會匹配任意 Rust 表達式，並給這個表達式一個 `$x` 名。

在 `$()` 後的逗號代表字面上的逗號分隔，可以選擇性地在匹配 `$()` 內的程式碼後出現。而 `*` 這指明，這個模式可以匹配零至多個在 `*` 之前的東西。

當我們的以 `vec![1, 2, 3]` 呼叫這個巨集，`$x` 模式會匹配到三次，分別為 `1`、`2` 和 `3` 三個表達式。

現在來看看這個模式分支的本體程式碼：在 `$()*` 內的 `temp_vec.push()` 會根據 `$()` 模式匹配了幾次而產生幾次。這個 `$x` 會被每個匹配的表達式取代。當我們使用 `vec![1, 2, 3]` 呼叫巨集時，這個取代巨集呼叫而產生出來的程式碼會是：

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

我們定義了一個巨集，接收任意數量任意型別的引數，並產生建立一個包含指定元素的向量的程式碼。

有鑑於 `macro_rules!` 仍有些詭異的邊界情況（edge case），未來 Rust 會有第二類宣告式巨集，會具有相似的工作流程，但會修復這些邊界情況。在該更新到來過後，`macro_rules!` 會即期棄用（deprecate）。考量到這點，加上以事實來說大多數 Rust 程式設計師**使用**巨集多過**撰寫**巨集，所以 `macro_rules!` 相關討論就此打住，想理解更多有關撰寫巨集之事，可查閱線上文件或其他資源，例如[「The Little Book of Rust Macros」][tlborm]。

[tlborm]: https://danielkeep.github.io/tlborm/book/index.html

### 使用程序式巨集從屬性產生程式碼

第二種巨集形式是**程序式巨集**，其行為更像是函式（也是一種程序）。程序式巨集接受一些程式碼作為輸入，操作這些程式碼，然後輸出一些程式碼。和宣告式巨集去匹配模式和取代程式碼的方式不同。

三種程序式巨集（客製化 derive，類屬性、類函式）都有著相近的工作方式。

當建立一個程序式巨集時，該巨集必須放置在自己特殊的一種 crate 中。會這種是因為一些複雜的技術問題，我們希望在未來消弭這個情況。使用程序式巨集看起來就像範例 19-29，其中 `some_attribute` 是一個用來代表特定巨集的佔位符。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

<span class="caption">範例 19-29：使用程序式巨集</span>

這個函式定義一個程序式巨集，接受輸入 `TokenStream`，並輸入 `TokenStream`。`TokenStream` 型別定義在 `proc_macro` crate 中，這個 crate 包含在 Rust 中，可以表示一連串的標記，這就是巨集的核心：巨集替來自輸入的 `TokenStream` 搽脂抹粉，而巨集產生的程式碼就是輸出的 `TokenStream`。上面例子中這個函式附加了一個屬性，指定我們要產生哪個程序式巨集。在同一個 crate 中我們可以使用多個不同的程序式巨集。

我們來看不同的程序式巨集吧。就從客製化 derive 巨集開始，逐步介紹它與其他種類巨集的細部差異。

### 如何撰寫客製化的 `derive` 巨集

我們建立一個 `hello_macro` crate，並定義 `HelloMacro` 特徵與它的 `hello_macro` 關聯函式。我們提供一個程序式巨集，讓使用者透過 `#[derive(HelloMacro)]` 標註它們的型別，來獲得預設的 `hello_macro` 函式的實作，而不需要使用者替每個型別手動實作 `HelloMacro` 特徵。這個預設的函式實作會印出 `你好，巨集，我叫作型別名稱！`，其中 `型別名稱` 是實作特徵那個型別的名字。換句話說，就是我們會寫出一個 crate，讓其他程式設計師用我們的 crate，以範例 19-30 的方式來寫程式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-30/src/main.rs}}
```

<span class="caption">範例 19-30：使用者使用我們的程序式巨集時，能夠寫出的程式碼</span>

當我們完成後，這段程式碼會印出 `你好，巨集！我叫作鬆餅！`。第一步，先建立一個新的函式庫 crate：

```console
$ cargo new hello_macro --lib
```

接下來，我們會定義 `HelloMacro` 特徵與它的關聯函式：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/hello_macro/src/lib.rs}}
```

我們有個特徵及其函式。至此，我們的 crate 使用者可以實作此特徵來達成他們想要的功能，例如：

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/pancakes/src/main.rs}}
```

然而，使用者必須自行替每個想使用 `hello_macro` 的型別分別撰寫實作區塊，我們想節約這些重複工作。

另外，我們尚未提供 `hello_macro` 函式的預設實作，這個預設實作將會印出實作該特徵的型別名稱，但 Rust 並沒有反射（reflection）這種功能，所以無法在執行期檢查型別，因此我們需要一個巨集在編譯期產生程式碼。

下一步是定義程序式巨集。在我們寫此章時，程序式巨集必須在自己的 crate 中定義，最終這個限制會解除。組織安排 crate 和巨集 crate 的慣例如下：有一個 crate `foo` 和一個客製化 derive 程序式巨集 crate `foo_derive`，讓我們在 `hello_macro` 專案中建立一個新的 crate `hello_macro_derive`：

```console
$ cargo new hello_macro_derive --lib
```

由於我們的兩個 crate 高度關聯，所以會在 `hello_macro` crate 的目錄中建立一個程序式巨集 crate。若我們改變 `hello_macro` 中定義的特徵，就必須同時改變 `hello_macro_derive` 中的程序式巨集。這兩個 crate 必須各自發佈，且若程式設計師想要使用這些 crate，則必須將兩者都加入為依賴（dependency），並將之帶入作用域。當然，我們也可以讓 `hello_macro` 將 `hello_macro_derive` 作為一個依賴並重新導出（re-export）該程序式巨集。然而，我們這樣組織專案的方式就是想提供當程式設計師不想要 `derive` 功能時，也可以直接使用 `hello_macro`。

我們必須宣告 `hello_macro_derive` 為一個程序式巨集 crate。我們同時需要等會兒就會遇到的 `syn` 和 `quote` 這些 crate 的功能，所以先將他們加至依賴。至此，`hello_macro_derive` 的 *Cargo.toml* 會加入以下的程式碼：

<span class="filename">檔案名稱：hello_macro_derive/Cargo.toml</span>

```toml
{{#include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/Cargo.toml:7:12}}
```

欲開始定義程序式巨集，請將範例 19-31 的程式碼放入你的 `hello_macro_derive` crate 的 *src.lib.rs* 檔案中。注意，在我們定義 `impl_hello_macro` 函式之前，這段程式碼都無法編譯。

<span class="filename">檔案名稱：hello_macro_derive/src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/src/lib.rs}}
```

<span class="caption">範例 19-31：若要產生 Rust 程式碼，絕大部分程序式巨集 crate 都必須包含這段程式碼</span>

留意到了嗎，我們將程式碼函式拆分，其中 `hello_macro_derive` 函式負責解析 `TokenStream`，而 `impl_hello_macro` 函式則用來轉換語法樹（syntax tree），這讓撰寫程序式巨集更為方便。外面這個函式的程式碼（在這例子是 `hello_macro_derive`）在每個你遇見或建立的程序式巨集裡看起來都幾乎一模一樣。而在裡面的函式（在這個例子是 `impl_hello_macro`）的本體則根據不同程序式巨集的目的而有所不同。

我們導入了三個新 crate：`proc_macro`，[`syn`] 和 [`quote`]。`proc_macro` 包含在 Rust 裡面，所以我們不需要將之加入 *Cargo.toml*。`proc_macro` crate 就是編譯器的 API，提供從我們的程式碼讀取和操作 Rust 程式碼。

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

`syn` crate 負責從字串解析 Rust 程式碼，轉成我們可以操作的資料結構。而 `qoute` crate 則將 `syn` 的資料結構轉回 Rust 程式碼。撰寫完整的Rust 程式碼解析器並不是容易的工作，而這些 crate 讓解析任何 Rust 程式碼更為簡便。

當使用者在一個型別上指定 `#[derive(HelloMacro)]`，`hello_macro_derive` 函式就會被呼叫，這是由於我們使用 `proc_macro_derive` 和指定的 `HelloMacro` 名稱來標註 `hello_macro_derive` 函式，而其中的 `HelloMacro` 是我們的特徵名稱。以上就是大多數程序式巨集遵守的慣例。

`hello_macro_derive` 函式會先將輸入 `input` 的 `TokenStream` 轉換成一個我們可以翻譯並執行操作的資料結構，這就是 `syn` 參與的部分，`syn` 的 `parse` 函式需要一個 `TokenStream` 並回傳一個 `DeriveInput` 結構體，代表解析過後的 Rust 程式碼。範例 19-32 展示了解析完 `struct Pancakes` 字串後所得的 `DeriveInput` 的部分：

```rust,ignore
DeriveInput {
    // --省略--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<span class="caption">範例 19-32：這是在範例 19-30 解析具有 macro 屬性的程式碼時所得的 `DeriveInput` 實例</span>

這些結構體的欄位展示了解析過後的 Rust 程式碼是一個結構體，帶著 `ident`（識別字 identifier）。這裡其他結構體的欄位都在描述 Rust 程式碼，更多資訊請參考 [`syn` 有關 `DeriveInput` 的文件][syn-docs]。

[syn-docs]: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html


我們很快就進入定義 `impl_hello_macro` 函式的環節，這個函式協助打造我們想要的新 Rust 程式碼。再動手做之前，注意我們的 derive 巨集輸出也是一個 `TokenStream`。回傳的 `TokenStream` 會添加到我們的 crate 使用者撰寫的程式碼中，因此，當他們編譯他們的 crate 時，會從我們提供的修編過的 `TokenStream` 中取得額外功能。

也許你注意到我們對 `hello_macro_derive` 呼叫 `unwrap` 讓 `sync::parse` 函式失敗時恐慌。由於我們需要符合 `proc_macro_derive` 程序式巨集的 API 定 義，回傳一個 `TokenStream` 而非 `Result`，所以我們的程序式巨集必須在錯誤時恐慌。這裡使用 `unwrap` 是為了簡化範例，在正式環境程式碼中，你應該透過 `panic!` 或 `expect` 提供更特定的錯誤訊息，告知什麼出錯了。

現在，被標註的 Rust 程式碼已經從一個 `TokenStream` 轉換成 `DeriveInput` 實例，現在來替被標註的型別產生實作 `HelloMacro` 特徵的程式碼，如範例 19-33。

<span class="filename">檔案名稱：hello_macro_derive/src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-33/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

<span class="caption">範例 19-33：利用解析過的 Rust 程式碼來實作 `HelloMacro` 特徵</span>

我們從 `ast.ident` 取得 `Ident` 結構體實例，這個實例中帶有被標註的型別之名稱（識別字）。當我們執行在範例 19-30 程式碼中的 `impl_hello_macro` 函式，會獲得一個 `ident`，帶有一個值為 `"Pancakes"` 的 `ident` 欄位，就如同範例 19-30 所示。因此，在範例 19-33 的 `name` 變數會包含一個 `Ident` 結構體實例，當我們印之，會出現字串 `"Pancakes"`，也就是該結構體在範例 19-30 所示的名字。

`quote!` 巨集提供我們定義想要回傳的 Rust 程式碼。編譯器期望接收到不同於 `quote!` 巨集執行後直接輸出的結果，所以我們需要將結果轉換為一個 `TokenStream`。我們透過呼叫 `into` 方法達成，這個方法會消耗中介碼（intermediate representation）並回傳一個型別為 `TokenStream` 之值。

`quote!` 巨集也提供非常炫的模板機制：我們可以輸入 `#name`，而 `quote!` 會以變數 `name` 值取而代之。我們甚至可以做一些類似普通巨集的重複工作。閱讀 [`quote` crate 的文件][quote-docs]以獲得完整的介紹。

[quote-docs]: https://docs.rs/quote

我們想要我們的程序式巨集對使用者標註的型別產生 `HelloMacro` 特徵的實作，這個標註的型別名稱可以從 `#name` 取得。這個特徵的實作有一個函式 `hello_macro`，函式本體包含我們想要的功能：印出 `你好，巨集，我叫作` 再加上被標註的型別的名稱。

`stringify!` 巨集是 Rust 內建的，會將一個 Rust 表達式，例如 `1 + 2`，在編譯期轉換成字串字面值（string literal），例如 `"1 + 2"`。這和 `format!` 或 `println!` 巨集會對表達式求值並將結果轉為 `String` 不同。因為輸入的 `#name` 可能是一個表達式，但要直接照字面印出來，所以我們選擇使用 `stringify!`。使用 `stringify!` 也可以節省在編譯器因為轉換 `#name` 成為字串字面量所需的空間配置。

至此，`cargo build` 應該可以成功在 `hello_macro` 和 `hello_macro_derive` 完成。我們在範例 19-30 來玩玩這些 crate 看看他們如何實際作用！先在你的**專案**目錄下，透過 `cargo new pancakes` 建立一個新的二進制專案。我們必須將 `hello_macro` 和 `hello_macro_derive` 加入 `pancakes` 的 *Cargo.toml* 作為依賴。若你已經發佈自己的 `hello_macro` 和 `hello_macro_derive` 的版本到 [crates.io](https://crates.io/)，他們就是普通的依賴；若無，你可以指定他們為 `path` 的依賴，如下：

```toml
{{#include ../listings/ch19-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

將這段程式碼放到範例 19-30 的 *src/main.rs* 並執行 `cargo run`，他應該會印出 `你好，巨集！我叫作鬆餅！`。這個由程序式巨集實作的 `HelloMacro` 特徵，不需要 `pancakes` 自行手動實作，而是透過 `#[derive(HelloMacro)]` 將特徵的實作加上去。

接著，一起來探索其他種類的程序式巨集和客製化 derive 巨集有何不同。

### 類屬性巨集

類屬性巨集和客製化 derive 巨集相似，但並非只能透過 `derive` 屬性產生程式碼，類屬性巨集讓你可以建立新的屬性。它們更靈活：`derive` 只能用在結構體和枚舉，而屬性可以用在其他項目之上，例如函式。這裡有個類屬性巨集例子，是在使用一個網頁應用程式框架時，透過你的 `route` 屬性來標註一個函式：

```rust,ignore
#[route(GET, "/")]
fn index() {
```

這個 `#[route]` 屬性在該框架以程序式巨集定義之，其巨集定義函式的簽名如下：

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

這裡，我們有兩個 `TokenStream` 型別的參數，第一個是屬性的內容，也就是 `Get, "/"` 這部分。第二部分則是該屬性附著的項目本體：在這個例子就是 `fn index() {}` 及其函式本體。

除此之外，類屬性巨集的工作方式和客製化 derive 巨集一樣：透過 `proc-macro` crate 建立一個 crate，並實作一個函式替你產生程式碼！

### 類函式巨集

類函式巨集可以定義和函式呼叫很類似的巨集。和 `marco_rules!` 一樣，類函式巨集比函式更有靈活，例如可以接收未知長度的引數。然而，`macro_rules!` 巨集只能使用像 match 一樣的語法，如同早前在[「使用 `macro_rules!` 宣告式巨集做普通的超程式設計」][宣告式巨集]一節所述。而類函式巨集則可以拿 `TokenStream` 參數及其定義來操作 Rust 程式碼，和另外兩個程序式巨集所做的一模一樣。

舉個例子，一個 `sql!` 類函式巨集可能會被這樣呼叫：

[宣告式巨集]: #使用-macro_rules-宣告式巨集做普通的超程式設計

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

這個巨集會解析他內部的 SQL 陳述句（statement），並檢查語法是否正確，這個過程比 `macro_rules!` 能做到的複雜太多。這個 `sql!` 巨集定義如下：

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

這個定義和客製化 derive 巨集簽名相似：我們接受在圓括號內的標記，並回傳想要產生的程式碼。

## 小結

太帥了！現在你的工具箱多了一些 Rust 特色功能，雖然不常用，但在特定情況下你會知道它們存在。我們介紹了許多複雜的主題，所以當你在錯誤訊息或是其他人的程式碼與它們相遇，你會有辦法辨認這些概念和語法。你可以將這章作為能引導找到解法的參考書。

接下來，我們會動手做另一個專案，實際運用本書所講的一切。

> - translators: [Weihang Lo <me@weihanglo.tw>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch19-06-macros.md)
> - updated: 2020-09-20
