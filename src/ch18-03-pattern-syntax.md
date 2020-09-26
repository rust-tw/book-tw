## 模式語法

在整本書中，我們已經看過許多種類的模式範例了。在此段落中，我們會收集所有模式中的有效語法，並討論你會怎麼使用它們。

### 配對字面值

如同你在第六章所見的，你可以直接使用字面值來配對模式，以下程式碼展示了一些範例：

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

此程式碼會顯示 one 因爲 x 的數值爲 1。此語法適用於當你想要程式碼取得一個特定數值時，就馬上採取行動的情況。

### 配對變數名稱

變數名稱是能配對任何數值的不可反駁模式，而且我們在本書中已經使用非常多次。不過當你在 `match` 表達式中使用變數名稱時會複雜一點。因為 `match` 會初始一個新的作用域，作為 `match` 表達式部分模式的宣告變數會遮蔽 `match` 結構外同名的變數，和所有變數一樣。在範例 18-11 中，我宣告了一個變數叫做 `x` 其有數值 `Some(5)` 和一個變數 `y` 其有數值 `10`。然後我們建立一個數值 `x` 的 `match` 表達式。檢查配對分之中的模式並在最後用 `println!` 顯示出來，並嘗試在程式碼執行或進一步閱讀之前推測其會顯示的結果會為何。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-11/src/main.rs:here}}
```

<span class="caption">範例 18-11：`match` 表達式其中一個分支宣告了遮蔽的變數 `y`</span>

讓我們跑一遍看看當 `match` 執行時發生了什麼事。第一個配對分支並不符合 `x` 定義的數值，所以程式繼續執行下去。

第二個配對分支宣告了一個新的變數叫做 `y` 來配對 `Some` 內的任何數值。因為我們位於 `match` 表達式內的新作用域，此新的 `y` 變數並不是我們一開始宣告有數值 10 的 `y`。這個新的 `y` 會配對 `Some` 內的任何數值，，也就是 `x` 擁有的數值。因此，這個新的 `y` 會綁定 `x` 中 `Some` 的內部數值。該數值是 `5`，所以該分支的表達式就會執行並印出 `Matched, y = 5`。

如果 `x` 是 `None` 數值而非 `Some(5)` 的話，前兩個分支的模式都不會配對到，所以數值會配對到底線的分支。我們沒有在底線分支的模式中宣告 `x` 變數，所以表達式中的 `x` 仍然是外部沒有被遮蔽的 `x` 。在這樣的假設狀況下，`match` 會印出 `Default case, x = None`。

當 `match` 完成時，其作用域就會結束，所以作用域內的內部 `y` 也會結束。最後一個 `println!` 會顯示 `at the end: x = Some(5), y = 10`。

要建立個能對外部 `x` 與 `y` 數值做比較的 `match` 表達式而非遮蔽變數的話，我們需要改用條件配對防護。我們會在之後的[「提供額外條件的配對防護」](#extra-conditionals-with-match-guards)<!-- ignore -->段落討論配對防護。

### 多重模式

在 `match` 表達式中，你可以使用 `|` 語法來配對數個模式，其代表*或*的意思。舉例來說，以下程式碼會配對 `x` 的數值到配對分支，第一個分支有個*或者*的選項，代表如果 `x` 的數值配對的到分支中任一數值的話，該分支的程式碼就會執行：

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

此程式碼會印出 `one or two`。

### 透過 `..=` 配對數值範圍

`..=` 語法讓我們可以配對一個範圍內包含的數值。在以下程式碼中，當模式配對的到範圍內的任何數值時，該分支就會執行：

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

如果 `x` 是 1、2、3、4 或 5 的話，第一個分支就能配對到。此語法比使用 `|` 運算子來表達相同概念還輕鬆得多。如果我們使用 `|` 的話，就得指明 `1 | 2 | 3 | 4 | 5` 而非 `1..=5`。指定範圍相對就簡短許多，尤其是如果我們得配對像是數字 1 到 1,000 的話！

範圍只允許使用數字或 `char` 數值，因為編譯器會在編譯時檢查範圍是否為空。`char` 與數字數值是 Rust 中唯一能判斷範圍是否為空的型別。

以下是使用 `char` 數值作為範圍的範例：

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust 可以知道 `c` 有包含在第一個模式的範圍內，所以印出 `early ASCII letter`。

### 解構拆開數值

我們可以使用模式來解構結構體、枚舉、元組與引用，以便使用這些數值的不同部分。讓我們依序來看看。

#### 解構結構體

範例 18-12 有個結構體 `Point` 其有兩個欄位 `x` 與 `y`，我們可以在 `let` 陳述式使用模式來拆開它。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-12/src/main.rs}}
```

<span class="caption">範例 18-12：解構結構體欄位成獨立的變數</span>

此程式碼建立了變數 `a` 與 `b` 來配對 `p` 結構體中 `x` 與 `y` 的欄位。此範例顯示出模式中的變數名稱不必與結構體中的欄位名稱一樣。不過通常還是建議變數名稱與欄位名稱一樣，以便記得哪些變數來自於哪個欄位。

因為用變數名稱來配對欄位是十分常見的，而且因為 `let Point { x: x, y: y } = p;` 會包含許多重複部分，所以配對結構體欄位的模式有另一種簡寫方式，你只需要列出結構體欄位的名稱，這樣從結構體建立的變數名稱就會有相同名稱。範例 18-13 顯示的程式碼行為與範例 18-12 一樣，但是在 `let` 模式建立的變數是 `x` 與 `y` 而非 `a` 與 `b`。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-13/src/main.rs}}
```

<span class="caption">範例 18-13：使用結構體欄位簡寫來解構結構體欄位</span>

此程式碼建立了變數 `x` 與 `y` 並配對到變數 `p` 的 `x` 與 `y` 欄位。結果就是變數 `x` 與 `y` 會包含 `p` 結構體中的數值。

我們也可以將字面值數值作為結構體模式中的一部分，而不用建立所有欄位的變數。這樣做我們可以在解構一些欄位成變數時，測試其他欄位是否有特定數值。

範例 18-14 的 `match` 表達式將 `Point` 的數值分成三種情況：位於 `x` 軸的點（也就是 `y = 0`）、位於 `y` 軸的點（`x = 0`） 或不在任何軸的點。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-14/src/main.rs:here}}
```

<span class="caption">範例 18-14：解構並配對模式中的字面值數值</span>

第一個分支透過指定 `y` 欄位配對字面值為 `0` 來配對任何在 `x` 軸上的點。此模式仍然會建立變數 `x` 能讓我們在此分支的程式碼中使用。

同樣地，第二個分支透過指定 `x` 欄位配對字面值為 `0` 來配對任何在 `y` 軸上的點，並建立擁有 `y` 欄位數值的變數 `y`。 第三個分支沒有指定任何字面值，所以它能配對任何其他 `Point` 並建立 `x` 與 `y` 欄位對應的變數。

在此例中，數值 `p` 會配對到第二個分支，因為其 `x` 為 0，所以此程式碼會印出 `On the y axis at 7`。

#### 解構枚舉

我們已經在本書中之前的章節就解構過枚舉。舉例來說，第六章的範例 6-5 我們就解構了 `Option<i32>`。其中一個我們還沒談到的細節是解構枚舉的模式必須與枚舉定義中其所儲存的資料相符。作為示範，我們在範例 18-15 中使用範例 6-2 的 `Message` 枚舉並寫一個 `match` 來提供會解構每個內部數值的模式。

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-15/src/main.rs}}
```

<span class="caption">範例 18-15：解構持有不同種數值的枚舉變體</span>

此程式碼會印出 `Change the color to red 0, green 160, and blue 255`。請嘗試變更 `msg` 的數值來看看其他分支的程式碼會執行出什麼。

對於像是 `Message::Quit` 這種沒有任何資料的枚舉，我們無法進一步解構出任何資料。我們只能配對其本身的數值 `Message::Quit`，所以在該模式中沒有任何變數。

對於像是 `Message::Move` 這種類結構體枚舉變體，我們可以使用類似於指定配對結構體的模式。在變體名稱之後，我們加上大括號以及列出欄位名稱作為變數，讓我們能拆成不同部分並在此分支的程式碼中使用。我們在此使用範例 18-13 一樣的簡寫形式。

對於像是 `Message::Write` 這種持有一個元素，以及 `Message::ChangeColor` 這種持有三個元素的類元組枚舉變體，我們可以使用類似於配對元組的模式。模式中的變數數量必須與我們要配對的變體中元素數量相符。

#### 解構巢狀結構體與枚舉

到目前為止，我們所有的結構體或枚舉配對範例的深度都只有一層。配對也可以用於巢狀項目中！

舉例來說，我們可以重構範例 18-15 的程式碼，在 `ChangeColor` 中支援 RGB 與 HSV 顏色，如範例 18-16 所示。

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-16/src/main.rs}}
```

<span class="caption">範例 18-16：配對巢狀枚舉</span>

`match` 表達式的第一個分支模式會配對包含 `Color::Rgb` 變體的 `Message::ChangeColor` 枚舉變體，然後該模式會綁定內部三個 `i32` 數值。第二個分支也是配對到 `Message::ChangeColor` 枚舉變體，但是內部枚舉會改配對 `Color::Hsv`。我們可以在一個 `match` 表達式指定這些複雜條件，即使有兩個枚舉參與其中。

#### 解構結構體與元組

我們甚至可以用更複雜的方式來混合、配對並巢狀解構模式。以下範例展示了一個複雜的結構模式，其將一個結構體與一個元組置於另一個元組中，並將所有的原始數值全部解構出來：

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

此程式碼讓我們將複雜的型別拆成部分元件，讓我們可以分別使用我們有興趣的數值。

解構模式是個能方便使用部分數值的方式，比如結構體每個欄位分別獨立的數值。

### 忽略模式中的數值

You’ve seen that it’s sometimes useful to ignore values in a pattern, such as
in the last arm of a `match`, to get a catchall that doesn’t actually do
anything but does account for all remaining possible values. There are a few
ways to ignore entire values or parts of values in a pattern: using the `_`
pattern (which you’ve seen), using the `_` pattern within another pattern,
using a name that starts with an underscore, or using `..` to ignore remaining
parts of a value. Let’s explore how and why to use each of these patterns.

#### Ignoring an Entire Value with `_`

We’ve used the underscore (`_`) as a wildcard pattern that will match any value
but not bind to the value. Although the underscore `_` pattern is especially
useful as the last arm in a `match` expression, we can use it in any pattern,
including function parameters, as shown in Listing 18-17.

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-17/src/main.rs}}
```

<span class="caption">範例 18-17: Using `_` in a function signature</span>

This code will completely ignore the value passed as the first argument, `3`,
and will print `This code only uses the y parameter: 4`.

In most cases when you no longer need a particular function parameter, you
would change the signature so it doesn’t include the unused parameter. Ignoring
a function parameter can be especially useful in some cases, for example, when
implementing a trait when you need a certain type signature but the function
body in your implementation doesn’t need one of the parameters. The compiler
will then not warn about unused function parameters, as it would if you used a
name instead.

#### Ignoring Parts of a Value with a Nested `_`

We can also use `_` inside another pattern to ignore just part of a value, for
example, when we want to test for only part of a value but have no use for the
other parts in the corresponding code we want to run. Listing 18-18 shows code
responsible for managing a setting’s value. The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting but can unset the setting and give it a value if it is currently unset.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-18/src/main.rs:here}}
```

<span class="caption">範例 18-18: Using an underscore within patterns that
match `Some` variants when we don’t need to use the value inside the
`Some`</span>

This code will print `Can't overwrite an existing customized value` and then
`setting is Some(5)`. In the first match arm, we don’t need to match on or use
the values inside either `Some` variant, but we do need to test for the case
when `setting_value` and `new_setting_value` are the `Some` variant. In that
case, we print why we’re not changing `setting_value`, and it doesn’t get
changed.

In all other cases (if either `setting_value` or `new_setting_value` are
`None`) expressed by the `_` pattern in the second arm, we want to allow
`new_setting_value` to become `setting_value`.

We can also use underscores in multiple places within one pattern to ignore
particular values. Listing 18-19 shows an example of ignoring the second and
fourth values in a tuple of five items.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-19/src/main.rs:here}}
```

<span class="caption">範例 18-19: Ignoring multiple parts of a tuple</span>

This code will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be
ignored.

#### Ignoring an Unused Variable by Starting Its Name with `_`

If you create a variable but don’t use it anywhere, Rust will usually issue a
warning because that could be a bug. But sometimes it’s useful to create a
variable you won’t use yet, such as when you’re prototyping or just starting a
project. In this situation, you can tell Rust not to warn you about the unused
variable by starting the name of the variable with an underscore. In Listing
18-20, we create two unused variables, but when we run this code, we should
only get a warning about one of them.

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-20/src/main.rs}}
```

<span class="caption">範例 18-20: Starting a variable name with an
underscore to avoid getting unused variable warnings</span>

Here we get a warning about not using the variable `y`, but we don’t get a
warning about not using the variable preceded by the underscore.

Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. The syntax `_x` still binds the value to the
variable, whereas `_` doesn’t bind at all. To show a case where this
distinction matters, Listing 18-21 will provide us with an error.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-21/src/main.rs:here}}
```

<span class="caption">範例 18-21: An unused variable starting with an
underscore still binds the value, which might take ownership of the value</span>

We’ll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. However, using the underscore by itself
doesn’t ever bind to the value. Listing 18-22 will compile without any errors
because `s` doesn’t get moved into `_`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-22/src/main.rs:here}}
```

<span class="caption">範例 18-22: Using an underscore does not bind the
value</span>

This code works just fine because we never bind `s` to anything; it isn’t moved.

#### Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can use the `..` syntax to use only a few
parts and ignore the rest, avoiding the need to list underscores for each
ignored value. The `..` pattern ignores any parts of a value that we haven’t
explicitly matched in the rest of the pattern. In Listing 18-23, we have a
`Point` struct that holds a coordinate in three-dimensional space. In the
`match` expression, we want to operate only on the `x` coordinate and ignore
the values in the `y` and `z` fields.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-23/src/main.rs:here}}
```

<span class="caption">範例 18-23: Ignoring all fields of a `Point` except
for `x` by using `..`</span>

We list the `x` value and then just include the `..` pattern. This is quicker
than having to list `y: _` and `z: _`, particularly when we’re working with
structs that have lots of fields in situations where only one or two fields are
relevant.

The syntax `..` will expand to as many values as it needs to be. Listing 18-24
shows how to use `..` with a tuple.

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-24/src/main.rs}}
```

<span class="caption">範例 18-24: Matching only the first and last values in
a tuple and ignoring all other values</span>

In this code, the first and last value are matched with `first` and `last`. The
`..` will match and ignore everything in the middle.

However, using `..` must be unambiguous. If it is unclear which values are
intended for matching and which should be ignored, Rust will give us an error.
Listing 18-25 shows an example of using `..` ambiguously, so it will not
compile.

<span class="filename">檔案名稱：src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-25/src/main.rs}}
```

<span class="caption">範例 18-25: An attempt to use `..` in an ambiguous
way</span>

When we compile this example, we get this error:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-25/output.txt}}
```

It’s impossible for Rust to determine how many values in the tuple to ignore
before matching a value with `second` and then how many further values to
ignore thereafter. This code could mean that we want to ignore `2`, bind
`second` to `4`, and then ignore `8`, `16`, and `32`; or that we want to ignore
`2` and `4`, bind `second` to `8`, and then ignore `16` and `32`; and so forth.
The variable name `second` doesn’t mean anything special to Rust, so we get a
compiler error because using `..` in two places like this is ambiguous.

### Extra Conditionals with Match Guards

A *match guard* is an additional `if` condition specified after the pattern in
a `match` arm that must also match, along with the pattern matching, for that
arm to be chosen. Match guards are useful for expressing more complex ideas
than a pattern alone allows.

The condition can use variables created in the pattern. Listing 18-26 shows a
`match` where the first arm has the pattern `Some(x)` and also has a match
guard of `if x < 5`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-26/src/main.rs:here}}
```

<span class="caption">範例 18-26: Adding a match guard to a pattern</span>

This example will print `less than five: 4`. When `num` is compared to the
pattern in the first arm, it matches, because `Some(4)` matches `Some(x)`. Then
the match guard checks whether the value in `x` is less than `5`, and because
it is, the first arm is selected.

If `num` had been `Some(10)` instead, the match guard in the first arm would
have been false because 10 is not less than 5. Rust would then go to the second
arm, which would match because the second arm doesn’t have a match guard and
therefore matches any `Some` variant.

There is no way to express the `if x < 5` condition within a pattern, so the
match guard gives us the ability to express this logic.

In Listing 18-11, we mentioned that we could use match guards to solve our
pattern-shadowing problem. Recall that a new variable was created inside the
pattern in the `match` expression instead of using the variable outside the
`match`. That new variable meant we couldn’t test against the value of the
outer variable. Listing 18-27 shows how we can use a match guard to fix this
problem.

<span class="filename">檔案名稱：src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-27/src/main.rs}}
```

<span class="caption">範例 18-27: Using a match guard to test for equality
with an outer variable</span>

This code will now print `Default case, x = Some(5)`. The pattern in the second
match arm doesn’t introduce a new variable `y` that would shadow the outer `y`,
meaning we can use the outer `y` in the match guard. Instead of specifying the
pattern as `Some(y)`, which would have shadowed the outer `y`, we specify
`Some(n)`. This creates a new variable `n` that doesn’t shadow anything because
there is no `n` variable outside the `match`.

The match guard `if n == y` is not a pattern and therefore doesn’t introduce
new variables. This `y` *is* the outer `y` rather than a new shadowed `y`, and
we can look for a value that has the same value as the outer `y` by comparing
`n` to `y`.

You can also use the *or* operator `|` in a match guard to specify multiple
patterns; the match guard condition will apply to all the patterns. Listing
18-28 shows the precedence of combining a match guard with a pattern that uses
`|`. The important part of this example is that the `if y` match guard applies
to `4`, `5`, *and* `6`, even though it might look like `if y` only applies to
`6`.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-28/src/main.rs:here}}
```

<span class="caption">範例 18-28: Combining multiple patterns with a match
guard</span>

The match condition states that the arm only matches if the value of `x` is
equal to `4`, `5`, or `6` *and* if `y` is `true`. When this code runs, the
pattern of the first arm matches because `x` is `4`, but the match guard `if y`
is false, so the first arm is not chosen. The code moves on to the second arm,
which does match, and this program prints `no`. The reason is that the `if`
condition applies to the whole pattern `4 | 5 | 6`, not only to the last value
`6`. In other words, the precedence of a match guard in relation to a pattern
behaves like this:

```text
(4 | 5 | 6) if y => ...
```

rather than this:

```text
4 | 5 | (6 if y) => ...
```

After running the code, the precedence behavior is evident: if the match guard
were applied only to the final value in the list of values specified using the
`|` operator, the arm would have matched and the program would have printed
`yes`.

### `@` Bindings

The *at* operator (`@`) lets us create a variable that holds a value at the
same time we’re testing that value to see whether it matches a pattern. Listing
18-29 shows an example where we want to test that a `Message::Hello` `id` field
is within the range `3..=7`. But we also want to bind the value to the variable
`id_variable` so we can use it in the code associated with the arm. We could
name this variable `id`, the same as the field, but for this example we’ll use
a different name.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-29/src/main.rs:here}}
```

<span class="caption">範例 18-29: Using `@` to bind to a value in a pattern
while also testing it</span>

This example will print `Found an id in range: 5`. By specifying `id_variable
@` before the range `3..=7`, we’re capturing whatever value matched the range
while also testing that the value matched the range pattern.

In the second arm, where we only have a range specified in the pattern, the code
associated with the arm doesn’t have a variable that contains the actual value
of the `id` field. The `id` field’s value could have been 10, 11, or 12, but
the code that goes with that pattern doesn’t know which it is. The pattern code
isn’t able to use the value from the `id` field, because we haven’t saved the
`id` value in a variable.

In the last arm, where we’ve specified a variable without a range, we do have
the value available to use in the arm’s code in a variable named `id`. The
reason is that we’ve used the struct field shorthand syntax. But we haven’t
applied any test to the value in the `id` field in this arm, as we did with the
first two arms: any value would match this pattern.

Using `@` lets us test a value and save it in a variable within one pattern.

## Summary

Rust’s patterns are very useful in that they help distinguish between different
kinds of data. When used in `match` expressions, Rust ensures your patterns
cover every possible value, or your program won’t compile. Patterns in `let`
statements and function parameters make those constructs more useful, enabling
the destructuring of values into smaller parts at the same time as assigning to
variables. We can create simple or complex patterns to suit our needs.

Next, for the penultimate chapter of the book, we’ll look at some advanced
aspects of a variety of Rust’s features.
