## Сериализация записей в CSV формат

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

В этом примере показано, как сериализовать Rust кортеж. [`csv::writer`](https://docs.rs/csv/*/csv/struct.Writer.html) поддерживает автоматическую сериализацию Rust типов в CSV записи. Метод [`write_record`](https://docs.rs/csv/*/csv/struct.Writer.html#method.write_record) записывает простую запись, содержащую только строковые данные. Данные с более сложными значениями, такими как числа, числа с плавающей запятой и перечисления используют метод [`serialize`](https://docs.rs/csv/*/csv/struct.Writer.html#method.serialize). Поскольку CSV writer использует внутренний буфер, всегда явно выполняйте [`flush`](https://docs.rs/csv/*/csv/struct.Writer.html#method.flush).

```rust
# #[macro_use]
# extern crate error_chain;
extern crate csv;

use std::io;
#
# error_chain! {
#     foreign_links {
#         CSVError(csv::Error);
#         IOError(std::io::Error);
#    }
# }

fn main() -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}
```


