## Генерация случайных числе с заданным распределением

[![rand_distr-badge]][rand_distr] [![cat-science-badge]][cat-science]

По умолчанию, случайные числа из пакета `rand` имеют [непрерывное равномерное распределение (uniform distribution)](https://ru.wikipedia.org/wiki/%D0%9D%D0%B5%D0%BF%D1%80%D0%B5%D1%80%D1%8B%D0%B2%D0%BD%D0%BE%D0%B5_%D1%80%D0%B0%D0%B2%D0%BD%D0%BE%D0%BC%D0%B5%D1%80%D0%BD%D0%BE%D0%B5_%D1%80%D0%B0%D1%81%D0%BF%D1%80%D0%B5%D0%B4%D0%B5%D0%BB%D0%B5%D0%BD%D0%B8%D0%B5). Пакет [`rand_distr`](https://docs.rs/rand_distr/*/rand_distr/index.html) предоставляет другие типы распределений. Для их использования вы инстанциируете распределение, затем элемент из этого распределения используя метод [`Distribution::sample`](https://docs.rs/rand/*/rand/distributions/trait.Distribution.html#tymethod.sample) с помошью генератора случайных чисел [`rand::Rng`](https://docs.rs/rand/*/rand/trait.Rng.html).

Доступные распределения описаны [здесь](https://docs.rs/rand_distr/*/rand_distr/index.html). Пример использования нормального распределения [`Normal`](https://docs.rs/rand_distr/*/rand_distr/struct.Normal.html) показан ниже.

```rust,ignore
extern crate rand;

use rand::distributions::{Normal, Distribution};

fn main() {
  let mut rng = rand::thread_rng();
  let normal = Normal::new(2.0, 3.0);
  let v = normal.sample(&mut rng);
  println!("{} is from a N(2, 9) distribution", v)
}
```


