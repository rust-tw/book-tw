# 枚舉與模式配對

在本章節中，我們將討論 *枚舉（enumerations）*，有時也被簡寫爲 *enums*。枚舉讓你定義一個能夠列舉其可能*變體（variants）*的型別。首先，我們會定義並使用枚舉來展示枚舉如何將其數據組織起來。再來，我們會來探討一個特定的實用枚舉：`Option`，其代表該值爲某些東西不然就是什麼都沒有。然後我們會看看 `match` 表達式的模式配對是怎麼運作的，讓它能夠針對枚舉中不同數值執行不同的程式碼。最後，我們會介紹 `if let` 如何組織你的程式碼，這是另一個讓你處理枚舉的方法，而且簡潔又方便。

枚舉是許多語言中都有提供的功能，不過它們能做的事在不同語言而異。Rust 的枚舉最接近於函式語言的*代數型別（algebraic data types）*，像是 F#、OCaml 與 Haskell。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [5c71aac](https://github.com/rust-lang/book/blob/5c71aac64380f74f34cd9a158cc2b1d9122b5ceb/src/ch06-00-enums.md)
> - updated: 2020-09-11
