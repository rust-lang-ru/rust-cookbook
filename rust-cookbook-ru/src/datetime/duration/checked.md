## Проверяемое вычисление даты и времени

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Код в примере вычисляет и отображает дату и время через 2 недели от текущего момента времени используя [`DateTime::checked_add_signed`](https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_add_signed) и дату за сутки до этого дня используя [`DateTime::checked_sub_signed`](https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_sub_signed). Эти методы возвращают None, если дата и время не могут быть вычислены.

Доступные управляющие последовательности для [`DateTime::format`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format) могут быть найдены в [`chrono::format::strftime`](https://docs.rs/chrono/*/chrono/format/strftime/index.html).

```rust
extern crate chrono;
use chrono::{DateTime, Duration, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn main() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2))
            .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
            .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}
```


