// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    MaintenanceCrew,
    MarketingDepartment,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

struct Employee {
    name: String,
    employee_type: EmployeeType,
    terminated: bool,
}

fn can_enter_building(employee: &Employee) -> Result<bool, String> {
    if employee.terminated {
        Err("terminated employee can't enter".to_owned())
    }
    else {
        match employee.employee_type {
            EmployeeType::MaintenanceCrew | EmployeeType::MarketingDepartment | EmployeeType::Manager => Ok(true),
            EmployeeType::LineSupervisor | EmployeeType::KitchenStaff | EmployeeType::AssemblyTechnician => Err("invalid employee type".to_owned()),
        }
    }
}

fn print_result(employee: &Employee) -> Result<(), String> {
    let result = can_enter_building(employee)?;
    println!("{:?} can enter the building", employee.name);
    Ok(())
}

fn main() {
    let employees = vec![
        Employee{ name: "Lee".to_owned(), employee_type: EmployeeType::AssemblyTechnician, terminated: false },
        Employee{ name: "Kim".to_owned(), employee_type: EmployeeType::Manager, terminated: true },
        Employee{ name: "Park".to_owned(), employee_type: EmployeeType::MaintenanceCrew, terminated: false },
        Employee{ name: "Ji".to_owned(), employee_type: EmployeeType::LineSupervisor, terminated: false },
        Employee{ name: "Jo".to_owned(), employee_type: EmployeeType::MarketingDepartment, terminated: true },
    ];

    for employee in employees {
        let result = print_result(&employee);
        match result {
            Err(msg) => println!("{:?}: {:?}", employee.name, msg),
            _ => ()
        }
    }
}
