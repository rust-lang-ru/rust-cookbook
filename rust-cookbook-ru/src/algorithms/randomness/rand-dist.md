## Генерация случайных числе с заданным распределением

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

По умолчанию, случайные числа имеют [равномерное распределение](https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)). Чтобы иметь возможность генерировать случайные числа с другими распределениями, нам нужно сначала создать нужное распределение и потом генерировать случайные значения по этому распределению посредством метода [`Distribution::sample`](https://docs.rs/rand/*/rand/distributions/trait.Distribution.html#tymethod.sample) с помощью генератора случайных чисел [`rand::Rng`](https://docs.rs/rand/*/rand/trait.Rng.html).

[Документация по доступным распределениям находится здесь](https://docs.rs/rand/*/rand/distributions/index.html). Ниже представлен пример, демонстрирующий использование [`Normal`](https://docs.rs/rand/*/rand/distributions/normal/struct.Normal.html) (нормального или гауссового) распределения.

```
extern crate rand;

use rand::distributions::{Normal, Distribution};

fn main() {
  let mut rng = rand::thread_rng();
  let normal = Normal::new(2.0, 3.0);
  let v = normal.sample(&mut rng);
  println!("{} is from a N(2, 9) distribution", v)
}
```


