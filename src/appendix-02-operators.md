## 附錄 B：運算子與符號

此附錄包含 Rust 語法的詞彙表，包含運算子以及其他符號，這些符號會單獨出現或出現在路徑、泛型、特徵界限、巨集、屬性、註解、元組與大括號中。

### 運算子

表 B-1 包含 Rust 中的運算子、運算子如何出現的範例、簡單解釋以及該運算子是否能超載（overloadable）。如果一個運算子可以超載，用來超載該運算子對應的特徵會列出來。

<span class="caption">表 B-1：運算子</span>

| 運算子   | 範例    | 解釋        | 能否超載？    |
|----------|---------|-------------|---------------|
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | 巨集表達式 | |
| `!` | `!expr` | 位元運算（Bitwise）或邏輯運算補數（logical complement） | `Not` |
| `!=` | `var != expr` | 不相等比較 | `PartialEq` |
| `%` | `expr % expr` | 算數餘數 | `Rem` |
| `%=` | `var %= expr` | 算數餘數並賦值 | `RemAssign` |
| `&` | `&expr`, `&mut expr` | 借用 | |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | 借用指標型別 | |
| `&` | `expr & expr` | 位元運算 AND | `BitAnd` |
| `&=` | `var &= expr` | 位元運算 AND 並賦值 | `BitAndAssign` |
| `&&` | `expr && expr` | 邏輯運算 AND | |
| `*` | `expr * expr` | 算數乘法 | `Mul` |
| `*=` | `var *= expr` | 算數乘法並賦值 | `MulAssign` |
| `*` | `*expr` | 解引用 | |
| `*` | `*const type`, `*mut type` | 裸指標 | |
| `+` | `trait + trait`, `'a + trait` | 複合型別約束 | |
| `+` | `expr + expr` | 算數加法 | `Add` |
| `+=` | `var += expr` | 算數加法並賦值 | `AddAssign` |
| `,` | `expr, expr` | 引數與元素分隔符 | |
| `-` | `- expr` | 算數負數 | `Neg` |
| `-` | `expr - expr` | 算數減法 | `Sub` |
| `-=` | `var -= expr` | 算數減法並賦值 | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | 函式與閉包回傳型別 | |
| `.` | `expr.ident` | 成員存取 | |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | 右排除範圍 | |
| `..=` | `..=expr`, `expr..=expr` | 右包含範圍 | |
| `..` | `..expr` | 結構體更新語法 | |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | 「與剩餘部分」模式綁定 | |
| `...` | `expr...expr` | 模式：包含範圍模式 | |
| `/` | `expr / expr` | 算數除法 | `Div` |
| `/=` | `var /= expr` | 算數除法並賦值 | `DivAssign` |
| `:` | `pat: type`, `ident: type` | 約束 | |
| `:` | `ident: expr` | 結構體欄位初始化 | |
| `:` | `'a: loop {...}` | 迴圈標籤 | |
| `;` | `expr;` | 陳述式與項目結束符 | |
| `;` | `[...; len]` | 固定大小陣列語法的其中一部分 | |
| `<<` | `expr << expr` | 左移 | `Shl` |
| `<<=` | `var <<= expr` | 左移並賦值 | `ShlAssign` |
| `<` | `expr < expr` | 小於比較 | `PartialOrd` |
| `<=` | `expr <= expr` | 小於等於比較 | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | 賦值／等值 | |
| `==` | `expr == expr` | 等於比較 | `PartialEq` |
| `=>` | `pat => expr` | 配對分支語法的其中一部分 | |
| `>` | `expr > expr` | 大於比較 | `PartialOrd` |
| `>=` | `expr >= expr` | 大於等於比較 | `PartialOrd` |
| `>>` | `expr >> expr` | 右移 | `Shr` |
| `>>=` | `var >>= expr` | 右移並賦值 | `ShrAssign` |
| `@` | `ident @ pat` | 模式綁定 | |
| `^` | `expr ^ expr` | 位元運算互斥（exclusive）OR | `BitXor` |
| `^=` | `var ^= expr` | 位元運算互斥 OR 並賦值 | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | 模式 OR | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | 位元運算 OR | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | 位元運算 OR 並賦值 | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | 邏輯運算 OR | |
| `?` | `expr?` | 錯誤傳遞 | |

### 非運算子符號

以下列表包含所有不作爲運算子的非字母符號；也就是說，它們的行爲並不像是在呼叫函式或方法。

表 B-2 顯示了出現在各處單獨出現且有效的符號。

<span class="caption">表 B-2：獨立語法</span>

| 符號   | 解釋        |
|--------|-------------|
| `'ident` | 有名稱的生命週期或迴圈標籤 |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | 指定型別的數值字面值 |
| `"..."` | 字串字面值 |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | 原始字串字面值，不會處理跳脫字元 |
| `b"..."` | 位元組字串字面值，其會組織一個 `[u8]` 而非字串 |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | 原始位元組字串字面值，結合原始與位元組字串的字面值 |
| `'...'` | 字元字面值 |
| `b'...'` | ASCII 位元組字面值 |
| <code>&vert;...&vert; expr</code> | 閉包 |
| `!` | 發散函式（diverging functions）的永遠爲空的型別 |
| `_` | 「忽略」模式綁定，也用於整數字面值的可讀性 |

表 B-3 顯示了出現在模組架構中到一個項目的路徑的符號。

<span class="caption">表 B-3：路徑相關語法</span>

| 符號   | 解釋        |
|--------|-------------|
| `ident::ident` | 命名空間路徑 |
| `::path` | 與 crate 源頭相對應的路徑（如顯式絕對路徑） |
| `self::path` | 與目前模組相對應的路徑（如顯式相對路徑） |
| `super::path` | 與上層模組相對應的路徑 |
| `type::ident`, `<type as trait>::ident` | 關聯常數、函式與型別 |
| `<type>::...` | 無法直接命名的型別的關聯項目（如 `<&T>::...`、`<[T]>::...` 等等） |
| `trait::method(...)` | 透過命名其定義的特徵來消除方法呼叫的歧義 |
| `type::method(...)` | 透過命名其定義的型別來消除方法呼叫的歧義 |
| `<type as trait>::method(...)` | 透過命名特徵與型別來消除方法呼叫的歧義 |

Table B-4 顯示出現在泛型型別參數的符號。

<span class="caption">表 B-4：泛型</span>

| 型別   | 解釋        |
|--------|-------------|
| `path<...>` | 指定參數給型別中的泛型型別（如 `Vec<u8>`） |
| `path::<...>`, `method::<...>` | 指定參數給表達式中的泛型型別、函式或方法，通常被稱之爲 turbofish（如 `"42".parse::<i32>()`） |
| `fn ident<...> ...` | 定義泛型函式 |
| `struct ident<...> ...` | 定義泛型結構體 |
| `enum ident<...> ...` | 定義枚舉結構體 |
| `impl<...> ...` | 定義泛型實作 |
| `for<...> type` | 高階生命週期界限 |
| `type<ident=type>` | 其一或數個關聯型別有特定賦值的泛型型別（如 `Iterator<Item=T>`） |

表 B-5 顯式出現在透過特徵界限約束泛型型別參數的符號。

<span class="caption">表 B-5：特徵界限約束</span>

| 符號   | 解釋        |
|--------|-------------|
| `T: U` | 泛型參數 `T` 約束於實作 `U` 的型別 |
| `T: 'a` | 泛型參數 `T` 的生命週期必須比 `'a` 還長（代表該型別無法傳遞包含任何聲明週期短於 `'a` 的因引用） |
| `T : 'static` | 泛型型別 `T` 不包含 `'static` 以外的借用引用 |
| `'b: 'a` | 泛型生命週期 `'b` 必須長於 `'a` |
| `T: ?Sized` | 允許泛型型別參數爲動態大小型別 |
| `'a + trait`, `trait + trait` | 複合型別約束 |

表 B-6 顯示出現在呼叫或定義巨集與指定項目屬性的符號。

<span class="caption">表 B-6：巨集與屬性</span>

| 符號   | 解釋        |
|--------|-------------|
| `#[meta]` | 外部屬性 |
| `#![meta]` | 內部屬性 |
| `$ident` | 巨集替代 |
| `$ident:kind` | 巨集捕獲 |
| `$(…)…` | 巨集重複 |
| `ident!(...)`, `ident!{...}`, `ident![...]` | 巨集調用 |

表 B-7 顯示建立註解的符號。

<span class="caption">表 B-7：註解</span>

| 符號   | 解釋        |
|--------|-------------|
| `//` | 行註解 |
| `//!` | 內部行技術文件註解 |
| `///` | 外部行技術文件註解 |
| `/*...*/` | 區塊註解 |
| `/*!...*/` | 內部區塊技術文件註解 |
| `/**...*/` | 外部區塊技術文件註解 |

表 B-8 顯示出現在元組中的符號。

<span class="caption">表 B-8：元組</span>

| 符號   | 解釋        |
|--------|-------------|
| `()` | 空元組（也稱爲單元），同時是字面值與型別 |
| `(expr)` | 括號表達式 |
| `(expr,)` | 單一元素元組表達式 |
| `(type,)` | 單一元素元組型別 |
| `(expr, ...)` | 元組表達式 |
| `(type, ...)` | 元組型別 |
| `expr(expr, ...)` | 函式呼叫表達式，也用來初始化元組 `struct` 與元組 `enum` 變體 |
| `expr.0`, `expr.1`, etc. | 元組索引 |

表 B-9 顯在大括號使用到的地方。

<span class="caption">表 B-9：大括號</span>

| 符號    | 解釋        |
|---------|-------------|
| `{...}` | 區塊表達式 |
| `Type {...}` | `struct` 字面值 |

表 B-10 顯示中括號使用到的地方。

<span class="caption">表 B-10：中括號</span>

| 符號    | 解釋        |
|---------|-------------|
| `[...]` | 陣列字面值 |
| `[expr; len]` | 包含 `len` 個 `expr` 的陣列字面值 |
| `[type; len]` | 包含 `len` 個 `type` 的陣列字面值 |
| `expr[expr]` | 集合索引，可超載（`Index`、`IndexMut`） |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | 使用 `Range`、`RangeFrom`、`RangeTo` 或 `RangeFull` 作爲「索引」來替代集合 slice 的集合索引 |

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/appendix-02-operators.md)
> - updated: 2020-09-29
