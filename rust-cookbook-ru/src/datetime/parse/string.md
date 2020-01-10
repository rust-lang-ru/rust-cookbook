## Разбор строки в структуру DateTime

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Код совершает разбор в структуру [`DateTime`](https://docs.rs/chrono/*/chrono/struct.DateTime.html) строк, представляющих распространённые форматы [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt), [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) и пользовательский формат.
Разбор использует функции [`DateTime::parse_from_rfc2822`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_rfc2822), [`DateTime::parse_from_rfc3339`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_rfc3339), и
[`DateTime::parse_from_str`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_str) соответственно.

Управляющие последовательности, которые доступны для [`DateTime::parse_from_str`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_str), могут быть найдены в документации к [`chrono::format::strftime`](https://docs.rs/chrono/*/chrono/format/strftime/index.html).
Заметим, что функция [`DateTime::parse_from_str`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.parse_from_str) требует, что такая структура DateTime должна иметь возможность быть созданной так, что она однозначно определяет дату и время. Для парсинга дат и времён без временных зон мы используем [`NaiveDate`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveDate.html), [`NaiveTime`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveTime.html) и [`NaiveDateTime`](https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html).

```rust
extern crate chrono;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() -> Result<(), ParseError> {
    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    Ok(())
}
```


