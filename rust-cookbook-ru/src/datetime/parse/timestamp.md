## Преобразование даты в UNIX-timestamp и обратно

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Пример преобразует дату, переданную через [`NaiveDate::from_ymd`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html#method.from_ymd) и [`NaiveTime::from_hms`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveTime.html#method.from_hms) во [временную метку UNIX](https://en.wikipedia.org/wiki/Unix_time) (UNIX timestemp) используя метод [`NaiveDateTime::timestamp`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html#method.timestamp).
Затем он вычисляет дату, соответствующую одному миллиарду секунд, прошедших с 1 Января, 1970 UTC при помощи функции [`NaiveDateTime::from_timestamp`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html#method.from_timestamp).

```rust
extern crate chrono;

use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time, date_time.timestamp());

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds);
}
```


