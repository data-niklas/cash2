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
