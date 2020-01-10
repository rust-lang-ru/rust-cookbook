## Форматированный вывод даты и времени

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Пример получает и отображает текущее время в UTC используя метод [`Utc::now`](https://docs.rs/chrono/*/chrono/offset/struct.Utc.html#method.now). Форматирует текущее время в распространённых форматах [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) посредством [`DateTime::to_rfc2822`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc2822), [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) посредством [`DateTime::to_rfc3339`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.to_rfc3339), и пользовательском формате используя [`DateTime::format`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format).

```rust
extern crate chrono;
use chrono::{DateTime, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}
```


