use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::fs;

fn main(){


    match fs::create_dir("/csci-shell/home") {
        Err(why) => println!("/csci-shell/home {:?}", why.kind()),
        Ok(_) => {},
    }

    let root = Path::new("/csci-shell/home");
        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
        }

    loop {

        // let key = "PATH";
        // match env::var(key) {    
        //     Ok(val) => {
        //     //   println!("val =>{}",val);
        //     },
        //     Err(e) => println!("couldn't interpret {}: {}", key, e),
        // }

        print!("user@localhost:/csci-shell/home$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();


        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

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
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
 
            final_command.wait().unwrap();
        }

    }
}
