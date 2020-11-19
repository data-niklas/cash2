use backend::Runtime;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rt = Runtime::new();
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    /*if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }*/

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 2 {
        println!("Error: Usage is `cash filename` or `cash`");
        return;
    }
    if args.len() == 2 {
        let path = std::fs::canonicalize(std::path::Path::new(&args[1]))
            .expect("Cannot canonicalize path");
        let contents = std::fs::read_to_string(&path).expect("Errored while reading file!");
        let cwd = std::env::current_dir().expect("Cannot find current working directory");
        assert!(
            std::env::set_current_dir(path.parent().expect("Cannot find parent of file")).is_ok()
        );
        match rt.interpret(contents) {
            Ok(tree_result) => {
                if tree_result.get_type_name() == "none" {
                } else {
                    println!("{}", tree_result);
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
        assert!(std::env::set_current_dir(&cwd).is_ok());
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match rt.interpret(line.as_str().to_owned()) {
                    Ok(tree_result) => {
                        if tree_result.get_type_name() == "none" {
                        } else {
                            println!("{}", tree_result);
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    //rl.save_history("history.txt").unwrap();
}
