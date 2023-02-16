struct Student{
    name:String,
    locker_assignment:Option<i32>

}

pub fn main() {

let vector_student =vec![ Student{
        name:"ahmed".to_owned(),
        locker_assignment:Some(103030),
    },
    Student{
        name:"umar".to_owned(),
        locker_assignment:Some(92934)
    },
    Student{
        name:"abooh".to_owned(),
        locker_assignment:None
    }
];
for student in vector_student{
    match student.locker_assignment{
        Some(num)=>println!("the student name is {}, his locker_assignment is {}",student.name,num),
        None=>println!("the student {} has no locker_assignment",student.name)
    }
}
 
}
