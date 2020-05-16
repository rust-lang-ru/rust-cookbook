## Вставка и чтение данных

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Этот рецепт вставляет данные в таблицу `author` используя метод [`execute`] из типа `Client`. Затем выводит данные из той же таблицы `author` используя метод [`query`] снова из `Client`.

```rust,no_run
extern crate postgres;

use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library",
                                    NoTls)?;
    
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string()
        };

        client.execute(
                "INSERT INTO author (name, country) VALUES ($1, $2)",
                &[&author.name, &author.country],
        )?;
    }

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    Ok(())

}
```


[`execute`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.execute
[`query`]: https://docs.rs/postgres/0.17.2/postgres/struct.Client.html#method.query