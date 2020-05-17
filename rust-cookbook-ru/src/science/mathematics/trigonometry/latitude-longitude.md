## Расстояние между двумя точками на земной сфере

[![std-badge]][std]

По умолчанию Rust предоставляет математические [методы с плавающей точкой,] такие как тригонометрические функции, квадратный корень, функции преобразования между радианами и градусами и т.д.

В следующем примере вычисляется расстояние в километрах между двумя точками на Земле с помощью [формулы Хаверсина]. Точки задаются парами значений широты и долготы в градусах. Затем метод [`to_radians`] преобразует их в радианы. Метод [`sin`], [`cos`], [`powi`] и [`sqrt`] вычисляют центральный угол. Наконец, можно рассчитать расстояние.

```rust
fn main() {
    let earth_radius_kilometer = 6371.0_f64;
    let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34880_f64);
    let (london_latitude_degrees, london_longitude_degrees) = (51.50853_f64, -0.12574_f64);

    let paris_latitude = paris_latitude_degrees.to_radians();
    let london_latitude = london_latitude_degrees.to_radians();

    let delta_latitude = (paris_latitude_degrees - london_latitude_degrees).to_radians();
    let delta_longitude = (paris_longitude_degrees - london_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + paris_latitude.cos() * london_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
        distance
    );
}
```


[методы с плавающей точкой,]: https://doc.rust-lang.org/std/primitive.f64.html#methods
[`to_radians`]: https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians
[`sin`]: https://doc.rust-lang.org/std/primitive.f64.html#method.sin
[`cos`]: https://doc.rust-lang.org/std/primitive.f64.html#method.cos
[`powi`]: https://doc.rust-lang.org/std/primitive.f64.html#method.powi
[`sqrt`]: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
[формулы Хаверсина]: https://en.wikipedia.org/wiki/Haversine_formula