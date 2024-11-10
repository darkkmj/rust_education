// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.
use std::fs;
use std::env;
use std::str;
use std::io::BufRead;
use std::collections::HashMap;
use std::ops::Add;
use structopt::StructOpt;

const DATA_FILE_PATH: &str = "./src/bin/p2_data.csv";
const DATA_TEMP_FILE_PATH: &str = "./src/bin/p2_data_temp.csv";

#[derive(Debug, StructOpt)]
#[structopt(name="contact_manager", about="Contact Manager CLI tool")]
struct Opt {
    #[structopt(long="cmd", default_value="load")]
    cmd: String,
    #[structopt(long="id", default_value="0")]
    id: u32,
    #[structopt(long="name", default_value="")]
    name: String,
    #[structopt(long="email", default_value="")]
    email: String,
}

#[derive(Debug, Clone)]
struct ContactInfo {
    name: String,
    email: String,
}

struct ContactInfoList {
    contact_list: HashMap<u32, ContactInfo>
}

impl ContactInfoList {
    fn new() -> Self {
        Self {
            contact_list: HashMap::new()
        }
    }

    fn add(&mut self, _id: u32, _name: &str, _email: &str) {
        if self.contact_list.contains_key(&_id) {
            println!("id({}) is already contained", _id)
        }
        else {
            self.contact_list.insert(_id, ContactInfo { name: _name.to_owned(), email: _email.to_owned() });
        }
    }

    fn print(self) {
        for contact_info in self.contact_list {
            println!("id: {}, {:?}", contact_info.0, contact_info.1)
        }
    }

    fn load(&mut self) {
        match fs::read(DATA_FILE_PATH) {
            Ok(_file_data) => {
                let mut _line_data = str::from_utf8(&*_file_data).unwrap().split("\r\n");
                let mut _contact_lists = _line_data.for_each(|line| {
                    let mut token_list = line.split(",");
                    let _id = token_list.next().unwrap();
                    let _name = token_list.next().unwrap_or("");
                    let _email = token_list.next().unwrap_or("");

                    match _id.parse::<u32>() {
                        Ok(_id_num) => {
                            self.add(_id_num, _name, _email);
                            // self.contact_list.insert(_id_num, ContactInfo { name: _name.to_string(), email: _email.to_string() });
                        },
                        _ => {}
                    }
                });
            }
            Err(_err) => {
                println!("{:?}", _err);
                let _path = env::current_dir();
                println!("current path is {}. check path of a file", _path.unwrap().display());
            }
        }
    }

    fn search(&self, _id: u32) -> Option<ContactInfo> {
        if self.contact_list.contains_key(&_id) {
            println!("id({_id}) is in contained ({:?})", self.contact_list[&_id]);
            self.contact_list[&_id].clone().into()
        }
        else {
            println!("id({_id}) is not contained");
            None
        }
    }

    fn save(self) -> std::io::Result<()> {
        let mut data_to_write = "id,_name,email\\r\\n".to_owned();
        self.contact_list.iter().for_each(|(_id, _contact_info)| {
            data_to_write.push_str(format!("{},{},{}\r\n", _id, _contact_info.name, _contact_info.email).as_str());
        });
        fs::write(DATA_TEMP_FILE_PATH, data_to_write)
    }
}


fn main() {
    // Create contact info list
    let mut contact_info_list = ContactInfoList::new();

    // Load data from CSV file
    contact_info_list.load();

    let opt: Opt = Opt::from_args();
    // println!("{:?}", opt);
    match opt.cmd.as_str() {
        "print" => contact_info_list.print(),
        "search" => {
            contact_info_list.search(opt.id);
        }
        _ => {}
    }
}
