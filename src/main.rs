struct Person{
    name : String,
    age : u8
}

struct Person2{
    name: String,
    age: u8,
}

trait HasVoiceBox {
    fn speak(&self);

    fn cant_speak(&self) -> bool;
}

impl HasVoiceBox for Person2{
    fn speak(&self) {
        println!("Hello,my name is {}", self.name);
    }
    fn cant_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and i am {} years old.", self.name,self.age)
    }
}
fn main() {

    let ak =  Person{
        name : String::from("Akash"),
        age :23,
    };

    let bc = Person {
        name :String::from("Akshay"),
        age:25,
    };
    println!("{} \n {} ",ak.to_string(),bc.to_string());

        let person = Person2 {
        name: String::from("Jack"),
        age: 0,
    };

    println!("can {} speak? \n {}", person.name, person.cant_speak());
    person.speak();


}

