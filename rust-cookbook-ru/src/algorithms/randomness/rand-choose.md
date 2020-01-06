## Создание случайных паролей из заданного множества символов

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Пример случайно генерирует строку заданной длины из ASCII символов по заданному пользователем байтовой строки с помощью [`gen_range`](https://docs.rs/rand/*/rand/trait.Rng.html#method.gen_range).

```rust
extern crate rand;

fn main() {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
```


