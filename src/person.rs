pub struct Person {
        name:String,
        age:u32
    }

impl Person{
    pub fn new(name:&str,age:u32) ->Self{
        Self{
            name:name.to_string(),
            age
        }
    }
    pub fn greet(&self){
        println!("Hello, my name is {} and I'm {} years old",self.name,self.age);
    }
    pub fn have_birthdate(&mut self){
        self.age+=1;
    }
    pub fn get_name(&self)-> &str{
        return &self.name;
    }
}