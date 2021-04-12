use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::fs;


fn main(){


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
        
        //trim returns the trimmed string as a slice,without modifying the original
        let mut commands = input.trim().split(" | ").peekable();
             
        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "exit" => return,
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

                command => { let mut child = Command::new(command)
                        .args(args)
                        .spawn()
                        .unwrap();

                        child.wait().expect("process failed");
                },
                
            }
        }
        


    }
}
