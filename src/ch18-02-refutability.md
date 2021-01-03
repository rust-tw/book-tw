## 可反駁性：何時模式可能會配對失敗

模式有兩種形式：可反駁的（refutable）與不可反駁的（irrefutable）。可以配對任何可能數值的模式屬於**不可反駁的（irrefutable）**。其中一個例子就是陳述式 `let x = 5;` 中的 `x`，因為 `x` 可以配對任何數值，因此不可能會配對失敗。而可能會對某些數值配對失敗的模式則屬於**可反駁的（refutable）**。其中一個例子是表達式 `if let Some(x) = a_value` 中的 `Some(x)`，因為如果 `a_value` 變數中的數值為 `None` 而非 `Some` 的話， `Some(x)` 模式就會配對失敗。

函式參數、`let` 陳述式與 `for` 迴圈只能接受不可反駁的模式，當數值無法配對時，程式無法作出任何有意義的事。`if let` 與 `while let` 表達式接受可反駁與不可反駁的模式，但是編譯器會警告不可反駁的模式，因為定義上來說它們用來處理可能會失敗的場合，條件表達式的功能就是依據成功或失敗來執行不同動作。

大致上來說，你通常不需要擔心可反駁與不可反駁模式之間的區別，不過你會需要熟悉可反駁性這樣的概念，所以當你看到錯誤訊息時，能及時反應理解。在這樣的場合，你需要依據程式碼的預期行為來改變模式或是使用模式的結構。

讓我們看看當我們嘗試在 Rust 要求不可反駁模式的地方使用可反駁模式的範例與其反例。範例 18-8 顯示了一個 `let` 陳述式，但是我們指定的模式是 `Some(x)`，這是可反駁模式。如我們所預期的，此程式碼無法編譯。

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-08/src/main.rs:here}}
```

<span class="caption">範例 18-8：嘗試在 `let` 使用可反駁模式</span>

如果 `some_option_value` 是數值 `None`，它會無法與模式 `Some(x)` 做配對，這意味著此模式是可反駁的。但是 `let` 陳述式只能接受不可反駁的模式，因為 程式碼對 `None` 數值就無法作出任何有效的動作。在編譯時 Rust 就會抱怨我們嘗試在需要不可反駁模式的地方使用了可反駁模式：

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-08/output.txt}}
```

因為 `Some(x)` 模式沒有涵蓋（且也涵蓋不了！）所有有效的數值，Rust 合理地產生了編譯錯誤。

要修正在需要不可反駁模式的地方使用可反駁模式的錯誤，我們可以變更使用此模式的程式碼，與其使用 `let`，我們可以改用 `if let`。這樣如果模式不符的話，程式碼就會跳過大括號中的程式碼，讓我們可以繼續有效執行下去。範例 18-9 顯示了如何修正範例 18-8 的程式碼。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-09/src/main.rs:here}}
```

<span class="caption">範例 18-9：使用 `if let` 而非 `let` 來使用可反駁模式</span>

我們給了程式碼出路！此程式碼可以完美執行，雖然這也代表我們使用不可反駁模式的話會得到一些警告。如果我們給予 `if let` 一個像是 `x` 這樣永遠能配對的模式的話，編譯器會出現警告，如範例 18-10 所示。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-10/src/main.rs:here}}
```

<span class="caption">範例 18-10：嘗試在 `if let` 使用不可反駁的模式</span>

Rust 會抱怨說在 `if let` 使用不可反駁的模式沒有任何意義：

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-10/output.txt}}
```

基於此原因，`match` 的分支必須是可反駁模式。除了最後一個分支因為要配對任何剩餘數值，所以會是不可反駁模式。Rust 允許我們在 `match` 只使用一個不可反駁模式的分支，不過這樣做並不是很實用，且可以直接用簡單的 `let` 陳述式取代。

現在你知道哪裡能使用模式，以及可反駁與不可反駁模式的不同了。讓我們來涵蓋模式建立時可以使用的所有語法吧。

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch18-02-refutability.md)
> - updated: 2020-09-25
