## Сравнение вектора

[![ndarray-badge]](https://docs.rs/crate/ndarray/*)

Крейт [ndarray](https://docs.rs/crate/ndarray/*) поддерживает несколько способов создания массивов - этот рецепт создаёт [`ndarray::Array`](https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html) массивы из `std::Vec`, используя `from`. Затем он суммирует массивы поэлементно.

Этот рецепт содержит пример поэлементного сравнения двух векторов с плавающей точкой. Числа с плавающей точкой часто хранятся неточно, что затрудняет точное сравнение. Тем не менее, [`assert_abs_diff_eq!`](https://docs.rs/approx/*/approx/macro.assert_abs_diff_eq.html) макрос из крейта [`approx`](https://docs.rs/approx/*/approx/index.html) позволяет удобное, поэлементное сравнение. Для того, чтобы использовать `approx` крейта с `ndarray`, то функция `approx` должна быть добавлена к `ndarray` зависимости в `Cargo.toml`. Например, `ndarray = { version = "0.13", features = ["approx"] }` .

Этот рецепт также содержит дополнительные примеры владения. Здесь `let z = a + b` потребляет `a` и `b`, обновляет переменную `a` результатом, а затем переносит владение в `z`. В качестве альтернативы, `let w = &c + &d` создает новый вектор без использования `c` или `d`, что позволяет их модифицировать позже. См. [Бинарные операторы с двумя массивами](https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#binary-operators-with-two-arrays) для дополнительной информации.

```rust
#[macro_use(assert_abs_diff_eq)]
extern crate approx;
extern crate ndarray;

use ndarray::Array;

fn main() {
  let a = Array::from(vec![1., 2., 3., 4., 5.]);
  let b = Array::from(vec![5., 4., 3., 2., 1.]);
  let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
  let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

  let z = a + b;
  let w =  &c + &d;

  assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

  println!("c = {}", c);
  c[0] = 10.;
  d[1] = 10.;

  assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));

}
```

