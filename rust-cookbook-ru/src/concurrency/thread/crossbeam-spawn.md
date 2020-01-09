## Запуск короткоживущего потока

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

Этот пример использует крейт [crossbeam], который содержит структуры данных и функции для параллельного и многопоточного программирования. [`Scope::spawn`](https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn) порождает новый  поток со своей областью видимости, который гарантированно прекращается после возврата из области замыкания, переданной в функцию [`crossbeam::scope`](https://docs.rs/crossbeam/*/crossbeam/fn.scope.html). Это означает, что можем безопасно ссылаться на данные из вызванной функции.

Пример делит массив пополам и выполняет работу над его половинами в новых потоках.

```rust
extern crate crossbeam;

fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
  
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
  
    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
  
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
  
        Some(max_l.max(max_r))
    }).unwrap()
}
```


