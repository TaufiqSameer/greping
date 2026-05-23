use greping::{search, search_case_insensitive};
use std::error::Error;
use std::fs;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // for i in args{
    //     println!("{i}");
    // }

    // be careful guys, args[0] stores the name of the current file so we are starting from 1th Index
    let arg = Arg::build(&args).unwrap_or_else(|err| {
        eprint!("Problem : {err}");
        process::exit(1)
    });

    match run(arg) {
        Ok(()) => println!("Ran successfully"),
        Err(e) => eprintln!("{}", e),
    }

    // println!("The query is : {query} and filename is : {file}")
    // println!("The file path is {}",arg.file);
    // let content =  match fs::read_to_string(arg.file) {
    //     Ok(data) => data,
    //     Err(err) => {
    //         println!("Error occured while reading {}", err);
    //         return;
    //     }
    // };

    // println!("The content of the files are {}",content);
}

fn run(arg: Arg) -> Result<(), Box<dyn Error>> {
    println!("The file path is {}", arg.file);
    let content = match fs::read_to_string(arg.file) {
        Ok(data) => data,
        Err(err) => {
            println!("Error occured while reading {}", err);
            return Err(Box::new(err));
        }
    };

    let results = if arg.casing {
        search_case_insensitive(&arg.query, &content)
    } else {
        search(&arg.query, &content)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

struct Arg {
    query: String,
    file: String,
    casing: bool,
}
impl Arg {
    fn build(args: &[String]) -> Result<Arg, &'static str> {
        if (args.len() < 3) {
            return Err("ArgumentError : Enter atleast 3 Arguments in the command line");
        }
        Ok(Arg {
            query: args[1].clone(),
            file: args[2].clone(),
            casing : env::var("IGNORE_CASE").is_ok()
        })
    }
}
