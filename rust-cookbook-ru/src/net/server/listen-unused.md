## Прослушивание неиспользуемого порта TCP/IP

[![std-badge]][std] [![cat-net-badge]][cat-net]

В этом примере порт отображается в консоли и программа будет прослушивать, пока на него не будет сделан запрос. `SocketAddrV4` назначает случайный порт при установке порта в 0.

```rust,no_run
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Error};

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}
```
