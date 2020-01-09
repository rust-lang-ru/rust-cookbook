## Поддержание изменяемого глобального состояния

[![lazy_static-badge]][lazy_static] [![cat-rust-patterns-badge]][cat-rust-patterns]

В примере объявляется глобальное состояние используя [lazy_static]. [lazy_static] создаёт объект, доступный глобально через ссылку `static ref`, которая требует [`Mutex`], чтобы сделать этот объект изменяемым (можно также использовать [`RwLock`]).
Обёртка [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) гарантирует, что к этому состоянию не может быть одновременно получен доступ из множества потоков, таким образом предотвращая гонки.
Чтобы прочитать или записать в объект, защищённым [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)-ом, нужно получить специальный объект типа [`MutexGuard`], и осуществлять доступ через него.

```rust
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
#
# error_chain!{ }

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}
```


[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`MutexGuard`]: https://doc.rust-lang.org/std/sync/struct.MutexGuard.html
[`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html