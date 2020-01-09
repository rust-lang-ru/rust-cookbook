# Передача данных между двумя потоками

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

Этот пример демонстрирует использование [crossbeam-channel] с одиночным производителем и одиночным потребителем (single producer - single consumer, SPSC).
Мы далее развиваем [ex-crossbeam-spawn] пример, используя [`crossbeam::scope`](https://docs.rs/crossbeam/*/crossbeam/fn.scope.html) и [`Scope::spawn`](https://docs.rs/crossbeam/*/crossbeam/thread/struct.Scope.html#method.spawn) для управления производящим потоком.
Данные передаются между двумя потоками через [`crossbeam_channel::unbounded`](https://docs.rs/crossbeam-channel/*/crossbeam_channel/fn.unbounded.html) канал, то есть в канале может храниться неограниченное количество сообщений (которое фактически ограничено доступным объёмом памяти). Поток-производитель засыпает на полсекунды между посылкой сообщений.

```rust
extern crate crossbeam;
extern crate crossbeam_channel;

use std::{thread, time};
use crossbeam_channel::unbounded;

fn main() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    }).unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}
```


[crossbeam-channel]: https://docs.rs/crate/crossbeam-channel/
[ex-crossbeam-spawn]: concurrency/threads.html#spawn-a-short-lived-thread