## Норма вектора

[![ndarray-badge]][ndarray]

Этот рецепт демонстрирует использование типа [`Array1`](https://docs.rs/ndarray/*/ndarray/type.Array1.html), типа [`ArrayView1`](https://docs.rs/ndarray/*/ndarray/type.ArrayView1.html), метода [`fold`](https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.fold), метода [`dot`](https://docs.rs/ndarray/*/ndarray/struct.ArrayBase.html#method.dot) при вычислении нормы [l1] и [l2] для данного вектора.

- Функция `l2_norm` является более простой из двух, так как она вычисляет квадратный корень из точечного произведения вектора с самим собой.
- Функция `l1_norm` вычисляется с помощью операции `fold`, которая суммирует абсолютные значения элементов. (Это также может быть выполнено с помощью `x.mapv(f64::abs).scalar_sum()`, но это выделит новый массив для результата `mapv`.)

Обратите внимание, что и `l1_norm` и `l2_norm` принимают аргумент типа [`ArrayView1`](https://docs.rs/ndarray/*/ndarray/type.ArrayView1.html). Этот рецепт учитывает векторные нормы, поэтому функции норм должны принимать только одномерные представления (следовательно, [`ArrayView1`](https://docs.rs/ndarray/*/ndarray/type.ArrayView1.html) ). Хотя функции могут принимать параметр типа `&Array1<f64>`, это потребует от вызывающей стороны иметь ссылку на собственный массив, что является более ограничительным, чем просто доступ к представлению (поскольку представление может быть создано из любого массива или просмотр, а не просто принадлежащий массив).

Типы `Array` и `ArrayView` являются псевдонимами для типов `ArrayBase`. Таким образом, наиболее общий тип аргумента для вызывающей стороны будет `&ArrayBase<S, Ix1> where S: Data`, потому что когда вызывающая сторона может использовать `&array` или `& view` вместо `x.view()`. Если функция является частью публичного API, это может быть лучшим выбором для удобства пользователей. Для внутренних функций предпочтительным может быть более лаконичный `ArrayView1<f64>`.

```rust
#[macro_use(array)]
extern crate ndarray;

use ndarray::{Array1, ArrayView1};

fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

fn main() {
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}
```


[l1]: https://docs.rs/ndarray/*/ndarray/type.Array1.html
[l2]: https://docs.rs/ndarray/*/ndarray/type.ArrayView1.html