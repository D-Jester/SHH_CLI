use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io;
use std::io::BufRead;
use std::io::Write;

//direction names
static  DIRECTORY: &str = "passwords.json";
static  DIRECTORY_TEMP: &str = "passwords_temp.json";

#[derive(Debug, Serialize, Deserialize)]
//Struct for storing password details
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    //function to create Service wrapper
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        //reading service name from uer input
        println!("Enter service name:");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line!");

        //reading username from user input
        println!("Enter username:");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line!");

        //reading password from user input
        println!("Enter password:");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line!");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to Serialize to json")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", &self.to_json());
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to File: {}", e);
                } else {
                    println!("Successfully saved!")
                }
            }
            Err(e) => {
                eprintln!("Error opening file : {}", e);
            }
        }
    }
}

//function for reading passwords from file
pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = std::io::BufReader::new(file);
    let mut services: Vec<ServiceInfo> = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }
    Ok(services) //returns vector
}

//function to edit
pub fn edit_password_from_file<'a>(service_name: &'a String, choice: &'a String) -> Result<&'a str,io::Error> {
    let file = File::open("passwords.json")?;
    let reader = std::io::BufReader::new(file);

    let mut value: String = String::new();
    if choice.as_str() == "1" {
        println!("Enter new username:");
    } else {
        println!("Enter new password:");
    }
    std::io::stdin().read_line(&mut value).unwrap();

    //creating temp directory and replacing
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(DIRECTORY_TEMP)
    {
        Ok(mut file) => {
            for line in reader.lines() {
                if let Ok(json_string) = line {
                    if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                        if !(service_info.service.as_str() == service_name.as_str()) {
                            let json_output: String = format!("{}\n", service_info.to_json());
                            if let Err(e) = file.write_all(json_output.as_bytes()) {
                                eprintln!("Can't write: {}", e);
                            }
                        } else {
                            if choice.as_str() == "1" {
                                let edited_user: ServiceInfo = ServiceInfo::new(
                                    service_info.service,
                                    value.clone(),
                                    service_info.password,
                                );
                                let edited_json: String = format!("{}\n", edited_user.to_json());
                                if let Err(e) = file.write_all(&edited_json.as_bytes()) {
                                    eprint!("Can't write new: {}", e);
                                }
                            } else {
                                let edited_user: ServiceInfo = ServiceInfo::new(
                                    service_info.service,
                                    service_info.username,
                                    value.clone(),
                                );
                                let edited_json: String = format!("{}\n", edited_user.to_json());
                                if let Err(e) = file.write_all(&edited_json.as_bytes()) {
                                    eprint!("Can't write new: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
    if let Err(e) = fs::rename(DIRECTORY_TEMP, DIRECTORY) {
        eprintln!("Error replacing file: {}", e);
    }
    let result: &str = "Editing succesfull";
    Ok(&result)
}

//function to delete
pub fn delete_password_from_file(service_name: &String) -> Result<&str, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = std::io::BufReader::new(file);

    //creating a temporary directory and replacing
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open("passwords_temp.json")
    {
        Ok(mut file) => {
            for line in reader.lines() {
                if let Ok(json_string) = line {
                    if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                        if !(service_info.service.as_str() == service_name) {
                            let json_output: String = format!("{}\n", service_info.to_json());
                            if let Err(e) = file.write_all(json_output.as_bytes()) {
                                eprintln!("Can't write: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file :{}", e);
        }
    }
    if let Err(e) = fs::rename("passwords_temp.json", "passwords.json") {
        eprintln!("Error replacing file: {}", e);
    }
    let result: &str = "Deletion Successful";
    Ok(&result)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
