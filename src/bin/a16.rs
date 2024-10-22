// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct LockerInfo {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let locker_info_list = vec![
        LockerInfo {name: "jimmy".to_owned(), locker_assignment: Some(1)},
        LockerInfo {name: "lucy".to_owned(), locker_assignment: Some(2)},
        LockerInfo {name: "jack".to_owned(), locker_assignment: None},
        LockerInfo {name: "argo".to_owned(), locker_assignment: Some(3)},
        LockerInfo {name: "mario".to_owned(), locker_assignment: Some(4)},
    ];

    for locker_info in locker_info_list {
        match locker_info.locker_assignment {
            Some(assignment) => println!("name: {:?}, locker: {:?}", locker_info.name, assignment),
            None => println!("name: {:?}, locker: not assign", locker_info.name),
        }
    }
}
