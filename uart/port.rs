pub struct Port<K>{
    kernel: K
}
pub trait Kernel {
    
}

impl <K> Port<K> {
    pub fn new(kernel: K)->Self{

        Self {  
            kernel
        }
    }
}