use std::convert::TryInto;

use tokio::net::{TcpListener, TcpSocket, UdpSocket};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use servertype as subtype;
mod service;
mod servertype;
const BufferLen: usize = 2048;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let exp_Service= service::example::new_service_example();
    let arunner=servertype::ServiceRunner{ ServiceRuntime: Box::new( exp_Service)  };

    

}




