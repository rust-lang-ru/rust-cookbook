## Создание случайных паролей из букв и цифр

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Пример случайно генерирует строку с заданной длиной из ASCII символов из диапазона `A-Z, a-z, 0-9` с помощью образца [`Alphanumeric`](https://docs.rs/rand/*/rand/distributions/struct.Alphanumeric.html).

```rust,ignore
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}
```


