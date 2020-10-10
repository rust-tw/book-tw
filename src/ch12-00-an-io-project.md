# I/O 專案：建立一個命令列程式

本章節用來回顧你目前學過的許多計技能，並探索些標準函式庫中的更多功能。我們會來建立個命令列工作來處理檔案與命令列輸入／輸出，以此練習些已經在你的工具箱內的 Rust 概念。

Rust 的速度、安全、單一二進制輸出與跨平台支援使其成為建立命令列工具的絕佳語言。所以在我們的專案中，我們要寫出我們自己的經典命令列工具 `grep`（**g**lobally search a **r**egular **e**xpression and **p**rint）。在最簡單的使用場合中，`grep` 會搜尋指定檔案中的指定字串。為此 `grep` 會接收一個檔案名稱與一個字串作為其引數。然後它會讀取檔案、在該檔案中找到包含字串引數的行數，並印出這些行數。

在過程中，我們會展示如何讓我們的命令列工具和其他許多命令列工具一樣使用終端機的功能。我們會讀取一個環境變數的數值來讓使用者可以配置此工具的行為。我們還會將錯誤訊息在控制台中的標準錯誤（`stderr`）顯示而非標準輸出（`stdout`）。所以舉例來說，使用者可以將成功的標準輸出重新導向至一個檔案，並仍能在螢幕上看到錯誤訊息。

其中一位 Rust 社群成員 Andrew Gallant 已經有建立個功能完善且十分迅速的 `grep` 版本，叫做 `ripgrep`。相比之下，我們的 `grep` 版本會相對簡單許多，但此章節能給你些背景知識，來幫你理解像是 `ripgrep` 等真實專案。

我們的 `grep` 專案會組合你所學過的各種概念：

* 組織程式碼（使用你在[第七章][ch7]<!--   ignore -->所學的模組）
* 使用向量與字串（[第八章][ch8]<!-- ignore -->的集合）
* 錯誤處理（[第九章][ch9]<!-- ignore -->）
* 合理的使用特徵與生命週期（[第十章][ch10]<!-- ignore -->）
* 測試（[第十一章][ch11]<!-- ignore -->）

我們還會簡單介紹閉包、疊代器與特徵物件，這些在[第十三章][ch13]<!-- ignore -->與[第十七章][ch17]<!-- ignore -->會做詳細介紹。

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch12-00-an-io-project.md)
> - updated: 2020-10-02
