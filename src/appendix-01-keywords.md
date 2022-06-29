## 附錄 A：關鍵字

以下列表包含 Rust 目前或未來會使用到而保留起來的關鍵字。這意味著它們不能作為標識符使用（不過等等會提到的「[原始標識符][raw-identifiers]<!-- ignore -->」除外），這包含函式、變數、參數、結構體欄位、模組、crates、常數、巨集、靜態數值、屬性、型別、特徵與生命週期的名稱。

[raw-identifiers]: #原始標識符

### 目前有在使用的關鍵字

以下為目前的關鍵字列表與其對應的功能描述。

* `as` - 進行原始型別轉換、消除包含項目的特定特徵，或重新命名 `use` 陳述式內的項目
* `async` -  回傳 `Future` 而非阻擋目前執行緒
* `await` - 暫停執行直到 `Future` 的結果已經準備好
* `break` - 立即離開迴圈
* `const` - 定義常數項目或常數裸指標
* `continue` - 繼續進入下一次迴圈疊代
* `crate` - 在模組路徑中，指的是 crate 的源頭
* `dyn` - 對特徵物件的動態分配
* `else` - `if` 與 `if let` 控制流結構的例外選項
* `enum` - 定義枚舉
* `extern` - 連結外部函式或變數
* `false` - 布林字面值 false
* `fn` - 定義函式或函式指標型別
* `for` - 從疊代器遍歷項目、實作特徵，或指定高階生命週期（higher-ranked lifetime）
* `if` - 依據條件表達式的分支
* `impl` - 實作本身或特徵的功能
* `in` - `for` 迴圈語法的其中一部分
* `let` - 綁定變數
* `loop` - 無條件的迴圈
* `match` - 將數值配對到模式
* `mod` - 定義模組
* `move` - 讓閉包取得其所有捕獲的所有權
* `mut` - 表示引用、裸指標或模式綁定具有可變性
* `pub` - 表示結構欄位、`impl` 區塊或模組對外公開
* `ref` - 綁定引用
* `return` - 函式的回傳
* `Self` - 我們正在定義或實作型別的型別別名
* `self` - 方法本體或當前模組
* `static` - 全域變數或存在於整個程式執行期間的生命週期
* `struct` - 定義結構體
* `super` - 當前模組的上層模組
* `trait` - 定義特徵
* `true` - 布林字面值 true
* `type` - 定義型別別名或關聯型別
* `union` - 定義聯集 [union][union]<!-- ignore -->; 只作為宣告聯集時的關鍵字
* `unsafe` - 表示不安全的程式碼、函式、特徵或實作
* `use` - 將符號引入作用域
* `where` - 表示約束該型別用的子句
* `while` - 依據表達式結果的條件迴圈

[union]: https://doc.rust-lang.org/reference/items/unions.html

### 未來可能會使用而保留的關鍵字

以下關鍵字沒有任何功能但是 Rust 可能會在未來使用到所以作為保留：

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### 原始標識符

*原始標識（Raw identifiers）*是個能讓你使用正常情況下不允許使用的關鍵字的語法。你能夠過加上 `r#` 關鍵字前綴來使用原始標識符。

舉例來說，`match` 是個關鍵字。如果你嘗試編譯以下使用 `match` 作為名稱的函式的話：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

你會獲得此錯誤：

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

錯誤表示你不能使用關鍵字 `match` 作為函式標識符。要使用 `match` 作為函式名稱的話，你可以使用原始標識符語法，如以下所示：

<span class="filename">檔案名稱：src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

此程式碼就能夠編譯並沒有任何錯誤。注意到 `r#` 前綴會用於函式定義的名稱以及在 `main` 呼叫該函式的地方。

原始標識符能讓你使用任何字作為標識符，就算該字剛好是保留的關鍵字。這給了我們更多選擇標識符名稱的自由，以及讓我們與以這些單詞不是關鍵詞的語言編寫的程式進行整合。除此之外，原始標識符讓你可以使用與你 crate 的 Rust 版號（edition）不相同的函式庫。舉例來說 `try` 在 2015 版號還不是關鍵字，但到 2018 版號才加入。如果你依賴一個使用 2015 版號的函式庫，且其中有個 `try` 函式，你就需要使用原始標識符語法。在此例中就是在你 2018 版號的程式碼用 `r#try` 來呼叫該函式。情查閱[附錄 E][appendix-e]<!-- ignore -->以瞭解關於版號的更多資訊。

[appendix-e]: appendix-05-editions.html
