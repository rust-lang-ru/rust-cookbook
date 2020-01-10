## Преобразование локального времени во время в другом часовом поясе

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Этот пример получает локальное время и отображает его с помощью [`offset::Local::now`](https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now) и затем преобразует его в UTC используя метод [`DateTime::from_utc`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.from_utc). 
Затем время преобразуется посредством структуры [`offset::FixedOffset`](https://docs.rs/chrono/*/chrono/offset/struct.FixedOffset.html) и время из UTC затем преобразуется в UTC+8 и UTC-2.

```rust
extern crate chrono;

use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
}
```


