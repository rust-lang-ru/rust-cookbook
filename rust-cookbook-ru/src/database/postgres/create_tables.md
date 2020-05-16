## Создание таблиц в Postgres

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Используйте пакет [`postgres`](https://docs.rs/postgres/0.15.2/postgres/) для создания таблиц в Postgres.

С помощью [`Client::connect`] можно присоединиться к существующей базе данных. Этот рецепт использует строку подключения в формате URL для передачи в `Client::connect`. Эта функция предполагает, что база данных существует, названа `library`, имя пользователя `postgres` и пароль `postgres`.

```rust,no_run
extern crate postgres;

 use postgres::{Client, NoTls, Error};

 fn main() -> Result<(), Error> {
     let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;
     
     client.batch_execute("
         CREATE TABLE IF NOT EXISTS author (
             id              SERIAL PRIMARY KEY,
             name            VARCHAR NOT NULL,
             country         VARCHAR NOT NULL
             )
     ")?;

     client.batch_execute("
         CREATE TABLE IF NOT EXISTS book  (
             id              SERIAL PRIMARY KEY,
             title           VARCHAR NOT NULL,
             author_id       INTEGER NOT NULL REFERENCES author
             )
     ")?;

     Ok(())
 }
```


[`Client::connect`]: https://docs.rs/postgres/0.17.2/postgres/
