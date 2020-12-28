## 對無法復原的錯誤使用 `panic!`

有時候壞事就是會發生在你的程式中，這本來就是你沒辦法全部避免的。在這種情況，Rust 有提供 `panic!` 巨集。當 `panic!` 巨集執行時，你的程式就會印出程式出錯的訊息，展開並清理堆疊，然後離開程式。這常用來處理當程式遇到某種錯誤時，開發者不清楚如何處理該錯誤的狀況。

> ### 恐慌時該解開堆疊還是直接終止
>
> 當恐慌（panic）發生時，程式預設會開始做**解開**（unwind）堆疊的動作，這代表 Rust 會回溯整個堆疊，並清理每個它遇到的函式資料。但是這樣回溯並清理的動作很花力氣。另一種方式是直接**終止**（abort）程式而不清理，程式使用的記憶體會需要由作業系統來清理。如果你需要你的專案產生的二進制檔案越小越好，你可以從解開切換成終止，只要在 *Cargo.toml* 檔案中的 `[profile]` 段落加上 `panic = 'abort'` 就好。舉例來說，如果你希望在發佈模式（release mode）恐慌時直接終止，那就加上：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

讓我們先在小程式內試試呼叫 `panic!`：

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

當你執行程式時，你會看到像這樣的結果：

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

`panic!` 的呼叫導致印出了最後兩行的錯誤訊息。第一行顯示了我們的恐慌訊息以及該恐慌是在原始碼何處發生的：*src/main.rs:2:5* 指的是它發生在我們的 *src/main.rs* 檔案第二行第五個字元。

在此例中，該行指的就是我們寫的程式碼。如果我們查看該行，我們會看到 `panic!` 巨集的呼叫。在其他情形，`panic!` 的呼叫可能會發生在我們呼叫的程式碼內，所以錯誤訊息回報的檔案名稱與行數可能就會是其他人呼叫 `panic!` 巨集的程式碼，而不是因為我們的程式碼才導致 `panic!` 的呼叫。我們可以在呼叫 `panic!` 程式碼的地方使用 backtrace 來找出出現問題的地方。接下來我們就會深入瞭解 backtrace。

### 使用 `panic!` Backtrace

讓我們看看另一個例子，這是函式庫發生錯誤而呼叫 `panic!`，而不是來自於我們在程式碼自己呼叫的巨集。範例 9-1 是個嘗試從向量取得元素的例子。

<span class="filename">檔案名稱：src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">範例 9-1：嘗試取得超出向量長度的元素，進而導致 `panic!` 被呼叫</span>

我們在這邊嘗試取得向量中第 100 個元素（不過因為索引從零開始，所以是索引 99），但是它只有 3 個元素。在此情況下，Rust 就會恐慌。使用 `[]` 會回傳元素，但是如果你傳遞了無效的索引，Rust 就回傳不了正確的元素。

在 C 中，嘗試讀取資料結構結束之後的元素屬於未定義行為。你可能會得到該記憶體位置對應其資料結構的元素，即使該記憶體完全不屬於該資料結構。這就稱做**緩衝區過讀**（buffer overread）而且會導致安全漏洞。攻擊者可能故意操縱該索引來取得在資料結構後面他們原本不應該讀寫的值。

為了保護你的程式免於這樣的漏洞，如果你嘗試用一個不存在的索引讀取元素的話，Rust 會停止執行並拒絕繼續運作下去。讓我們嘗試執行並看看會如何：

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

此錯誤指向了一個不是我們寫的檔案 *libcore/slice/mod.rs*。這是實作 `slice` 的 Rust 原始碼。當我們在我們的 `v` 使用 `[]` 時就會執行 *libcore/slice/mod.rs* 內的程式碼，而這正是 `panic!` 實際發生的地方。

下一行提示告訴我們可以設置 `RUST_BACKTRACE` 環境變數來取得 backtrace 以知道錯誤發生時到底發生什麼事。*backtrace* 是一個函式列表，指出得到此錯誤時到底依序呼叫了哪些函式。Rust 的 backtraces 運作方式和其他語言一樣：讀取 backtrace 關鍵是從最一開始讀取直到你看到你寫的檔案。那就會是問題發生的源頭。你寫的程式碼以上的行數就是你所呼叫的程式，而以下則是其他呼叫你的程式碼的程式。這些行數可能還會包含 Rust 核心程式碼、標準函式庫程式碼，或是你所使用的 crate。我們設置 `RUST_BACKTRACE` 環境變數的值不為 0，來嘗試取得 backtrace 吧。你應該會看到和範例 9-2 類似的結果。

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::ArgumentV1::show_usize
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:193
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:471
  11: rust_begin_unwind
             at src/libstd/panicking.rs:375
  12: core::panicking::panic_fmt
             at src/libcore/panicking.rs:84
  13: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:62
  14: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806
  15: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2657
  16: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/liballoc/vec.rs:1871
  17: panic::main
             at src/main.rs:4
  18: std::rt::lang_start::{{closure}}
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  19: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:52
  20: std::panicking::try::do_call
             at src/libstd/panicking.rs:292
  21: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:78
  22: std::panicking::try
             at src/libstd/panicking.rs:270
  23: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  24: std::rt::lang_start_internal
             at src/libstd/rt.rs:51
  25: std::rt::lang_start
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  26: panic::main
```

<span class="caption">範例 9-2：當 `RUST_BACKTRACE` 設置時，透過呼叫 `panic!` 產生的 backtrace</span>

輸出結果有點多啊！你看到的實際輸出可能會因你的作業系統與 Rust 版本而有所不同。要取得這些資訊的 backtrace，除錯符號（debug symbols）必須啟用。當我們在使用 `cargo build` 或 `cargo run` 且沒有加上 `--release` 時，除錯符號預設是啟用的。

在範例 9-2 的輸出結果中，第 17 行的 backtrace 指向了我們專案中產生問題的地方：*src/main.rs* 中的第四行。如果我們不想讓程式恐慌，我們就要來調查我們所寫的程式中第一個被錯誤訊息指向的位置。在範例 9-1 中，我們為了顯示如何使用 backtrace，故意寫出會恐慌的程式碼。要修正的方法就是不要向只有 3 個元素的向量要求取得索引 99 的值。當在未來你的程式碼恐慌時，你會需要知道是程式碼中的什麼動作造成的、什麼數值導致恐慌以及正確的程式碼該怎麼處理。

我們會在本章節[「要 `panic!` 還是不要 `panic!`」][to-panic-or-not-to-panic]<!-- ignore -->的段落中再回來看 `panic!` 並研究何時該與不該使用 `panic!` 來處理錯誤條件。接下來，我們要看如何使用 `Result`來處理可回復的錯誤。

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic

> - translators: [Ngô͘ Io̍k-ūi <wusyong9104@gmail.com>]
> - commit: [e5ed971](https://github.com/rust-lang/book/blob/e5ed97128302d5fa45dbac0e64426bc7649a558c/src/ch09-01-unrecoverable-errors-with-panic.md)
> - updated: 2020-09-11
