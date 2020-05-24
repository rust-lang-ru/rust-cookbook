## Объявление лениво вычисляемой константы

[![lazy_static-badge]][lazy_static] [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns]

Объявляет лениво вычисляемыю константу [`HashMap`]. Катра [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) будет сразу вычислена и сохранена в глобальной статической ссылке.

```rust
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}

fn main() {
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);

    show_access("Jim");
}
```


[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html