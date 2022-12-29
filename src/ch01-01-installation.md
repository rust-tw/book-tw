## 安裝教學

第一步是安裝 Rust，我們將會透過 `rustup` 安裝 Rust，這是個管理 Rust 版本及相關工具的命令列工具。你將會需要網路連線才能下載。

> 注意：如果你基於某些原因不想使用 `rustup` 的話，請前往 [Rust 其他安裝方法的頁面][otherinstall]尋求其他選項。

以下步驟將會安裝最新的穩定版 Rust 編譯器。Rust 的穩定性能確保本書的所有範例在更新的 Rust 版本仍然能繼續編譯出來。輸出的結果可能會在不同版本間而有些微的差異，因為 Rust 時常會改善錯誤與警告訊息。換句話說，任何你所安裝的最新穩定版 Rust 都應該能夠正常運行本書的內容。

> ### 命令列標記
>
> 在本章節到整本書為止，我們將會顯示一些終端機會用到的命令。任一你會用到的命令都會始於 `$`。但你不需要去輸入 `$`，因為這通常代表每一命令列的起始位置。而沒有出現 `$` 的行數，通常則代表前一行命列輸出的結果。除此之外，針對 PowerShell 的範例則將會使用 `>` 而不是 `$`。

### 在 Linux 或 macOS 上安裝 `rustup`

如果你使用的是 Linux 或 macOS，請開啟終端機然後輸入以下命令：

```console
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

這道命令會下載一支腳本然後開始安裝 `rustup` 工具，接著安裝最新的穩定版 Rust。下載過程中可能會要求你輸入你的密碼。如果下載成功的話，將會出現以下內容：

```text
Rust is installed now. Great!
```

你還會需要一個**連結器（linker）** 來讓 Rust 將編譯好的輸出資料整理到一個檔案內。通常你很可能已經有安裝了，但如果你遇到連結器相關的錯誤時，這代表你需要安裝一個 C 編譯器，因爲它通常都會帶有一個的連結器。有個 C 編譯器通常也很實用，因爲一些常見的 Rust 套件也會依賴於 C 而需要一個 C 編譯器。

在 macOS 上，你可以輸入以下命令來安裝 C 編譯器：

```console
$ xcode-select --install
```

Linux 使用者的話則需要依據他們的發行版文件來安裝 GCC 或 Clang。舉例來說，如果你使用 Ubuntu 的話，你可以安裝 `build-essential` 套件。


### 在 Windows 上安裝 `rustup`

在 Windows 上請前往[下載頁面][install-windows]並依照指示安裝 Rust。在安裝的某個過程中，你將會看到一個訊息要求說你還需要 C++ build tools for Visual Studio 2013 或更新的版本。

要取得 build tools 的話，你需要安裝 [Visual Studio 2022][visualstudio]。當你被問到要安裝哪些時，請記得包含：

* “Desktop Development with C++”
* The Windows 10 or 11 SDK
* The English language pack component（以及其他你想選擇的語言包）

本書接下來使用的命令都相容於 *cmd.exe* 和 PowerShell。如果有特別不同的地方，我們會解釋該怎麼使用。

### 疑難排除

想簡單確認你是否有正確安裝 Rust 的話，請開啟 shell 然後輸入此命令：

```console
$ rustc --version
```

你應該會看到已發佈的最新穩定版本號、提交雜湊（hash）以及提交日期如以下格式所示：

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

如果你看到這則訊息代表你成功安裝 Rust 了！如果你沒有看到的話，請如下檢查 Rust 是否在你的 `%PATH%` 系統變數裡。

在 Windows CMD 中請使用：

```console
> echo %PATH%
```

在 PowerShell 中請使用：

```powershell
> echo $env:Path
```

在 Linux 和 macOS 的話請使用：

```console
$ echo $PATH
```

如果以上步驟皆正確無誤，但還是無法執行 Rust 的話，你可以前往一些地方尋求協助。例如您可以前往[社群頁面][community]聯絡其他 Rustaceans（這是我們常用稱呼自己取的暱稱）交談並取得協助。

### 更新與解除安裝

當你透過 `rustup` 安裝完 Rust 後，要更新到最新版本的方法非常簡單。在你的 shell 中執行以下更新腳本即可：

```console
$ rustup update
```

要解除安裝 Rust 與 `rustup` 的話，則在 shell 輸入以下解除安裝腳本：

```console
$ rustup self uninstall
```

### 本地端技術文件

安裝 Rust 的同時也會包含一份本地的技術文件副本，讓你可以離線閱讀。執行 `rustup doc` 就可以用你的瀏覽器開啟本地文件。

每當有任何型別或函式出現而你卻不清楚如何使用時，你就可以閱讀應用程式介面（API）技術文件來理解！

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/downloads/
[community]: https://www.rust-lang.org/community