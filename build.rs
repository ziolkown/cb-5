use rust_bison_skeleton::{process_bison_file, BisonErr};
use std::fs;
use std::path::Path;

fn main() {
    match process_bison_file(Path::new("src/parser/minako_syntax.y")) {
        Ok(_) => {}
        Err(BisonErr { message, .. }) => {
            eprintln!("Bison error:\n{}\nexiting with 1", message);
            std::process::exit(1);
        }
    }
    fix_lexer_lifetime(Path::new("src/parser/minako_syntax.rs"));
    println!("cargo:rerun-if-changed=src/parser/minako_syntax.y");
}

fn fix_lexer_lifetime(filepath: &Path) {
    let content = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(message) => {
            eprintln!(
                "Lexer lifetime fix error:\n{}\nPlease contact the Compilerbau staff.",
                message
            );
            std::process::exit(1);
        }
    };

    let content = content.replace("pub yylexer: Lexer,", "pub yylexer: Lexer<'a>,");
    let content = content.replace("impl Lexer {", "impl<'a> Lexer<'a> {");

    match fs::write(filepath, content) {
        Ok(_) => (),
        Err(message) => {
            eprintln!(
                "Lexer lifetime fix error:\n{}\nPlease contact the Compilerbau staff.",
                message
            );
            std::process::exit(1);
        }
    };
}
