## Hello, Cargo!

Cargo 是 Rust 的建構系統與套件管理工具。大部分的 Rustaceans 都會用此工具來管理他們的專案，因爲 Cargo 能幫你處理很多任務，像是建構你的程式碼、下載你程式碼所需要的依賴函式庫並建構它們。我們常簡稱程式碼所需要用到的函式庫爲*依賴（dependencies）*。

簡單的 Rust 程式像是我們目前所寫的不會有任何依賴。所以當我們用 Cargo 建構「Hello, world!」專案時，Cargo 只會用到建構程式碼的那部分。隨著你寫的 Rust 程式越來越複雜，你將會加入一些依賴函式庫來幫助你。而如果你使用 Cargo 的話，加入這些依賴就會簡單很多。

既然大多數的 Rust 專案都是用 Cargo，所以接下來本書也將預設你也使用 Cargo。Cargo 在你使用[「安裝教學」][installation]<!-- ignore --> 的官方安裝連結來安裝 Rust 時就已經連同安裝好了。如果你是用其他方式下載 Rust 的話，想要檢查 Cargo 有沒有下載好可以透過你的終端機輸入：

```console
$ cargo --version
```

如果你有看到版本號，那就代表你有安裝了！如果你看到錯誤訊息，像是 `command not found`，請查看你的安裝辦法的技術文件，尋找如何分別下載 Cargo。

### 使用 Cargo 建立專案

讓我們來用 Cargo 建立一個專案，並來比較他和我們原本的「Hello, world!」專案有什麼差別。請回到你的 *projects*  目錄（或者任何你決定存放程式碼的地方），然後在任何作業系統上輸入：

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

第一道命令會建立一個新的目錄叫做 *hello_cargo*。我們將我們的專案命名爲 *hello_cargo*，然後 Cargo 就會產生相同名稱的目錄並產生所需的檔案。

進入 *hello_cargo* 然後顯示檔案的話，你會看到 Cargo 產生了兩個檔案和一個目錄： *Cargo.toml* 檔案以及一個 *src* 目錄，其內包含一個 *main.rs* 檔案。

它還會初始化成一個新的 Git repository 並附上 *.gitignore* 檔案。如果已經在 Git repository 內的話，執行 `cargo new` 則不會產生 Git 的檔案。你可以用 `cargo new --vcs=git` 覆寫這項行爲。

> 注意：Git 是一個常見的版本控制系統。你可以加上 `--vcs` 來變更 `cargo new` 去使用不同的版本控制系統，或是不用任何版本控制系統。請執行 `cargo new --help` 來查看更多可使用的選項。

請用任何你喜歡的編輯器開啓 *Cargo.toml*，它應該會看起來和範例 1-2 差不多。

<span class="filename">檔案名稱：Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

<span class="caption">範例 1-2：用 `cargo new` 產生的 *Cargo.toml*</span>

此檔案用的是 [*TOML*](https://toml.io)<!-- ignore --> （*Tom’s Obvious, Minimal Language*）格式，這是 Cargo 配置文件的格式。

第一行的 `[package]` 是一個段落（section）標題，說明以下的陳述語句會配置這個套件。隨著我們加入更多資訊到此文件，我們也會加上更多段落。

接下來四行就是 Cargo 編譯你的程式所需的配置資訊：名稱、版本、誰寫的以及哪個 Rust `edition` 會用到。Cargo 會透過環境取得你的名字和電子郵件資訊，所以要是資訊不對的話，請現在編輯然後儲存檔案。我們會在附錄 E 介紹什麼是 `edition`。

最後一行 `[dependencies]` 是用來列出你的專案會用到哪些依賴的段落。在 Rust 中，程式碼套件會被稱爲 *crates*。我們在此專案還不需要任何其他 crate。但是我們會在第二章開始用到，屆時我們會再來介紹。

現在請開啓 *src/main.rs* 來看看：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo 預設會爲你產生一個「Hello, world!」程式，就像我們範例 1-1 寫得一樣！目前我們之前寫的專案與 Cargo 產生的程式碼不同的地方在於 Cargo 將程式碼放在 *src* 目錄底下，而且我們還有一個 *Cargo.toml* 配置文件在根目錄。

Cargo 預期你的原始文件都會放在 *src* 目錄底下。專案的根目錄是用來放 README 文件、授權條款、配置文件以及其他與你的程式碼不相關的文件。使用 Cargo 能夠幫助你組織你的專案，讓一切井然有序。

如果你的專案還沒開始使用 Cargo 的話，像是我們剛剛寫得「Hello, world!」專案，你只要將程式碼移入 *src* 然後產生正確的 *Cargo.toml* 文件，就可以將它轉換成能夠使用 Cargo 的專案。

### 建構並執行 Cargo 專案

現在讓我們看看用 Cargo 產生的「Hello, world!」程式在建構和執行時有什麼差別！請在你的 *hello_cargo* 目錄下輸入以下命令來建構專案：

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

此命令會產生一個執行檔 *target/debug/hello_cargo*（在 Windows 上則是 *target\debug\hello_cargo.exe*），而不是在你目前的目錄。你可以用以下命令運行執行檔：

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

如果一切順利，`Hello, world!` 就會顯示在終端機上。第一次執行 `cargo build` 的話，還會在根目錄產生另一個新檔案：*Cargo.lock*。此檔案用來追蹤依賴函式庫的確切版本。不過此專案沒有任何依賴，所以目前這個檔案看起來內容會有點少。你不會需要去手動更改此檔案，Cargo 會幫你管理這個檔案的內容。

我們剛用 `cargo build` 建構完專案並用 `./target/debug/hello_cargo` 執行它。不過我們其實也可以只用一道命令 `cargo run` 來編譯程式碼並接著運行產生的執行檔：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

請注意到這次輸出的結果我們沒有看到 Cargo 有在編譯 `hello_cargo` 的跡象，這是因爲 Cargo 可以知道檔案完全沒被更改過，所以它選擇直接執行二進制檔案。如果你有變更你的原始程式碼的話，Cargo 才會在執行前重新建構專案，你才會看到這樣的輸出結果：

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo 還提供一道命令 `cargo check`，此命令會快速檢查你的程式碼，確保它能編譯不過不會產生執行檔。：

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

爲啥你會不想要產生執行檔呢？這是因爲 `cargo check` 省略了產生執行檔的步驟，所以它執行的速度比 `cargo build` 還來的快。如果你在寫程式時需要持續檢查的話，使用 `cargo check` 可以加快整體過程！所以許多 Rustaceans 都會在寫程式的過程中時不時執行 `cargo check` 來確保它能編譯。最後當他們準備好要使用執行檔時，才會用 `cargo build`。

讓我們來回顧我們目前學到的 Cargo 內容：

* 我們可以用 `cargo build` 建構專案。
* 我們可以用 `cargo run` 一次建構並執行專案。
* 我們可以用 `cargo check` 建構專案來檢查錯誤，但不會產生執行檔。
* Cargo 會儲存建構結果在 *target/debug* 目錄底下，而不是放在與我們程式碼相同的目錄。

使用 Cargo 還有一項好處是在任何作業系統所使用的命令都是相同的，所以到這邊開始我們不再需要特別提供 Linux 和 macOS 相對於 Windows 不同的特殊命令。

### 建構發佈版本（Release）

當你的專案正式準備好要發佈的話，你可以使用 `cargo build --release` 來最佳化編譯結果。此命令會產生執行檔到 *target/release* 而不是 *target/debug*。最佳化可以讓你的 Rust 程式碼跑得更快，不過也會讓編譯的時間變得更久。這也是爲何 Cargo 提供兩種不同的設定檔（profile）：一個用來作爲開發使用，讓你可以快速並經常重新建構；另一個用來最終產生你要給使用者運行的程式用，它通常不會需要重新建構且能盡所能地跑得越快越好。如果你要做基準化分析（benchmarking）來檢測程式運行時間的話，請確認執行的是 `cargo build --release` 並使用 *target/release* 底下的執行檔做檢測。

### 將 Cargo 視爲常規

雖然在簡單的專案下，與 `rustc` 相比 Cargo 的確沒辦法突顯出什麼價值。但是當你的程式變得越來越複雜時，它將證明它的用途。在擁有一堆 crate 的龐大專案下，讓 Cargo 來協調你的專案會來的簡單許多。

儘管 `hello_cargo` 是個小專案，但它使用了你未來的 Rust 生涯中真實情況下會用到的工具。事實上，所有存在的專案，你幾乎都可以用以下命令完成：使用 Git 下載專案、更改至專案目錄然後建構完成。

```console
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

有關 Cargo 的更多資訊，請查看它的[技術文件]。

[技術文件]: https://doc.rust-lang.org/cargo/

## 總結

你已經完成你的 Rust 旅途的第一步了！在本章節你學到了：

* 使用 `rustup` 安裝最新穩定版 Rust
* 更新到最新 Rust 版本
* 開啓本地端安裝的技術文件
* 直接使用 `rustup` 編寫並執行一支「Hello, world!」程式
* 使用 Cargo 建立並執行一個新專案

接下來是時候來建立一個更實際的程式來熟悉 Rust 程式碼的讀寫了。所以在第二章我們將寫出一支猜謎遊戲的程式。如果你想直接學習 Rust 的常見程式設計概念的話，你可直接閱讀第三章，之後再回來看第二章。

[installation]: ch01-01-installation.html#installation

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [a2e4bbf](https://github.com/rust-lang/book/blob/a2e4bbfdfbdb195bf134e5a53174d7d4ab027b1f/src/ch01-03-hello-cargo.md)
> - updated: 2020-09-05