## Создание базы данных SQLite

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

Пакет `rusqlite` позволяет работать с базами данных SQLite. Посмотрите [этот пакет], если вам необходима компиляция под Windows.

[`Connection::open`](https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open) создаёт базу данных, если она ещё не существует.

```rust,no_run
extern crate rusqlite;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}
```


[этот пакет]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open