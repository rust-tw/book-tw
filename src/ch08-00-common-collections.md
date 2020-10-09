# 常見集合

Rust 的標準函式庫提供一些非常實用的資料結構稱之爲*集合（collections）*。多數其他資料型別只會呈現一個特定數值，但是集合可以包含數個數值。不像內建的陣列與元組型別，這些集合指向的資料位於堆積上，代表資料的數量不必在編譯期就知道，而且可以隨著程式執行增長或縮減。每種集合都有不同的能力以及消耗，依照你的情形選擇適當的集合，是一項你會隨著開發時間漸漸掌握的技能。在本章節我們會介紹三種在 Rust 程式中十分常用的集合：

* *向量（Vector）* 允許你接二連三地儲存數量不定的數值。
* *字串（String）* 是字元的集合。我們在之前就提過 `String` 型別，本章會正式深入介紹。這是從一種通用資料結構 *map* 衍生出來的特定實作。

想瞭解更多標準函式庫提供的集合種類的話，歡迎查閱[技術文件][collections]。

[collections]: https://doc.rust-lang.org/std/collections/index.html

我們將討論如何建立與更新向量、字串與雜湊映射，以及它們的所長。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch08-00-common-collections.md)
> - updated: 2020-09-11
