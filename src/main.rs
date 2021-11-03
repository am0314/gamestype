use std::convert::TryInto;

use tokio::net::{TcpListener, TcpSocket, UdpSocket};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BufferLen: usize = 2048;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
type StageResult = Result<(), String>;

pub trait TService {
    fn start(&self) -> StageResult;
    fn handle(&self) -> StageResult;
    fn shotdown(&self);
}

pub struct ServiceRunner {
    // pub ServiceRuntime: Vec<Box<dyn TService>>,
    pub ServiceRuntime: Box<dyn TService>,
}

impl ServiceRunner {
    pub fn run(&self) {
        let start_fn = &self.ServiceRuntime;
        if let Err(e) = start_fn.start() {
            panic!("{:?}", e)
        }

        {
            // let a=||async{}
            async {
                let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
                loop {
                    let (connection, addr) = listener.accept().await.unwrap();

                    tokio::spawn(async move {
                        let mut buffer: [u8; BufferLen];
                        loop {
                            connection.read(&buffer);
                        }
                        // Process each socket concurrently.
                        // process(socket).await
                    });
                }
            };
        }
    }
    //     for c in &mut self.ServiceRuntime.iter() {
    //         match c.start() {
    //             Ok(_) => (),
    //             Err(s) => println!("{}", &s),
    //         };
    //     }
    // }
}

pub struct ServiceMember {}

impl ServiceMember {
    pub fn start(self) {}
}

type HTypeContext = fn();

struct HTypeContextDefer {}
impl HTypeContextDefer {}

trait THTypeContext {
    // fn new(conn: tokio::net::tcp::stream::TcpStream);
    fn Body(&self) -> [u8];
    fn UDPSend(&self) -> SendJobReslut;
    fn TCPSend(&self) -> SendJobReslut;
}

type SendJobReslut = Result<(), String>;
