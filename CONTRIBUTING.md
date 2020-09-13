# Contributing

我們歡迎任何協助，非常感謝你願意一起支援這個社群！

## 授權條款

此專案的授權條款和其他 Rust 專案一樣都是 MIT/Apache2。你可以在 `LICENSE-*` 檔案中閱讀條款全文

## 行為準則

Rust 專案有一個 [行為準則](http://rust-lang.org/policies/code-of-conduct)會囊括其下專案，這當然包含此專案。請遵守這些準則！

## 翻譯流程

目前本書仍在翻譯中，我們希望能夠儘快將基本內容全數翻譯。所以目前只要不是機翻或者讀起來錯的太誇張，我們會盡量讓 PR 都能 merge。

我們的 [open pull requests][pulls] 接受任何翻譯或修正文章的 PR。在想開始翻譯一篇 RFC 前請先開一個 issue，如果你有希望被翻譯的也一樣歡迎開。翻譯完文章後請在每一篇文底加上以下以下資訊：

```
translators: [Firstname Lastname <email@address.tld>]
commit: [The commit link this page based on](https://github.com/rust-lang/rfcs/...)
updated: YYYY-MMM-DD
```

目前我們傾向於能將術語翻譯成中文就翻譯成中文，在每個第一次翻譯特殊術語的部分請加上括號附上英文原文即可。我們都希望所有術語盡量都有中文翻譯，尤其那些在電腦科學常見的術語（如陣列、陳述式、迷途指標等等），我們應該要讓讀者能更熟悉繁中的詞彙。我們在書中新增了一篇附錄來紀錄[中英文術語對照表][terms]，基本上引用大都來自維基百科與國家學術名詞資訊網。這些術語目前都還能更改，要是你覺得有更好的翻譯，或是有些無法達到信雅達的話，歡迎開 issue 或 pull request 來討論並修正。

當大多數的篇幅都處理完後，我們會正式依依審核，並以 PR 形式確定該章節翻譯無誤，這階段一樣歡迎開啟 issue 或 pull request 來協助幫忙！

[pulls]: https://github.com/rust-lang.tw/rfcs/pulls
[terms]: https://rust-lang.tw/book-tw/appendix-08-terminology.html
