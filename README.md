# Hunt down the wierd bug ヾ(＠⌒ー⌒＠)ノ

https://github.com/apache/arrow-datafusion/issues/2910

Running cargo run --release generates the bug below.
```
root@c014a8ed5bdb:/mnt/rust/projects/df_rs_error_find# cargo -V
cargo 1.62.0 (a748cf5a3 2022-06-08)
root@c014a8ed5bdb:/mnt/rust/projects/df_rs_error_find# cargo run --release
warning: unused imports: `ArrayRef`, `Array`, `StringArray`, `UInt8Array`, `common::DataFusionError`
 --> src/main.rs:3:17
  |
3 |         array::{Array, ArrayRef, StringArray, UInt8Array},
  |                 ^^^^^  ^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^
4 |     },
5 |     common::DataFusionError,
  |     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `File`, `collections::HashMap`, `self`
 --> src/main.rs:8:16
  |
8 | use std::{fs::{self, File}, collections::HashMap};
  |                ^^^^  ^^^^   ^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `BufRead`, `BufReader`, `Write`
 --> src/main.rs:9:15
  |
9 | use std::io::{BufRead, BufReader, Write};
  |               ^^^^^^^  ^^^^^^^^^  ^^^^^

warning: unused import: `std::sync::Arc`
  --> src/main.rs:10:5
   |
10 | use std::sync::Arc;
   |     ^^^^^^^^^^^^^^

warning: unused variable: `jpx`
  --> src/main.rs:14:9
   |
14 |     let jpx = "./srnd-itch_20210324_D.csv";
   |         ^^^ help: if this is intentional, prefix it with an underscore: `_jpx`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `jpx_mbo_csv_parquet` (bin "jpx_mbo_csv_parquet") generated 5 warnings
    Finished release [optimized] target(s) in 0.09s
     Running `/mnt/rust/target/release/jpx_mbo_csv_parquet`
+----------------------------------------------------------------------------------------------------------------------------------------------------+
| column_1                                                                                                                                           |
+----------------------------------------------------------------------------------------------------------------------------------------------------+
| 000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000         |
| 0000000000000000                                                                                                                                   |
| 0000000000000000                                                                                                                                   |
| 000                                                                                                                                                |
| 00000                                                                                                                                              |
| 00                                                                                                                                                 |
| x0                                                                                                                                                 |
| 00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000 |
+----------------------------------------------------------------------------------------------------------------------------------------------------+
thread 'tokio-runtime-worker' panicked at 'range end index 154 out of range for slice of length 152', library/core/src/slice/index.rs:73:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ArrowError(ExternalError(Execution("Join Error: task 16 panicked")))', src/main.rs:34:29
``` 