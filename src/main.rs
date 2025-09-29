

mod person;
use person::Person;

fn main() {
    

    let mut  person: Person = Person::new("massil",21);
    person.greet();
    person.have_birthdate();
    person.greet();
    println!("{}",person.get_name());
    
   
}
   