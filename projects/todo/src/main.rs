use std::env;

use todo::Todo;

fn main() {
    let todo = Todo::new();
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
