## Измерение прошедшего времени между двумя моментами выполнения кода

[![std-badge]][std] [![cat-time-badge]][cat-time]

Пример измеряет количество времени [`time::Instant::elapsed`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed), прошедшее с момента [`time::Instant::now`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now).

Вызов [`time::Instant::elapsed`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed) возвращает [`time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html), который мы выводим на экран в конце примера.
Этот метод не изменяет и не сбрасывает состояние объекта [`time::Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html).

```rust
use std::time::{Duration, Instant};
# use std::thread;
#
# fn expensive_function() {
#     thread::sleep(Duration::from_secs(1));
# }

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
```


