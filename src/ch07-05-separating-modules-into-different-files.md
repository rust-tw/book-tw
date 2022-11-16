## 將模組拆成不同檔案

本章節目前所有的範例將數個模組定義在同一個檔案中。當模組增長時，你可能會想要將它們的定義拆開到別的檔案中，好讓程式碼容易瀏覽。

舉例來說，讓我們從範例 7-17 餐廳的多重模組開始。我們會將模組差成數個檔案，而不只是將所有模組都放在 crate 源頭檔案。在此例中，源頭檔案為 *src/lib.rs* 不過這步驟在執行檔 crate 的 *src/main.rs* 一樣可行。

首先，我們將 `front_of_house` 模組移到獨立的檔案中。刪掉 `front_of_house` 模組大括號內的程式碼，只留下宣告 `mod front_of_house;`，讓 *src/lib.rs* 包含的程式碼如範例 7-21 所示。請注意在我們加上範例 7-22 的 *src/front_of_house.rs* 檔案前這會仍無法編譯。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">範例 7-21：宣告 `front_of_house` 模組，其本體位於 *src/front_of_house.rs*</span>

接著，將原本大括號內的程式碼寫到新的檔案 *src/front_of_house.rs* 中，如範例 7-22 所示。編譯器知道要查看這個檔案，因為 crate 源頭有宣告這個模組的名稱 `front_of_house`。

<span class="filename">檔案名稱：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">範例 7-22：`front_of_house` 模組的定義位於 *src/front_of_house.rs*</span>

你只需要在模組樹中使用 `mod` 宣告一次來讀取檔案就好。一旦編譯器知道該檔案屬於專案的一部分（且知道其位在模組樹中的何處，因為你有宣告 `mod` 陳述式），專案中的其他檔案就能用宣告的路徑讀取檔案的程式碼，如同[「引用模組項目的路徑」][paths]<!-- ignore -->段落提到的一樣。換句話說，`mod` 和你在其他程式語言可能會看到的「include」動作並**不一樣**。

要開始移動 `hosting` 的話，我們先改變 *src/front_of_house.rs*，讓它只包含 `hosting` 模組的宣告：

<span class="filename">檔案名稱：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

然後我們建立一個目錄 *src/front_of_house* 以及一個檔案 *src/front_of_house/hosting.rs* 來包含 `hosting` 模組的定義：

<span class="filename">檔案名稱：src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

如果我們將 *hosting.rs* 放在 *src* 目錄下，編譯器會將 *hosting.rs* 的程式碼視為是宣告在 crate 源頭底下的 `hosting` 模組。編譯器決定哪些檔案屬於哪些模組的規則讓目錄與檔案架構能更貼近模組樹的架構。

> ### 其他種的檔案路徑
>
> 目前我們涵蓋了 Rust 編譯器使用的最佳檔案路徑形式，但 Rust 仍然支援舊版的檔案路徑。當 crate 源頭宣告了一個模組 `front_of_house` 時，編譯器會在以下幾處尋找模組的程式碼：
>
> * *src/front_of_house.rs*（我們介紹的）
> * *src/front_of_house/mod.rs*（舊版風格，仍然支援的路徑形式）
>
> 當有個 `front_of_house` 的子模組 `hosting` 宣告時，編譯器會在以下幾處尋找模組的程式碼：
>
> * *src/front_of_house/hosting.rs*（我們介紹的）
> * *src/front_of_house/hosting/mod.rs*（舊版風格，仍然支援的路徑形式）
>
> 如果你對同個模組同時使用兩種風格的話，你會收到編譯器錯誤。在同個專案對不同模組使用不同風格則是允許的，但這有可能會讓瀏覽專案的人感到困惑。
>
> 使用 *mod.rs* 檔案名稱的風格最主要的缺點是你的專案可能最後會有很多檔案都叫做 *mod.rs*，當你在編輯器同時開啟這些檔案時可能會被混淆。

我們將模組的程式碼班到了不同的檔案，而模組樹仍維持完好如初。就算函式定義被移動不同檔案，`eat_at_restaurant` 內的函式呼叫不用任何修改仍能維持運作。這樣的方式讓你可以隨著模組成長時，移動到新的檔案中。

另外 *src/lib.rs* 內的 `pub use crate::front_of_house::hosting` 陳述式沒有改變，在檔案作為 crate 的一部分來編譯時，使用 `use` 的方式也沒有改變。`mod` 關鍵字能宣告模組，然後 Rust 會去同名的檔案尋找該模組的程式碼。

## 總結

Rust 讓你能夠將套件拆成數個 crate，然後 crate 能再分成數個模組，好讓你可以從一個模組內指定其他模組的項目。而你可以使用絕對或相對路徑來達成。這些路徑可以用 `use` 陳述式來引入作用域，讓你可以在該作用域用更短的路徑來多次呼叫該項目。模組程式碼預設為私有的，但你可以使用 `pub` 關鍵字公開它的定義內容。

在下個章節，我們將探討在標準函式庫中的一些資料結構集合，讓你可以利用它們寫出整潔有組織的程式碼。

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html