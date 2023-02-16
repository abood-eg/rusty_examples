#[derive(Debug)]

struct Adult{
    name:String,
    age:u8
}
impl Adult {
    fn new(age:u8,name:&str)->Result<Self,&str>{
        if age >=21 {
            Ok(Self {
                name:name.to_string(),
                age
            })

        }else{
            Err("u r not old enough!")
        }
    }
}

pub fn main() {
    let child = Adult::new(11,"honna");
    let adult=Adult::new(33, "ehha");
   match adult {
       Ok(c)=>println!("{},{}",c.name,c.age),
       Err(e)=>print!("{}",e),
       
   } 
}
