## Вывести тип MIME по имени файла

[![mime-badge]][mime] [![cat-encoding-badge]][cat-encoding]

Следующий пример показывает, как получить корректный MIME тип из имени файла с помощью крейта [mime]. Программа возьмёт расширение файла и сопоставит его с заранее известным списком. Возвращаемое значение имеет тип [`mime:Mime`](https://docs.rs/mime/*/mime/struct.Mime.html).

```rust
extern crate mime;
use mime::Mime;

fn find_mimetype (filename : &String) -> Mime{

    let parts : Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
            Some(v) =>
                match *v {
                    "png" => mime::IMAGE_PNG,
                    "jpg" => mime::IMAGE_JPEG,
                    "json" => mime::APPLICATION_JSON,
                    &_ => mime::TEXT_PLAIN,
                },
            None => mime::TEXT_PLAIN,
        };
    return res;
}

fn main() {
    let filenames = vec!("foobar.jpg", "foo.bar", "foobar.png");
    for file in filenames {
	    let mime = find_mimetype(&file.to_owned());
	 	println!("MIME for {}: {}", file, mime);
	 }

}
```


