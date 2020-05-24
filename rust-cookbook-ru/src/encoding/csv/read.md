## Чтение CSV записей

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Считывает стандартные CSV записи в [`csv::StringRecord`](https://docs.rs/csv/*/csv/struct.StringRecord.html) - слабо типизированное представление данных, которое ожидает допустимые строки UTF-8. В качестве альтернативы, [`csv::ByteRecord`](https://docs.rs/csv/*/csv/struct.ByteRecord.html) не делает никаких предположений об UTF-8.

```rust
extern crate csv;
use csv::Error;

fn main() -> Result<(), Error> {
    let csv = "year,make,model,description
		1948,Porsche,356,Luxury sports car
		1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

    Ok(())
}
```

Serde десериализует данные в строго типиированные структуры. Смотрите на метод [`csv::Reader::deserialize`](https://docs.rs/csv/*/csv/struct.Reader.html#method.deserialize).

```rust
extern crate csv;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn main() -> Result<(), csv::Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year,
            record.make,
            record.model,
            record.description
        );
    }

    Ok(())
}
```


