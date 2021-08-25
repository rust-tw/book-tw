## 將模組拆成不同檔案

本章節目前所有的範例將數個模組定義在同一個檔案中。當模組增長時，你可能會想要將它們的定義拆開到別的檔案中，好讓程式碼容易瀏覽。

舉例來說，讓我修將範例 7-17 中的 `front_of_house` 模組移到它自己的檔案 *src/front_of_house.rs*，然後在 crate 源頭檔案加上這個模組，如範例 7-21 所示。在此例中，源頭檔案為 *src/lib.rs* 不過這步驟在二進制執行檔 crate 的 *src/main.rs* 一樣可行。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">範例 7-21：宣告 `front_of_house` 模組，其本體位於 *src/front_of_house.rs*</span>

然後 `front_of_house` 模組的本體會定義在 *src/front_of_house.rs*，如範例 7-22 所示。

<span class="filename">檔案名稱：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">範例 7-22：`front_of_house` 模組的定義位於 *src/front_of_house.rs*</span>

在 `mod front_of_house` 之後用分號而不是大括號會告訴 Rust 讀取其他與模組同名的檔案以取得模組內容。讓我們繼續將範例中的 `hosting` 模組也取出並移到它自己的檔案中，我們可以變更 *src/front_of_house.rs* 成只包含 `hosting` 模組的宣告

<span class="filename">檔案名稱：src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

然後我們建立一個目錄 *src/front_of_house* 以及一個檔案 *src/front_of_house/hosting.rs* 來包含 `hosting` 模組的定義：

<span class="filename">檔案名稱：src/front_of_house/hosting.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

雖然定義都被移到不同的檔案了，但模組樹維持不變，而且在 `eat_at_restaurant` 的函式呼叫方式也不用做任何更改。此技巧讓你可以將增長中的模組移到新的檔案。

另外 *src/lib.rs* 內的 `pub use crate::front_of_house::hosting` 陳述式沒有改變，在檔案作為 crate 的一部分來編譯時，使用 `use` 的方式也沒有改變。`mod` 關鍵字能宣告模組，然後 Rust 會去同名的檔案尋找該模組的程式碼。

## 總結

Rust 讓你能夠將套件拆成數個 crate，然後 crate 能在分成數個模組，好讓你可以從一個模組內指定其他模組的項目。而你可以使用絕對或相對路徑來達成。這些路徑可以用 `use` 陳述式來引入作用域，讓你可以在該作用域用更短的路徑來多次呼叫該項目。模組程式碼預設為私有的，但你可以使用 `pub` 關鍵字公開它的定義內容。

在下個章節，我們將探討在標準函式庫中的一些資料結構集合，讓你可以利用它們寫出整潔有組織的程式碼。
