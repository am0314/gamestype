pub(crate) mod service_example;



pub fn new_service_example()->Example{
    return  Example{};
}
pub struct Example{

}

impl Example {
    fn start(&self) -> Result<(), String>{
        Ok(())
    }
    fn handle(&self) -> Result<(), String>{
        Ok(())

    }
    fn shotdown(&self)->Result<(), String>{
        Ok(())

    }
}

pub fn newone(){}