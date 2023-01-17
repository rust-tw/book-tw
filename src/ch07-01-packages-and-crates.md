## 套件與 Crates

首先我們要介紹的第一個模組系統部分為套件與 crates。

一個 *crate* 是 Rust 編譯器同個時間內視為程式碼的最小單位。就算你執行的是 `rustc` 而非 `cargo`，然後傳入單一源碼檔案（就像我們在第一章的「編寫並執行 Rust 程式」那樣），編譯器會將該檔案視為一個 crate。Crate 能包含模組，而模組可以在其他檔案中定義然後同時與 crate 一起編譯，我們會在接下來的段落看到。

一個 crate 可以有兩種形式：執行檔 crate 或函式庫 crate。**執行檔（Binary）crate** 是種你能編譯成執行檔並執行的程式，像是命令列程式或伺服器。這種 crate 需要有一個函式 `main` 來定義執行檔執行時該做什麼事。目前我們建立的所有 crate 都是執行檔 crate。

**函式庫（Library）crate** 則不會有 `main` 函式，而且它們也不會編譯成執行檔。這種 crate 定義的功能用來分享給多重專案使用。舉例來說，我們在[第二章][rand]<!-- ignore -->用到的 `rand` crate 就提供了產生隨機數值的功能。當大多數的 Rustacean 講到「crate」時，他們其實指的是函式庫 crate，所以他們講到「crate」時相當於就是在講其他程式語言概念中的「函式庫」。

**crate 的源頭**會是一個原始檔案，讓 Rust 的編譯器可以作為起始點並組織 crate 模組的地方（我們會在[「定義模組來控制作用域與隱私權」][modules]<!-- ignore -->的段落更加解釋模組）。

**套件**（package）則是提供一系列功能的一或數個 crate。一個套件會包含一個 *Cargo.toml* 檔案來解釋如何建構那些 crate。Cargo 本身其實就是個套件，包含了你已經用來建構程式碼的命令列工具。Cargo 套件還包含執行檔 crate 需要依賴的函式庫 crate。其他專案可以依賴 Cargo 函式庫來使用與 Cargo 命令列工具用到的相同邏輯功能。

一個套件能依照你的喜好擁有數個執行檔 crate，但最多只能有一個函式庫 crate。而一個套件至少要有一個 crate，無論是函式庫或執行檔 crate。

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

在我們執行 `cargo new` 之後，我們使用 `ls` 來查看 Cargo 建立了什麼。在專案的目錄中會有個 *Cargo.toml* 檔案，這是套件的設定檔。然後還會有個 *src* 目錄底下包含了 *main.rs*。透過你的文字編輯器打開 *Cargo.toml*，你會發現沒有提到 *src/main.rs*。Cargo 遵循的常規是 *src/main.rs* 就是與套件同名的執行檔 crate 的 crate 源頭。同樣地，Cargo 也會知道如果套件目錄包含 *src/lib.rs*的話，則該套件就會包含與套件同名的函式庫 crate。Cargo 會將 crate 源頭檔案傳遞給 `rustc` 來建構函式庫或執行檔。

我們在此的套件只有包含 *src/main.rs* 代表它只有一個同名的執行檔 crate 叫做 `my-project`。如果套件包含 *src/main.rs* 與 *src/lib.rs* 的話，它就有兩個 crate：一個執行檔與一個函式庫，兩者都與套件同名。一個套件可以有多個執行檔 crate，只要將檔案放在 *src/bin* 目錄底下就好，每個檔案會被視為獨立的執行檔 crate。

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#產生隨機數字
