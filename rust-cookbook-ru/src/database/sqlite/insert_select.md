## Выборка и вставка данных

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`](https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open) откроет базу данных `cats`, созданную в прошлом рецепте. Этот рецепт создаёт таблицы `cat_colors` и `cats` с помощью метода [`execute`](https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.execute) из `Connection`. Сначала данные вставляются в таблицу `cat_colors`. После того как запись о цвете вставилась, метод [`last_insert_rowid`](https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.last_insert_rowid) из `Connection` используется для получения `id` последней вставленной записи цвета. Этот `id` используется при вставке данных в таблицу `cats`. Затем подготавливается запрос на выборку с помощью метода [`prepare`](https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.prepare), который возвращает структуру [`statement`](https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html). Затем выполняется запрос,  используя метод [`query_map`](https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html#method.query_map) из [`statement`](https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html).

```rust,no_run
extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::collections::HashMap;

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}
```


