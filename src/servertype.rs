

use tokio::net::{TcpListener};

use tokio::io::{AsyncReadExt};

type StageResult = Result<(), String>;

pub trait TService {
    fn start(&self) -> StageResult;
    fn handle(&self) -> StageResult;
    fn shotdown(&self) -> StageResult;
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
            let _ = async {
                let listener = TcpListener::bind("0.0.0.0:5959").await.unwrap();
                loop {
                    let (mut connection, addr) = listener.accept().await.unwrap();

                    tokio::spawn(async move {
                        let mut buffer: Vec<u8> = Vec::new();
                        loop {
                            let _ = async {
                                match connection.read(&mut buffer[..]).await {
                                    Ok(_) => |msglen| {
                                        let res = &buffer[..msglen];
                                        println!("{:?}", &res)
                                    },

                                    Err(_) => todo!(),
                                }
                            };
                        }
                    });
                }
            };
        }
    }
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
