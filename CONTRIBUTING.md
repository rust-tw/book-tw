# Contributing

我們歡迎任何協助，非常感謝你願意一起支援這個社群！

## 授權條款

此專案的授權條款和其他 Rust 專案一樣都是 MIT/Apache2。你可以在 `LICENSE-*` 檔案中閱讀條款全文

## 行為準則

Rust 專案有一個 [行為準則](http://rust-lang.org/policies/code-of-conduct)會囊括其下專案，這當然包含此專案。請遵守這些準則！

## 文章審核

目前大部分的篇幅都已翻譯完畢，所以我們會逐步審核確定每篇文章都有符合要求。大致上，我們希望能檢查以下幾點：

- 沒有任何錯別字
- 沒有錯誤的術語，而且與其他篇章使用的術語均相同
- 語義都很通順，如果有會閱讀困難或念起來不太通順的地方，我們希望可以改善，至少以信亞達爲目標，不必全部都得符合原文字句。
- 範例都有中文化，Rust 的字串支援 UTF-8，所以說幾乎所有範例都可以用中文下字串或註解。
- 與官方最新 commit 內容符合，沒有缺少或遺漏任何內容
- 至少有一個專案維護者在對應的 tracking isse 表示審核通過

我們會依序開出 tracking issues（如果還沒有的話，歡迎幫忙開起來），當以上幾點都符合時，我們就會關閉 issue 並將該文章視爲審核通過。我們還有開一個 issue 來追蹤[所有文章審核的進度總覽](https://github.com/rust-tw/book-tw/issues/10)。

## 翻譯流程

目前所有章節都已翻譯完畢，但我們的 [open pull requests][pulls] 還是接受任何修正文章的 PR。在想開始翻譯一篇文章前請先開一個 issue，如果你有希望被翻譯的也一樣歡迎開。翻譯完文章後請在每一篇文底加上以下以下資訊：

```
translators: [Firstname Lastname <email@address.tld>]
commit: [The commit link this page based on](https://github.com/rust-lang/rfcs/...)
updated: YYYY-MMM-DD
```

我們傾向於能將術語翻譯成中文就翻譯成中文，在每個第一次翻譯特殊術語的部分請加上括號附上英文原文即可。我們都希望所有術語盡量都有中文翻譯，尤其那些在電腦科學常見的術語（如陣列、陳述式、迷途指標等等），我們應該要讓讀者能更熟悉繁中的詞彙。我們在書中新增了一篇附錄來紀錄[中英文術語對照表][terms]，基本上引用大都來自維基百科與國家學術名詞資訊網。這些術語目前都還能更改，要是你覺得有更好的翻譯，或是有些無法達到信雅達的話，歡迎開 issue 或 pull request 來討論並修正。

歡迎開啟 issue 或 pull request 來協助幫忙！

[pulls]: https://github.com/rust-lang.tw/rfcs/pulls
[terms]: https://rust-lang.tw/book-tw/appendix-08-terminology.html
