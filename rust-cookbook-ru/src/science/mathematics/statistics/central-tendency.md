### Меры среднего значения

[![std-badge]][std] [![cat-science-badge]][cat-science]

В этих примерах вычисляются показатели среднего значения для набора данных, содержащегося в Rust массиве. Для пустого набора данных не может быть среднего значения, медианы или моды, поэтому каждая функция возвращает [ `Option` ], который должен быть обработан вызывающей стороной.

В первом примере вычисляется среднее значение (сумма всех измерений, деленная на количество измерений в наборе) путем создания итератора ссылок по данным и использования [`sum`] и [`len`] для определения соответственно общего значения и количества значений.

```rust
fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
       positive if positive > 0 => Some(sum  / count as f32),
       _ => None
    };

    println!("Mean of the data is {:?}", mean);
}
```

Во втором примере вычисляется медиана с использованием алгоритма быстрого выбора, который позволяет избежать полного [`sort`] путем сортировки только тех разделов набора данных, о которых известно, что они могут содержать медиану. При этом используются [`cmp`] и [`Ordering`] для краткого выбора следующего исследуемого раздела и [`split_at`] для выбора произвольного центра в следующем разделе на каждом шаге.

```rust
use std::cmp::Ordering;

fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter()
                .fold((vec![], vec![]), |mut splits, next| {
                    {
                        let (ref mut left, ref mut right) = &mut splits;
                        if next < &pivot {
                            left.push(*next);
                        } else {
                            right.push(*next);
                        }
                    }
                    splits
                });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[i32], k: usize) -> Option<i32> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        },
    }
}

fn median(data: &[i32]) -> Option<f32> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None
            }
        },
        odd => select(data, odd / 2).map(|x| x as f32)
    }
}

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let part = partition(&data);
    println!("Partition is {:?}", part);

    let sel = select(&data, 5);
    println!("Selection at ordered index {} is {:?}", 5, sel);

    let med = median(&data);
    println!("Median is {:?}", med);
}
```

В последнем примере вычисляется мода с использованием изменяемого [`HashMap`] для сбора счетчиков каждого отдельного целого числа из набора с использованием API [`fold`] и [`entry`]. Наиболее частое значение в [`HashMap`] всплывает методом [`max_by_key`].

```rust
use std::collections::HashMap;

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is {:?}", mode);
}
```


