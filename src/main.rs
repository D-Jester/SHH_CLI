mod entry;
use std::io::Write;
use entry::ServiceInfo;
use crate::entry::prompt;
use crate::entry::read_passwords_from_file;
use crate::entry::delete_password_from_file;
use crate::entry::edit_password_from_file;
//creating clear function 
fn clear(){
    println!("\x1b[2J\x1b[H");
    std::io::stdout().flush().unwrap();

}

fn main(){
    clear();
    let ascii = r#"
   _____ _    _ _    _ 
  / ____| |  | | |  | |
 | (___ | |__| | |__| |
  \___ \|  __  |  __  |
  ____) | |  | | |  | |
 |_____/|_|  |_|_|  |_|
                       
                                                                                                                     
    "#;

    println!("{}",ascii);

    loop{
        println!("Manager Menu:");
        println!("1. Create Service");
        println!("2. List Services");
        println!("3. Find Service");
        println!("4. Edit Service");
        println!("5. Delete Service");
        println!("6. Exit");
        println!();println!();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim(){
            "1" => {
                println!("Creating new service:");
                let entry: ServiceInfo = ServiceInfo::new(
                    prompt("Name of Service:"),
                    prompt("Username:"),
                    prompt("Protection:"),
                );
                println!("Adding...");
                entry.write_to_file();
            }
            "2" => {
                let services = read_passwords_from_file().unwrap_or_else(|err|{
                    eprintln!("Failed to fetch password: {}",err);
                    Vec::new()
                });
                for item in services{
                    println!(
                        "
                        Service: {}
                            -Username: {}
                            -Protection: {}
                        ",
                        item.service,item.username,item.password
                    )
                }
            }
            "3" => {
                println!("Finding Services");
                let services = read_passwords_from_file().unwrap_or_else(|err|{
                    eprintln!("Failed to search for service: {}", err);
                    Vec::new()
                });
                let search: String = prompt("Search for particular service:");
                for item in services{
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "
                            Service: {}
                                -Username: {}
                                -Protection: {}
                            ",
                            item.service,item.username,item.password
                        )
                    }
                }
            }
            "4" => {
                println!("Which service you want to edit");
                let service_name: String = prompt("Enter service:");
                println!("What do you want to modify");
                println!("1. Username");
                println!("2. Password");
                let mut option = String::new();
                std::io::stdin().read_line(&mut option).unwrap();
                let response:&str=if option.as_str() == "1" {
                    let response = edit_password_from_file(&service_name, &option).unwrap_or_else(|err|{
                        eprintln!("No response: {}",err);
                        let msg:&str = "Editing cancelled...";
                        msg
                    });
                    response
                }else{
                    let response = edit_password_from_file(&service_name, &option).unwrap_or_else(|err|{
                        eprintln!("No renponse: {}", err);
                        let msg:&str = "Editing cancelled...";
                        msg
                    });
                    response
                };
                println!("{}",response);
            }
            "5" => {
                println!("Working on Deletion");
                let service_name = prompt("Which service is to be deleted:");
                let result: &str = delete_password_from_file(&service_name).unwrap_or_else(|err|{
                    eprintln!("No reposnse: {}",err);
                    let msg :&str = "Deletion cancelled...";
                    &msg
                });
                println!("{}",&result)
            }
            "6" => {
                println!("Come again!");
                break;
            }
            &_ => todo!("Choice not in option! Choose again.")
        }
    }
}