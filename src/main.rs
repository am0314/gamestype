fn main() {
    println!("Hello, world!");
}
type StageResult = Result<(), String>;

pub trait TService {
    fn start(self) -> StageResult;
    fn handle(self) -> StageResult;
    fn shotdown(self);
}

pub struct ServiceContext {
    pub ServiceRuntime: Vec<Box<dyn TService>>,
}

impl ServiceContext {
    pub fn Run(&mut self) {
        for c in &mut self.ServiceRuntime.iter() {
            match c.start() {
                Ok(_) => (),
                Err(s) => println!("{}", &s),
            };
        }
    }
}


pub struct ServiceMember {}

impl ServiceMember {
    pub fn start(self) {}
}
