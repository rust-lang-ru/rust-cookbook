## Генерация случайных чисел из диапазона

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Пример генерирует случайные числа из полуоткрытого диапазона `[0, 10)` (не включающего `10`) с помощью метода [`Rng::gen_range`](https://doc.rust-lang.org/rand/*/rand/trait.Rng.html#method.gen_range).

```rust,ignore
extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
}
```

[`Uniform`](https://docs.rs/rand/*/rand/distributions/uniform/struct.Uniform.html) задаёт [равномерное распределение](https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)). Его использование работает точно так же, но может работать быстрее, когда нам нужно генерировать числа из данного диапазона много раз.

```rust,ignore
extern crate rand;


use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
```


