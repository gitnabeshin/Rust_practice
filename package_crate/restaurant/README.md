# Package and Crate

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Module tree

* `src/main.rs` and `src/lib.rs` are called `crate roots`.
* modules nest inside on another
* parent, child
* entire module tree is rooted under the implicit module named `crate`

Path
* absolute: `crate::*`
* relative: `self::*`, `super::*`

Use

* bringing into scope with `use`
  * ex. `use crate::front_to_house::hosting` 
* re-exporting with `pub use`

Import External package

* add `Cargo.toml`
* find pagkages at `crates.io`

```
[dependencies]
rand = "0.5"
```

```
use rand::Rng;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 100);
}
```

* merge 2 path

```
use std::io;
use std::io::Write;
```

```
use std::io::{self, Write};
```

* Glob Operator
  * bring all pub items into scope

```
use std::collections::*;
```