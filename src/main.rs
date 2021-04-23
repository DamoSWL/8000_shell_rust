use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Stdio,Child};
use std::fs;
use std::vec::Vec;
use std::string::String;
use std::str::SplitWhitespace;


// struct HistoryEntry{
//     args : Vec<String>,
//     background : bool
// }


fn chdir(args : SplitWhitespace){
    let dir = args.peekable().peek().map_or("/",|x|*x);
    let root = Path::new(&(dir));

    if !root.is_dir() {
        println!("{}: No such directory", dir);
    }
    else{
        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
        }
    }
}

fn print_dir(){
    let dir = env::current_dir().unwrap();
    println!("{}",dir.to_str().unwrap());
}

fn list(args : SplitWhitespace, command: &str){
    let mut tmp_args = args.clone();
    if tmp_args.next() == None{
        let dirs =  fs::read_dir(".").unwrap();
        for dir in dirs{
            if let Ok(dir) = dir{
                let filename = dir.file_name().into_string().unwrap();
                println!("{}",filename);
            }

        }
    }
    else{
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .unwrap();

        child.wait().expect("process failed");

    }
}

fn find(args : SplitWhitespace){
    let mut tmp_args = args.clone();
    let mut real_args = args.clone();
    if tmp_args.next() == None{
        println!("invalid command format");
        return;
    }
    else{
        real_args.next();
    }
    if tmp_args.next() == None{
        println!("invalid command format");
        return;
    }
    else{
        real_args.next();
    }

    if tmp_args.next() == None{
        println!("invalid command format");
        return;
    }
    let pattern = real_args.next().unwrap();
    
    let mut filestrs = Vec::new();

    let path = env::current_dir().unwrap();
    let path = path.to_str().unwrap();

    let dirs =  fs::read_dir(path).unwrap();
    for dir in dirs{
        if let Ok(dir) = dir{
            let filename = dir.file_name().into_string().unwrap();

            if pattern.contains("*."){
                let len = pattern.len();
                let new_pattern = &pattern[2 .. len];

                if filename.contains(new_pattern){
                    filestrs.push(filename);
                }

            }else{
                if filename == pattern{
                    filestrs.push(filename);
                }

            } 
        }
    }

    for file_name in filestrs{
        let mut full_path = String::from(path);
        full_path.push_str("/");
        full_path.push_str(&file_name);
        println!("{}",full_path);

    }
}

fn history(his_cmds: &Vec<String>){
    for cmd in his_cmds{
        print!("{}",cmd);
    }
}




fn main(){
    let mut his_commands  = Vec::new();

    match fs::create_dir("/csci-shell/home") {
        Err(_) => (),
        Ok(_) => (),
    }

    let root = Path::new("/csci-shell/home");
    if let Err(e) = env::set_current_dir(&root) {
        println!("{}", e);
    }

    let mut prefix = String::new();
     match env::var("USER") {
         Ok(val) => { prefix = val.clone()},
         Err(_) => (),
     } 

     prefix.push_str("@localhost:");
     

    loop {

        let mut final_prefix = prefix.clone();

        match env::current_dir(){
            Ok(path) => {
                final_prefix.push_str(path.to_str().unwrap());
            },
            Err(_) => (),
        }
        
        final_prefix.push_str("$ ");
        print!("{}",final_prefix);

        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim().len() == 0{
            continue;
        }

        let history_cmd = String::from(&input);
        his_commands.push(history_cmd);

        let mut _service : bool = false;
        let mut _pipeline : bool = false;
       
        if input.contains("&"){
            _service = true;
            input = input.replace("&"," ");

        }

        if input.contains("|"){
            _pipeline = true;
           
        }
 
        if !_pipeline{
            //trim returns the trimmed string as a slice,without modifying the original
            let mut commands = input.trim().split(" | ").peekable();

            while let Some(command) = commands.next()  {

                let mut parts = command.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args = parts;
        
        
                match command {
                    "exit" => {                   
                        return;
                    },
                    "cd"   => {
                        chdir(args);                   
                    },
                    "pwd" => { 
                        print_dir();
                    },

                    "ls" => {           
                        list(args,command);                 
                    },


                    "find" =>{
                        find(args);
                
                    },
                    "history"=> {
                        history(&his_commands);
                    },

                    command => {
                        let child = Command::new(command)
                        .spawn()
                        .unwrap();

                        child.wait_with_output().unwrap();
    
                    },
                }
            }
                    
                
        }
        else{
            let mut commands = input.trim().split(" | ").peekable();
            let mut previous_command = None;
     
            while let Some(command) = commands.next()  {
     
                let mut parts = command.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args = parts;

                let stdin = previous_command
                .map_or(
                    Stdio::inherit(),
                    |output: Child| Stdio::from(output.stdout.unwrap())
                );

                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };

                let output = Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

                match output {
                    Ok(output) => { previous_command = Some(output); },
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    },
                }
            }

            if let Some(mut final_command) = previous_command {
                // block until the final command has finished
                final_command.wait().unwrap();
            }
        }
    }
}

