## Параллельный map-reduce

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

Этот пример использует методы [`rayon::filter`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.filter), [`rayon::map`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.map) и [`rayon::reduce`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.reduce) для вычисления среднего возраста объектов `Person`, у которых возраст больше 30.

Метод [`rayon::filter`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.filter) возвращает элементы из коллекции, которые удовлетворяют заданному предикату.
Метод [`rayon::map`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.map) выполняет заданную операцию над каждым элементом, тем самым создавая новый параллельный итератор.
Наконец, метод [`rayon::reduce`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.reduce) выполняет операцию свёртки, то есть некоторую операцию над предыдущей итерацией свёртки и текущим элементом.
Пример также демонстрирует использование метода [`rayon::sum`](https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.sum), который даёт тот же самый результат, что и операция свёртки с переданной функцией сложения и нулём в качестве начального значения.

```rust
extern crate rayon;

use rayon::prelude::*;

struct Person {
    age: u32,
}

fn main() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32/ num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}
```


