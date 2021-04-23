use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs;
use std::vec::Vec;
use std::string::String;

// struct HistoryEntry{
//     args : Vec<String>,
//     background : bool
// }

fn main(){
    let mut history  = Vec::new();

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
        history.push(history_cmd);
        //trim returns the trimmed string as a slice,without modifying the original
        let mut commands = input.trim().split(" | ").peekable();

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let mut args = parts;

          
            match command {
                "exit" => {
                    
                    return;
                },
                "cd"   => {

                    
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
                   
                },
                "pwd" => { 
                    
                    let dir = env::current_dir().unwrap();
                    println!("{}",dir.to_str().unwrap());

                },

                "ls" => { let mut tmp_args = args.clone();
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


                },


                "find" =>{
                    let mut tmp_args = args.clone();
                   if tmp_args.next() == None{
                       println!("invalid command format");
                       continue;
                   }
                   else{
                       args.next();
                   }
                   if tmp_args.next() == None{
                        println!("invalid command format");
                        continue;
                   }
                   else{
                       args.next();
                   }

                   if tmp_args.next() == None{
                        println!("invalid command format");
                        continue;
                    }
                   let pattern = args.next().unwrap();
                   
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

                },
                "history"=> {
                    for cmd in &history{
                        print!("{}",cmd);
                    }
                },

//create a new commmand
//use stdio::piped() to connect
                command => { let child = Command::new(command)
                    .stdin(Stdio::piped())
                    .spawn()
                    .unwrap();


                        //child.wait().expect("process failed");
                    let output = child.wait_with_output().unwrap();

                    println!("{}", String::from_utf8_lossy(&output.stdout));
                },
                
            }
        }
        // if let Some(mut final_command) = child {
        //     // block until the final command has finished
        //     final_command.wait().unwrap();
        // }
    }
}

