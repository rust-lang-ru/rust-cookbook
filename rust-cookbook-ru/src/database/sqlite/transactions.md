## Использование транзакций

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] откроет базу данных `cats.db` из первого рецепта.

Начните транзакцию с помощью [`Connection::transaction`](https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.transaction). Транзакции будут отменены, если они не будут явно закрыты с помощью [`Transaction::commit`](https://docs.rs/rusqlite/*/rusqlite/struct.Transaction.html#method.commit).

В следующем примере цвета добавляются в таблицу, которая имеет ограничение на уникальность для названия цвета. При попытке вставки дублирующего цвета транзакция откатывается.

```rust,no_run
extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("delete from cat_colors", NO_PARAMS)?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;
    tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;

    tx.commit()
}
```


