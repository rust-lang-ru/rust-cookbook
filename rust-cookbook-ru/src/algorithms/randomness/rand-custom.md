## Генерация случайных чисел пользовательского типа

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Пример случайно генерирует пару `(i32, bool, f64)` и переменную пользовательского типа `Point`.
Реализует типаж [`Distribution`](https://docs.rs/rand/*/rand/distributions/trait.Distribution.html) от Point для особенного типа [`Standard`](https://docs.rs/rand/*/rand/distributions/struct.Standard.html) для того, чтобы включить возможность случайной генерации значений Point.

```rust,ignore
extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
```


