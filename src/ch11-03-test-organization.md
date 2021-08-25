## 測試組織架構

如同本章開頭提到的，測試是個複雜的領域，不同的人可能使用不同的術語與組織架構。Rust 社群將測試分為兩大分類術語：**單元測試（unit tests）**和**整合測試（integration tests）**。單元測試比較小且較專注，傾向在隔離環境中一次只測試一個模組，且能夠測試私有介面。整合測試對於你的函式庫來說是個完全外部的程式碼，所以會如其他外部程式碼一樣使用你的程式碼，只能使用公開介面且每個測試可能會有數個模組。

這兩種測試都很重要，且能確保函式庫每個部分能在分別或一起執行的情況下，如你預期的方式運作。

### 單元測試

單元測試的目的是要在隔離其他程式碼的狀況下測試每個程式碼單元，迅速查明程式碼有沒有如預期或非預期的方式運作。你會將單元測試放在 *src* 目錄中每個你要測試的程式同個檔案下。我們常見的做法是在每個檔案建立一個模組 `tests` 來包含測試函式，並用 `cfg(test)` 來詮釋模組。

#### 測試模組與 `#[cfg(test)]`

測試模組上的 `#[cfg(test)]` 詮釋會告訴 Rust 當你執行 `cargo test` 才會編譯並執行測試程式碼。而不是當你執行 `cargo build`。當你想要建構函式庫時，這能節省編譯時間並降低編譯出的檔案所佔的空間，因為這些測試沒有被包含到。整合測試位於不同目錄，所以它們不需要 `#[cfg(test)]`。但是因為單元測試與程式碼位於相同的檔案下，你需要使用 `#[cfg(test)]` 來指明它們不應該被包含在編譯結果。

回想一下本章節第一個段落中我們建立了一個新專案 `adder`，並用 Cargo 為我們產生以下程式碼：

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

此程式碼是自動產生的測試模組。`cfg` 屬性代表的是 *configuration* 並告訴 Rust 以下項目只有在給予特定配置選項時才會被考慮。在此例中配置選項是 `test`，這是 Rust 提供用來編譯與執行測試的選項。使用 `cfg` 屬性的話，Cargo 只有在我們透過 `cargo test` 執行測試時才會編譯我們的測試程式碼。這包含此模組能可能需要的輔助函式，以及用 `#[test]` 詮釋的測試函式。

#### 測試私有函式

在測試領域的社群中對於是否應該直接測試私有函式一直存在著爭議，而且有些其他語言會讓測試私有函式變得很困難，甚至不可能。不管你認為哪個論點比較理想，Rust 的隱私權規則還是能讓你測試私有函式。考慮以下範例 11-12 擁有私有函式 `internal_adder` 的程式碼。

<span class="filename">檔案名稱：src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

<span class="caption">範例 11-12：測試私有函式</span>

注意到函式 `internal_adder` 沒有標記為 `pub`。測試也只是 Rust 的程式碼，且 `tests` 也只是另一個模組。如同我們在[引用模組項目的路徑][paths]<!-- ignore -->段落討論到的，下層模組的項目可以使用該項目以上的模組。在此測試中，我們透過 `use super::*` 引入 `test` 模組上層的所有項目，所以測試能呼叫 `internal_adder`。如果你不認為私有函式應該測試，Rust 也沒有什麼好阻止你的地方。

### 整合測試

在 Rust 中，整合測試對你的函式庫來說是完全外部的程式。它們使用你的函式庫的方式與其他程式碼一樣，所以它們只能呼叫屬於函式庫中公開 API 的函式。它們的目的是要測試你的函式庫屬個部分一起運作時有沒有正確無誤。單獨運作無誤的程式碼單元可能會在整合時出現問題，所以整合測試的程式碼的涵蓋率也很重要。要建立整合測試，你需要先有個 *tests* 目錄。

#### *tests* 目錄

我們在專案目錄最上層在 *src* 旁建立一個 *tests* 目錄。Cargo 知道要從此目錄來尋找整合測試。我們接著就可以在此目錄建立多少個測試都沒問題，Cargo會編譯每個檔案成獨立的 crate。

讓我們來建立一個整合測試，將範例 11-12 的程式碼保留在 *src/lib.rs* 檔案中，然後建立一個 *tests* 目錄、一個叫做 *tests/integration_test.rs* 的檔案並輸入範例 11-13 的程式碼。

<span class="filename">檔案名稱：tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

<span class="caption">範例 11-13：`adder` crate 中函式的整合測試</span>

我們在程式最上方加了 `use adder`，這在單元測試是不需要的。這裡要用到的原因是因為 `tests` 目錄的每個檔案都是獨立的 crate，所以我們需要將函式庫引入每個測試 crate 的作用域中。

我們不用對 *tests/integration_test.rs* 的任何程式碼詮釋 `#[cfg(test)]`。Cargo 會特別對待 `tests` 目錄並只在我們執行 `cargo test` 時，編譯此目錄的檔案。現在請執行 `cargo test`：

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

輸出結果中有三個段落，包含單元測試、整合測試與技術文件測試。第一個段落的單元測試與我們看過的相同：每行會是每個單元測試（在此例是我們在範例 11-12 寫得 `internal`）最後附上單元測試的總結。

整合測試段落從 `Running target/debug/deps/integration_test-1082c4b063a8fbe6` 開始（最後的雜湊值（hash）可能會與你的輸出不同），接著每行會是每個整合測試的測試函式，最後在 `Doc-tests adder` 段落開始前的那一行則是整合測試的總結結果。

當我們加入更多單元測試時，單元測試段落就會顯示更多結果。同樣地當我們將更多測試函式加入整合測試檔案內的話，該整合測試段落就會顯示更多結果。每個整合測試檔案會有自己的段落，如果如果我們在 *tests* 目錄加入更多檔案的話，就會出現更多整合測試段落。

我們一樣能用測試函式的名稱來作為 `cargo test` 的引數，來執行特定整合測試。要執行特定整合測試檔案內的所有測試，可以用 `--test` 作為 `cargo test` 的引數並加上檔案名稱：

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

此命令會只執行 *tests/integration_test.rs* 檔案內的測試。

#### 整合測試的子模組

隨著你加入的整合測試越多，你可能會想要在 *tests* 目錄下產生更多檔案來協助組織它們。舉例來說，你以用測試函式測試的功能來組織它們。如同稍早提到的，*tests*  目錄下的每個檔案都會編譯成自己獨立的 crate。

將每個整合測試檔案視為獨立的 crate 有助於建立不同的作用域，這就像是使用者使用你的 crate 的可能環境。然而這也代表 *tests* 目錄的檔案不會和 *src* 的檔案行為一樣，也就是你在第七章學到如何拆開程式碼成模組與檔案的部分。

當你希望擁有一些能協助數個整合測試檔案的輔助函式，並遵循第七章的[「將模組拆成不同檔案」][separating-modules-into-files]<!-- ignore -->段落來提取它們到一個通用模組時，你就會發現 *tests* 目錄下的檔案行為是不同的。舉例來說，我們建立了 *tests/common.rs* 並寫了一個函式 `setup`，然後我們希望 `setup` 能被不同測試檔案的數個測試函式呼叫：

<span class="filename">檔案名稱：tests/common.rs</span>

```rust
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

當我們再次執行程式時，我們會看到測試輸出多了一個 *common.rs* 檔案的段落，就算該檔案沒有包含任何測試函式，而且我們也還沒有在任何地方呼叫 `setup` 函式：

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

讓 `common` 出現在測試結果並顯示 `running 0 tests` 並不是我們想做的事。我們只是想要分享一些程式碼給其他整合測試檔案而已。

要防止 `common` 出現在測試輸出，我們不該建立 *tests/common.rs*，而是要建立 *tests/common/mod.rs*。這是另一個 Rust 知道的常用命名手段。這樣命名檔案的話會告訴 Rust 不要將 `common` 模組視為整合測試檔案。當我們將 `setup` 函式程式碼移到 *tests/common/mod.rs* 並刪除 *tests/common.rs* 檔案時，原本的段落就不會再出現在測試輸出。*tests* 目錄下子目錄的檔案不會被編譯成獨立 crate 或在測試輸出顯示段落。

在我們建立 *tests/common/mod.rs* 之後，我們可以將它以模組的形式用在任何整合測試檔案中。以下是在 *tests/integration_test.rs* 的 `it_adds_two` 測試中呼叫函式 `setup` 的範例：

<span class="filename">檔案名稱：tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

注意到 `mod common;` 的宣告與我們在範例 7-21 說明的模組宣告方式一樣。然而後在測試函式中，我們就可以呼叫函式 `common::setup()`。

#### 二進制執行檔 Crate 的整合測試

如果我們的專案是只包含 *src/main.rs* 檔案的二進制執行檔 crate 而沒有 *src/lib.rs* 檔案的話，我們無法在 *tests* 目錄下建立整合測試，也無法將 *src/main.rs* 檔案中定義的函式透過 `use` 陳述式引入作用域。只有函式庫 crate 能公開函式給其他 crate 使用，二進制 crate 只用於獨自執行。

這也是為何 Rust 專案為二進制執行檔提供直白的 *src/main.rs* 檔案並允許呼叫 *src/lib.rs* 檔案中的邏輯程式碼。使用這樣子的架構的話，整合測試**可以**透過 `use` 來測試函式庫 crate，並讓重點功能可以公開使用。如果重點功能可以運作的話，那 *src/main.rs* 檔案中剩下的程式碼部分也能夠如期執行，而這一小部分就不必特定做測試。

## 總結

Rust 的測試功能提供了判定程式碼怎樣才算正常運作的方法，以確保它能以你預期的方式運作，就算當你做了改變時也是如此。單元測試分別測試函式庫中每個不同的部分，且能測試私有實作細節。整合測試檢查函式庫數個部分一起執行時是否正確無誤，且它們使用函式庫公開 API 來測試程式碼的行為與外部程式碼使用的方式一樣。雖然 Rust 型別系統與所有權規則能避免某些種類的程式錯誤，測試還是減少邏輯程式錯誤的重要辦法，讓你的程式碼能如預期行為運作。

讓我們統整此章節以及之前的章節所學到的知識來寫一支專案吧！

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]:
ch07-05-separating-modules-into-different-files.html
