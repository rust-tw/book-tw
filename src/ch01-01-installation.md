## 安裝教學

第一步是安裝 Rust，我們將會透過 `rustup` 安裝 Rust，這是個管理 Rust 版本及相關工具的命令列工具。你將會需要網路連線才能下載。

> 注意：如果你基於某些原因不想使用 `rustup` 的話，請前往[安裝 Rust 頁面][install]尋求其他選項。

以下步驟將會安裝最新的穩定版 Rust 編譯器。Rust 的穩定性能確保本書的所有範例在更新的 Rust 版本仍然能繼續編譯出來。輸出的結果可能會在不同版本間而有些微的差異，因爲 Rust 時常會改善錯誤與警告訊息。換句話說，任何你所安裝的最新穩定版 Rust 都應該能夠正常運行本書的內容。

> ### 命令列標記
>
> 在本章節到整本書爲止，我們講會顯示一些終端機會用到的命令。任一你會用到的命令都會始於 `$`。但你不需要去輸入 `$`，因爲這通常代表每一命令列的起始位置。而沒有出現 `$` 的行數，通常則代表前一行命列輸出的結果。除此之外，針對 PowerShell 的範例則將會使用 `>` 而不是 `$`。

### 在 Linux 或 macOS 上安裝 `rustup`

如果你使用的是 Linux 或 macOS，請開啟終端機然後輸入以下命令：

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

這道命令會下載一支腳本然後開始安裝 `rusup` 工具，接著安裝最新的穩定版 Rust。下載過程中可能會要求你輸入你的密碼。如果下載成功的話，將會出現以下內容：

```text
Rust is installed now. Great!
```

除此之外，你還會需要某種類型的連結器（linker）。這通常在你的系統上都已經安裝了，但如果你嘗試編譯 Rust 程式卻遇到連結器無法執行的錯誤時，這代表你的系統並未安裝，而你需要自行安裝一個。C 編譯器通常都會帶有一個正確的連結器，你可以檢查你的平台文件，查看如何下載 C 編譯器。此外，一些常見的 Rust 套件也會依賴 C 的程式，所以也需要 C 編譯器。因此現在最好還是有安裝一個比較好。

### 在 Windows 上安裝 `rustup`

在 Windows 上請前往[下載頁面][install]並依照指示安裝 Rust。在安裝的某個過程中，你將會看到一個訊息要求你還需要 C++ build tools for Visual Studio 2013 或更新的版本。取得這項工具最簡單的辦法是下載 [Build Tools for Visual Studio][visualstudio]。當你被問到要安裝哪些項目時，請確認有選擇「C++ build tools」，而且有包含 Windows 10 SDK 和英文語言包套件。

[install]: https://www.rust-lang.org/zh-TW/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

本書接下來使用的命令都相容於 *cmd.exe* 和 PowerShell。如果有特別不同的地方，我們會解釋該怎麼使用。

### 更新與解除安裝

當你透過 `rustup` 安裝完 Rust 後，要更新到最新版本的方法非常簡單。在你的 shell 中執行以下更新腳本即可：

```console
$ rustup update
```

要解除安裝 Rust 與 `rustup` 的話，則在 shell 輸入以下解除安裝腳本：

```console
$ rustup self uninstall
```

### 疑難排除

想簡單確認你是否有正確安裝 Rust 的話，請開啟 shell 然後輸入此命令：

```console
$ rustc --version
```

你應該會看到已發佈的最新穩定版本號、提交雜湊（hash）以及提交日期如以下格式所示：

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

如果你看到這則訊息代表你成功安裝 Rust 了！如果你沒有看到且你是在 Windows 上的話，請檢查 Rust 是否在你的 `%PATH%` 系統變數裡。如果都正確無誤，但還是無法執行 Rust 的話，你可以前往一些地方尋求協助。最簡單的辦法是前往[官方 Rust Discord][discord] 的 #beginners 頻道詢問。在那裡你可以與其他 Rustaceans（這是我們常用稱呼自己取的暱稱）交談並取得協助。另外也有其他不錯的資源像是[使用者討論區][users] 和 [Stack Overflow][stackoverflow]。正體中文社群的話可以前往 rust-lang.tw，底下有 Facebook 或 Telegram 的連結一樣可以尋求協助。

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust

### 本地端技術文件

安裝 Rust 的同時也會包含一份本地的技術文件副本，讓你可以離線閱讀。執行 `rustup doc` 就可以用你的瀏覽器開啟本地文件。

每當有任何型別或函式出現而你卻不清楚如何使用時，你就可以閱讀應用程式介面（API）技術文件來理解！

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [bad683b](https://github.com/rust-lang/book/blob/bad683bb7dcd06ef7f5f83bad3a25b1706b7b230/src/ch01-01-installation.md)
> - updated: 2020-09-05