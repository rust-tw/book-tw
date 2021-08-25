## 改善我們的 I/O 專案

有了疊代器這樣的新知識，我們可以使用疊代器來改善第十二章的 I/O 專案，讓程式碼更清楚與簡潔。我們來看看疊代器如何改善 `Config::new` 函式與 `search` 函式的實作。

### 使用疊代器移除 `clone`

在範例 12-6 中，我們加了些程式碼來取得 `String` 數值的切片並透過索引切片與克隆數值來產生 `Config` 實例，讓 `Config` 結構體能擁有其數值。在範例 13-24 中，我們重現了範例 12-23 的 `Config::new` 函式實作：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

<span class="caption">範例 13-24：重現範例 12-23 的 `Config::new` 函式</span>

當時我們說先不用擔心 `clone` 呼叫帶來的效率問題，因為我們會在之後移除它們。現在正是絕佳時機！

我們在此需要 `clone` 的原因為我們的參數 `args` 是擁有 `String` 元素的切片，但是 `new` 函式並不擁有 `args`。要回傳 `Config` 實例的所有權，我們必須克隆數值給 `Config` 的 `query` 與 `filename` 欄位，`Config` 實例才能擁有其值。

有了我們新學到的疊代器，我們可以改變 `new` 函式來取得疊代器的所有權來作為引數，而非借用切片。我們會來使用疊代器的功能，而不是檢查切片長度並索引特定位置。這能讓 `Config::new` 函式的意圖更清楚，因為疊代器會存取數值。

一旦 `Config::new` 取得疊代器的所有權並不在使用借用的索引動作，我們就可以從疊代器中移動 `String` 的數值至 `Config` 而非呼叫 `clone` 來產生新的分配。

#### 直接使用回傳的疊代器

請開啟你的 I/O 專案下的 *src/main.rs* 檔案，這應該會看起來像這樣：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

我們會改變範例 12-24 的 `main` 函式開頭段落成範例 13-25 的程式碼。這在我們更新 `Config::new` 之前都還無法編譯。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-25/src/main.rs:here}}
```

<span class="caption">範例 13-25：傳遞 `env::args` 的回傳值給 `Config::new`</span>

`env::args` 函式回傳的是疊代器！與其收集疊代器的數值成一個向量再傳遞切片給 `Config::new`，現在我們可以直接傳遞 `env::args` 回傳的疊代器所有權給 `Config::new`。

接下來，我們需要更新 `Config::new` 的定義。在 I/O 專案的 *src/lib.rs* 檔案中，讓我們變更 `Config::new` 的簽名成範例 13-26 的樣子。這還無法編譯，因為我們需要更新函式本體。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-26/src/lib.rs:here}}
```

<span class="caption">範例 13-26：更新 `Config::new` 的簽名來接收疊代器</span>

標準函式庫技術文件顯示 `env::args` 函式回傳的疊代器型別為 `std::env::Args`。我們更新了 `Config::new` 函式的簽名，讓參數 `args` 的型別為 `std::env::Args` 而非 `&[String]`。因為我們取得了 `args` 的所有權，而且我們需要將 `args` 成為可變的讓我們可以疊代它，所以我們將關鍵字 `mut` 加到 `args` 的參數指定使其成為可變的。

我們還需要指定錯誤型別的字串切片只能是 `'static` 生命週期。因爲我們只會回傳字串字面值，當我們在參數中有引用時，回傳型別的引用就有可能會取得和參數引用一樣的生命週期。我們在第十章討論到的[「生命週期省略」][lifetime-elision]段落就適用於此，所以我們才不需要詮釋 `&str` 的生命週期。但現在 `args` 改變了，所以不再適用生命週期省略的規則，所以我們必須指定 `'static` 生命週期。

#### 使用 `Iterator` 特徵方法而非索引

接下來，我們要修正 `Config::new` 的本體。標準函式庫還提到了 `std::env::Args` 有實作 `Iterator` 特徵，所以我們知道我們可以對它呼叫 `next` 方法！範例 13-27 更新了範例 12-23 的程式碼來使用 `next` 方法：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-27/src/lib.rs:here}}
```

<span class="caption">範例 13-27：變更 `Config::new` 的本體來使用疊代器方法</span>

我們還記得 `env::args` 回傳的第一個數值會是程式名稱。我們想要忽略該值並取得下個數值，所以我們第一次呼叫 `next` 時不會對回傳值做任何事。再來我們才會呼叫 `next` 來取得我們想要的數值置入 `Config` 中的 `query` 欄位。如果 `next` 回傳 `Some` 的話，我們使用 `match` 來提取數值。如果它回傳 `None` 的話，這代表引數不足，所以我們提早用 `Err` 數值回傳。我們對 `filename` 數值也做一樣的事。

### 透過疊代配接器讓程式碼更清楚

我們也可以對 I/O 專案中的 `search` 函式利用疊代器的優勢，範例 13-28 重現了範例 12-19 的程式碼：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

<span class="caption">範例 13-28：範例 12-19 的 `search` 函式實作</span>

我們可以使用疊代配接器（iterator adaptor）方法讓此程式碼更精簡。這樣做也能避免我們產生過程中的 `results` 可變向量。函式程式設計風格傾向於最小化可變狀態的數量使程式碼更加簡潔。移除可變狀態還在未來有機會讓搜尋可以平行化，因為我們不需要去管理 `results` 向量的並行存取。範例 13-29 展示了此改變：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-29/src/lib.rs:here}}
```

<span class="caption">範例 13-29：對 `search` 函式實作使用疊代配接器方法</span>

回想一下 `search` 函式的目的是要回傳 `contents` 中所有包含 `query` 的行數。類似於範例 13-19 的 `filter` 範例，此程式碼使用 `filter` 配接器來只保留 `line.contains(query)` 回傳為 `true` 的行數。我們接著就可以用 `collect` 收集符合的行數成另一個向量。這樣簡單多了！你也可以對 `search_case_insensitive` 函式使用疊代器方法做出相同的改變。

接下來的邏輯問題是在你自己的程式碼中你應該與為何要使用哪種風格呢：是要原本範例 13-28 的程式碼，還是範例 13-29 使用疊代器的版本呢？大多數的 Rust 程式設計師傾向於使用疊代器。一開始的確會有點難上手，不過一旦你熟悉各種疊代配接器與它們的用途後，疊代器就會很好理解了。不同於用迴圈迂迴處理每一步並建構新的向量，疊代器能更專注在迴圈的高階抽象上。這能抽象出常見的程式碼，並能更容易看出程式碼中的重點部位，比如疊代器中每個元素要過濾的條件。

但是這兩種實作真的完全相等嗎？你的直覺可能會假設低階的迴圈可能更快些。讓我們來討論效能吧。

[lifetime-elision]: ch10-03-lifetime-syntax.html#生命週期省略