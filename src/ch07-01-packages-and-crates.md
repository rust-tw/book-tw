## 套件與 Crates

首先我們要介紹的第一個模組系統部分為套件與 crates。

一個 *crate* 是 Rust 編譯器同個時間內視為程式碼的最小單位。就算你執行的是 `rustc` 而非 `cargo`，然後傳入單一源碼檔案（就像我們在第一章的「編寫並執行 Rust 程式」那樣），編譯器會將該檔案視為一個 crate。Crate 能包含模組，而模組可以在其他檔案中定義然後同時與 crate 一起編譯，我們會在接下來的段落看到。

一個 crate 可以有兩種形式：執行檔 crate 或函式庫 crate。**執行檔（Binary） crate**
*Binary crates* are programs you can compile to an executable that you can run,
such as a command-line program or a server. Each must have a function called
`main` that defines what happens when the executable runs. All the crates we’ve
created so far have been binary crates.

*Library crates* don’t have a `main` function, and they don’t compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects. For example, the `rand` crate we used in [Chapter
2][rand]<!-- ignore --> provides functionality that generates random numbers.
Most of the time when Rustaceans say “crate”, they mean library crate, and they
use “crate” interchangeably with the general programming concept of a “library".

The *crate root* is a source file that the Rust compiler starts from and makes
up the root module of your crate (we’ll explain modules in depth in the
[“Defining Modules to Control Scope and Privacy”][modules]<!-- ignore -->
section).

A *package* is a bundle of one or more crates that provides a set of
functionality. A package contains a *Cargo.toml* file that describes how to
build those crates. Cargo is actually a package that contains the binary crate
for the command-line tool you’ve been using to build your code. The Cargo
package also contains a library crate that the binary crate depends on. Other
projects can depend on the Cargo library crate to use the same logic the Cargo
command-line tool uses.

A package can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that’s a
library or binary crate.
**crate 的源頭**會是一個原始檔案，讓 Rust 的編譯器可以作為起始點並組織 crate 模組的地方（我們會在[「定義模組來控制作用域與隱私權」][modules]<!-- ignore -->的段落更加解釋模組）。**套件**（package）則是提供一系列功能的一或數個 crate。一個套件會包含一個 *Cargo.toml* 檔案來解釋如何建構那些 crate。

套件依據一些規則來組成。一個套件最多可以包含一個函式庫 crate。它可以包含多少執行檔 crate 都沒關係，但一定得至少提供一個 crate（無論是函式庫或執行檔）。

讓我們看看當我們建立一個套件時發生了什麼事。首先我們先輸入 `cargo new` 命令：

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

當我們輸入命令時，Cargo 會建立一個 *Cargo.toml* 檔案並以此作為套件依據。查看 *Cargo.toml* 的內容時，你會發現沒有提到 *src/main.rs*，這是因為 Cargo 遵循一個常規，也就是 *src/main.rs* 就是與套件同名的
執行檔 crate 的 crate 源頭。同樣地，Cargo 也會知道如果套件目錄包含 *src/lib.rs*的話，則該套件就會包含與套件同名的函式庫 crate。Cargo 會將 crate 源頭檔案傳遞給 `rustc` 來建構函式庫或執行檔。

我們在此的套件只有包含 *src/main.rs* 代表它只有一個同名的執行檔 crate 叫做 `my-project`。如果套件包含 *src/main.rs* 與 *src/lib.rs* 的話，它就有兩個 crate：一個執行檔與一個函式庫，兩者都與套件同名。一個套件可以有多個執行檔 crate，只要將檔案放在 *src/bin* 目錄底下就好，每個檔案會被視為獨立的執行檔 crate。

Crate 會將相關的程式碼組織在一個作用域內，好讓其能易於分享給其他專案。舉例來說，我們在[第二章][rand]<!-- ignore -->使用到的 `rand` crate 就提供了產生隨機數值的功能。我們可以將 `rand` crate 引入我們的專案，讓我們可以在我們的專案使用這項功能。所有 `rand` crate 提供的功能都可以透過 crate 的名稱 `rand` 來索取。

將 crate 的功能維持在各自的作用域內能清楚地表達特定功能是定義在我們自己的 crate 還是 `rand` crate 的，以防止可能的衝突。舉例來說，`rand` crate 提供了一個特徵叫做 `Rng`，我們也可以在我們自己的 crate 中定義一個 `struct` 叫做 `Rng`。由於 crate 的功能都位於它所屬的作用域的命名空間底下，當我們加入 `rand` 作為依賴時，編譯器不會搞不清楚是哪個 `Rng` 被使用。在我們的 crate 中，它指的是我們定義的 `struct Rng`。而要使用 `rand` crate 的 `Rng` 特徵的話，我們得這樣使用 `rand::Rng`。

接下來讓我們繼續討論模組系統吧！

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#產生隨機數字
