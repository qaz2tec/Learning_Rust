enum Status {
    Okay,
    NotOk,
}


struct Condition {
    name: String,
    status: Status,

}

fn main() {
    let _student_condition:Condition = Condition{
        name:String::from("Dhruv Sharma"),
        status:Status::Okay
    };
    // println!("{:?}",student_condition);
}
