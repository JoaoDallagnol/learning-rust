use std::env;

use todo::Todo;

fn main() {
    let todo = Todo::load_file().expect("Erro while trying to load the file!");
    let args: Vec<String> = env::args().collect();

    // if args.len() > 1 {
    //     let command = &args[1];
    //     match &command[..] {
    //         "list" => todo.list()
    //     }
    // } else {
    //     todo.list();
    // }
}
