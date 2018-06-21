use lexer::Lexer;
use parser::Parser;
use wexp::Wexp;
use error::Error;

pub fn compile(text: &str) -> Result<Wexp, Error> {
    let lexer = Lexer::new(&text);
    let tokens = lexer.lex()?;
    let parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    Ok(program.codegen())
}

#[cfg(test)]
mod test {
    use super::*;

    use tempfile::Builder;

    use std::{fs, str};
    use std::process::Command;

    fn run(text: &str) -> String {
        let output = compile(text)
            .expect("Compilation failed")
            .to_string();

        let wat = Builder::new()
            .prefix("compyler-test-input")
            .suffix(".wat")
            .tempfile()
            .expect("Failed to create tempfile");

        fs::write(&wat, &output)
            .expect("Failed to write WAT.");

        let wasm = Builder::new()
            .prefix("compyler-test-wasm")
            .suffix(".wasm")
            .tempfile()
            .expect("Failed to create tempfile");

        let wat2wasm_status = Command::new("wat2wasm")
            .arg("-o")
            .arg(wasm.path())
            .arg(wat.path())
            .status()
            .expect("wat2wasm failed.");

        assert!(wat2wasm_status.success());

        let output = Command::new("wasm-interp")
            .arg("--host-print")
            .arg("--run-all-exports")
            .arg(wasm.path())
            .output()
            .expect("Failed to execute wasm-interp");

        if !output.status.success() {
            let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
            let stderr = str::from_utf8(&output.stderr).unwrap().to_string();
            println!("{}", stdout);
            println!("{}", stderr);
            panic!();
        }

        let stdout = str::from_utf8(&output.stdout)
            .expect("wasm-interp output was not UTF-8");

        stdout[9..stdout.len()-1].to_string()
    }

    macro_rules! test {
        (
            name:   $name:ident,
            input:  $input:expr,
            output: $output:expr,
        ) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = $output;
                let actual = run(input);
                assert_eq!(expected, actual);
            }
        }
    }

    test! {
        name:   empty,
        input:  "",
        output: "",
    }
}
