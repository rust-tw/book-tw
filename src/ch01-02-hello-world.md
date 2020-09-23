## Hello, World!

現在你已經安裝好 Rust，讓我們開始來寫你的第一支 Rust 程式吧。當我們學習一門新的語言時，有一個習慣是寫一支印出「Hello, world!」到螢幕上的小程式，此章節將教你做一樣的事！

> 注意：本書將預設你已經知道命令列最基本的使用方法。Rust 對於你的編輯器、工具以及程式碼位於何處沒有特殊的要求，所以如果你更傾向於使用整合開發環境（IDE）的話，請儘管使用你最愛的 IDE。許多 IDE 都已經針對 Rust 提供某種程度的支援，請查看你所使用的 IDE 技術文件以瞭解詳情。最近，Rust 團隊正在積極專注在提升 IDE 的支援，而且進展十分迅速且出色！

### 建立專案目錄

你將先建立一個目錄來儲存你的 Rust 程式碼。程式碼位於何處並不重要，但爲了能好好練習書中的範例和專案，我們建議你可以在你的 home 目錄建立一個 *projects* 目錄然後將你所有的專案保存在此。

請開啟終端機然後輸入以下命令來建立 *projects* 目錄和另一個在 *projects* 目錄底下的真正要寫「Hello, world!」專案的目錄。

對於 Linux、macOS 和 Windows 的 PowerShell，請輸入：

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

對於 Windows CMD，請輸入：

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### 編寫並執行 Rust 程式

接著，請產生一個原始文件並叫做 *main.rs*。Rust 的文件檔案都會以 *.rs* 副檔名稱作爲結尾。如果你用到不止一個單字的話，請用底線區隔開來。比方說，請使用 *hello_world.rs* 而不是 *helloworld.rs*。

現在請開啟 *main.rs* 文件然而後輸入範例 1-1 中的程式碼。

<span class="filename">檔案名稱：main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<span class="caption">範例 1-1：印出「Hello, world!」的程式</span>

儲存檔案然後回到你的終端機螢幕。在 Linux 或 macOS 上，請輸入以下命令來編譯並執行檔案：

```console
$ rustc main.rs
$ ./main
Hello, world!
```

在 Windows 上則輸入 `.\main.exe` 而非 `./main`：

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

不管你的作業系統維和，終端機上應該都會出現 `Hello, world!`。如果你沒有看到，可以回到安裝章節中的[「疑難排除」][troubleshooting]<!-- ignore -->尋求協助。

如果 `Hello, world!` 有印出來，那麼恭喜你！你正式寫了一支 Rust 程式，所以你也正式成爲 Rust 開發者——歡迎加入！

### 分析這支 Rust 程式

讓我們來仔細瞧瞧你的「Hello, world!」程式實際上發生了什麼事。這是第一塊拼圖：

```rust
fn main() {

}
```

這幾行在 Rust 中定義了一個函式。`main` 是一個特別的函式：它是每個可執行的 Rust 程式永遠第一個執行的程式碼。第一行宣告了一個函式 `main`，它沒有參數也不回傳任何東西。如果有參數的話，它們會被加進括號 `()` 內。

再來，請注意到函式本體被囊括在大括號 `{}` 內，Rust 要求所有函式都用大括號包起來。一般來說，良好的程式碼風格會要求將前大括號置於宣告函式的同一行，並用一個空格區隔開來。

在本書撰寫的期間，有一支自動格式化的工具叫做 `rustfmt` 正在開發中。如果你想要在不同 Rust 專案之間統一標準風格的話，`rustfmt` 可以格式化你的程式成特定的風格。Rust 團隊計劃最終將此工具納入標準 Rust 發行版中，就像 `rustc` 一樣。所以依照你閱讀此書的時間點，它很可能已經安裝到你的電腦上了！請查看線上技術文件以瞭解詳情。

在 `main` 函式內有以下程式碼：

```rust
    println!("Hello, world!");
```

此行負責了整支程式要做的事：它將文字顯示在螢幕上。這邊有四個細節要注意。

首先，Rust 的排版風格是 4 個空格而非一個 tab。

第二，`println!` 會呼叫一支 Rust 巨集（macro）。如果是呼叫函式的話，那則會是 `println`（去掉 `!`）。我們會在第 19 章討論更多巨集的細節。現在你只需要知道使用 `!` 代表呼叫一支巨集而非一個正常的函式。

第三，`"Hello, world!"` 是一個字串，我們將此字串作爲引數傳遞給 `println!`，然後該字串就會被顯示到螢幕上。

第四，我們用分號（`;`）作爲該行結尾，代表此表達式的結束和下一個表達式的開始。多數的 Rust 程式碼都以分號做結尾。

### 編譯和執行是不同的步驟

你剛剛執行了一個薪建立的程式，讓我們來檢查過程中的每一個步驟吧。

在你執行一支 Rust 程式前，你必須用 Rust 編譯器來編譯它，也就是輸入 `rustc` 命令然後加上你的原始文件，像這樣子：

```console
$ rustc main.rs
```

如果你已經有 C 或 C++ 的背景，你應該就會發現這和 `gcc` 或 `clang` 非常相似。編譯成功後，Rust 編譯器會輸出一個二進制執行檔（binary executable）。

在 Linux、macOS 和 Windows 上的 PowerShell，你可以在你的 shell 輸入 `ls` 來查看你的執行檔。在 Linux 和 macOS，你會看到兩個檔案。而在 Windows 上的 PowerShell，你會和使用 CMD 一樣看到三個檔案。

```text
$ ls
main  main.rs
```

在 Windows 上的 CMD，你需要輸入：

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

這顯示了副檔名爲 *.rs*  的原始程式碼檔案、執行檔（在 Winddows 上爲 *main.exe*；其他則爲 *main*），然後在 Windows 上會再出現一個副檔名爲 *.pdb* 的除錯資訊文件。在這裡，你就可以像這樣執行 *main* 或 *main.exe* 檔案：

```console
$ ./main # or .\main.exe on Windows
```

如果 *main.rs* 正是你的 “Hello, world!” 程式，這命令就會顯示 `Hello, world!` 到你的終端機。

如果你比較熟悉動態語言，像是 Ruby、Python 或 JavaScript，你可能會比較不習慣將編譯與執行程式分爲兩個不同的步驟。Rust 是一門 *預先編譯（ahead-of-time compiled）* 的語言，代表你可以編譯完成後將執行檔送到其他地方，然後他們就算沒有安裝 Rust 一樣可以執行起來。但如果你給某個人 *.rb*、*.py* 或 *.js* 檔案，他們就需要 Ruby、Python 或 Javascript 分別都有安裝好。當然你在這些語言只需要一行命令就可以執行，在語言設計中這一切都只是取捨。

在簡單的程式使用 `rustc` 來編譯不會有什麼問題，但當你的專案成長時，你將會需要管理所有選擇並讓程式碼易於分享。接下來我們將介紹 Cargo 這項工具給你，它將協助你寫出真正的 Rust 程式。

[troubleshooting]: ch01-01-installation.html#troubleshooting

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [fce7281](https://github.com/rust-lang/book/blob/fce7281061fd4b18a87075f43c17fc3168230a21/src/ch01-02-hello-world.md)
> - updated: 2020-09-05