## Обработка неверных CSV данных с помощью Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

Файлы CSV часто содержат неверные данные. Для этих случаев крейт `csv` предоставляет специальный десериализатор [`csv::invalid_option`](https://docs.rs/csv/*/csv/fn.invalid_option.html), который автоматически преобразует недопустимые данные в значения None.

```rust
extern crate csv;
use csv::Error;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
```


