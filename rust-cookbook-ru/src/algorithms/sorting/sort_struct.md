## Сортировка вектора структур

[![std-badge]][std] [![cat-science-badge]][cat-science]

Пример сортирует вектор персон (то есть структур типа Person) с полями `name` и `age` согласно естественному порядку (по имени и возрасту). Чтобы сделать структуру Person сортируемой, нам нужно реализовать четыре типажа: [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html),
[`PartialEq`], [`Ord`] and [`PartialOrd`]. Эти типажи могут быть просто реализованы автоматически с помощью аннотации `derive`.
Мы можем также передать нашу функцию сравнения используя метод [`vec:sort_by`] и отсортировать только по возрасту.

```rust
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort people by derived natural order (Name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);

    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);

}
```


[`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html 
[`Ord`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[`vec:sort_by`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html