## Агрегирование данных

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Этот рецепт выводит в порядке убывания национальности первых 7999 художников из базы данных ["Museum of Modern Art"].

```rust,no_run
extern crate postgres;
use postgres::{Client, Error, NoTls};

struct Nation {
    nationality: String,
    count: i64,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@127.0.0.1/moma",
        NoTls,
    )?;

    for row in client.query
	("SELECT nationality, COUNT(nationality) AS count
	FROM artists GROUP BY nationality ORDER BY count DESC", &[])? {
        
        let (nationality, count) : (Option<String>, Option<i64>)
		= (row.get (0), row.get (1));
        
        if nationality.is_some () && count.is_some () {

            let nation = Nation{
                nationality: nationality.unwrap(),
                count: count.unwrap(),
        };
            println!("{} {}", nation.nationality, nation.count);
            
        }
    }

    Ok(())
}
```


["Museum of Modern Art"]: https://github.com/MuseumofModernArt/collection/blob/master/Artists.csv