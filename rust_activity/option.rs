struct Servey{
    q1:Option<i32>,
    q2:Option<String>,
    q3:Option<bool>
}

pub fn main() {
    let mo=Servey{
        q1:Some(29),
        q2:Some("hell is real".to_owned()),
        q3:Some(true)
    };
    let so=Servey{
        q1:None,
        q2:Some("hell is real".to_owned()),
        q3:Some(true)
    };
    let ao=Servey{
        q1:Some(29),
        q2:None,
        q3:Some(true)
    };
    let vector=vec![mo,so,ao];
    for vect in vector{
        match vect.q1{
            Some(age)=>println!("the answer is {} years",age),
            None=>println!("age was not geven for this one")
            
        }
    }
    
}
