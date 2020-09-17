## 進階函式與閉包

接下來，我們會探索函式與閉包相關的進階特色，包括函式指標和回傳閉包。

### 函式指標

我們已探討過如何將閉包傳遞給函式，其實你還可以將一般的函式傳給函式！當你想要傳遞已經定義好的函式，而不是新的閉包時，就會凸顯這個技巧好用之處。有了函式指標，就可以將函式當作其他函式的引數，而這個作為引數的函式會轉型為 `fn` 型別（小寫的 f），別和閉包特徵的 `Fn` 搞混了。這個 `fn` 型別就稱為**函式指標（function pointer）**。其語法與閉包語法類似，可用於將函式指標作為參數的型別，如範例 19-27 所示。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-27/src/main.rs}}
```

<span class="caption">範例 19-27：藉由 `fn` 型別接收函式指標引數</span>

這段程式碼會印出 `答案是：12`。我們可以指定 `do_twice` 的參數 `f` 是一個需要一個 `i32` 當參數的 `fn`，並會回傳 `i32`。接下來我們在 `do_twice` 內呼叫 `f`。在 `main` 中，我們就可將 `add_one` 函式作為 `do_twice` 第一個引數。

和閉包不同的是，`fn` 不是特徵而是一個型別，所以我們可以直接將 `fn` 作為參數型別，而不需要宣告一個以 `Fn` 特徵作為特徵限制的泛型型別參數。


函式指標將 三個閉包特徵（`Fn`、`FnMut`、`FnOnce`）通通實作了，所以在預期要傳入閉包之處，你一定可以將函式指標作為引數傳進去。最佳的 做法是寫一個同時使用泛型型別和其中一個閉包特徵的函式，這樣無論是函式還是閉包，你的函式全都可以接收。

有個你只會想接收 `fn` 但不要閉包例子，就是當你在與外部那些沒有閉包的程式碼打交道的時候，比如 C 可以接收函式作為引數，但 C 並沒有閉包。

讓我們來看一下 `map` 的用法，`map` 就是可以用行內閉包（closure defined inline）或一個命名函式（named function）的例子。欲將數字的 vector 轉換成字串的 vector，我們可以使用閉包，例如：

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

或者，我們也可以將一個函式作為引數，代替閉包傳入 `map`：

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

請注意，因為有多個可用的函式都叫做 `to_string`，所以我們必須使用先前在[「進階特徵」][進階特徵]一節提及的完全限定語法。這裡，我們使用了在 `ToString` 特徵中定義的 `to_string` 函式，只要有實作 `Display` 的型別，標準函式庫都會提供 `ToString` 的實作。

另一個實用模式是深度利用元組結構體和在枚舉變體（enum variant）中的的元組結構體的實作細節。這些型別以 `()` 作為初始化語法，看起來就像函式呼叫。事實上這些初始化以函式來實作，這個函式接收引數來建立並回傳一個實例。我們可以將這些初始化函式當作實作了閉包特徵的函式指標，這就代表我們可以指定初始化函式作為引數，傳給需要閉包的方法，例如：

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```
這裡，我們對一個範圍呼叫 `map`，並用每個 `u32` 值，透過 `Status::Value` 的初始化函式來建立 `Status::Value` 的實例。有些人更喜歡上述的作法，但有人偏好閉包。這兩者的編譯結果相同，所以選一個你覺得清晰的風格吧。

### 回傳閉包

閉包是用特徵來表示，言下之意是你不能直接回傳一個閉包。大多數的情況，當你想回傳一個特徵時，可以改回傳有實作該特徵的具體型別。但你並無法對閉包這樣做，因為它們根本沒有可供回傳的具體型別，比方說不允許你使用 `fn` 函式指標作為回傳型別。

接下來的程式碼嘗試直接回傳一個閉包，但它無法編譯：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

編譯錯誤如下：

```console
{{#include ../listings/ch19-advanced-features/no-listing-18-returns-closure/output.txt}}
```

這個錯誤再度指出 `Sized` 特徵！Rust 不知道我們需要多少空間儲存這個閉包，我們之前看過這類問題的解法。可以使用特徵物件（trait object）：

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```

這段程式碼恰巧能通過編譯。欲知更多特徵物件相關資訊，請查閱第十七章[「允許不同型別數值的特徵物件」][允許不同型別數值的特徵物件]部分。

接下來，一起來關注巨集吧！

[進階特徵]: ch19-03-advanced-traits.html#進階特徵
[允許不同型別數值的特徵物件]: ch17-02-trait-objects.html#允許不同型別數值的特徵物件

> - translators: [Weihang Lo <me@weihanglo.tw>]
> - commit: [d44317c](https://github.com/rust-lang/book/blob/d44317c3122b44fb713aba66cc295dee3453b24b/src/ch19-05-advanced-functions-and-closures.md)
> - updated: 2020-09-17
