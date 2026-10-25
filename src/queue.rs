

pub struct Queue{
    elements: Vec<i32>
}

impl Queue {

    pub fn new()-> Self{
        Self {elements : Vec::new(),}
    }

    pub fn enqueue(&mut self, data: i32){
        self.elements.push(data);
    }

    pub fn dequeue(&mut self)-> Result<i32,String>{
        
        if self.is_empty(){
            return Err("queue is empty".to_string())
        }
        
        return Ok(self.elements.remove(0))
    }


    pub fn is_empty(&self)-> bool{
        self.elements.is_empty()
    }

}