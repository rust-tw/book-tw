# 最終專案：建立多執行緒網頁伺服器

這真是趟漫長的旅途，但我們已經抵達本書的最終章了。在本章中，我們會在建構另一個專案來解釋最後幾章提到的概念，並複習一些之前更早的章節。

在我們的最終專案，我們會建立一個會回復「hello」的網頁伺服器，如圖示 20-1 的網頁瀏覽器所示。

![hello from rust](img/trpl20-01.png)

<span class="caption">圖示 20-1：我們的最終專案</span>

以下是我們建構網頁伺服器的計劃：

1. 學習一些 TCP 與 HTTP。
2. 在插座（socket）上監聽 TCP 連線。
3. 解析一些的 HTTP 請求。
4. 建立合適的回應。
5. 透過執行緒池（thread pool）改善伺服器的吞吐量。

不過在我們開始之前，我們需要提醒一件事，我們使用的方法不會是在 Rust 中建立網頁伺服器的最佳方案。[crates.io](https://crates.io/) 上有不少已經能用在生產環境的 crate，它們都有提供比我們所建立的還更完善的網頁伺服器與執行緒池。

然而我們在本章節的目的是要幫助你學習，而不是走捷徑。因為 Rust 是個系統程式設計語言，我們可以選擇我們想運用的抽象層級，而且可以比其他語言更可能且實際地抵達最底層。我們會親自寫出基本的 HTTP 伺服器與執行緒池，來幫助你瞭解往後你可能會用到的 crate 背後的基本概念與技術。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch20-00-final-project-a-web-server.md)
> - updated: 2020-10-01
