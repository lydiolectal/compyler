use common::*;
use regex::Regex;

lazy_static! {
    static ref PRINT_RE: Regex = Regex::new(r"^called host host.print[(]([^)]*)[)] =>$").unwrap();
}

pub fn compile(text: &str) -> Result<Wexp, Error> {
    let lexer = Lexer::new(&text);
    let tokens = lexer.lex()?;
    let parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let codegenerator = CodeGenerator::new(program);
    Ok(codegenerator.codegen())
}

#[cfg(test)]
mod test {
    use super::*;

    use tempfile::Builder;

    use std::process::Command;
    use std::{fs, str};

    fn run(text: &str) -> Vec<String> {
        println!("compiling program: {}", text);

        let wat = compile(text).expect("Compilation failed").to_string();

        println!("compiled wat: {}", wat);

        let watfile = Builder::new()
            .prefix("compyler-test-input")
            .suffix(".wat")
            .tempfile()
            .expect("Failed to create tempfile");

        fs::write(&watfile, &wat).expect("Failed to write WAT.");

        let wasm = Builder::new()
            .prefix("compyler-test-wasm")
            .suffix(".wasm")
            .tempfile()
            .expect("Failed to create tempfile");

        let wat2wasm_status = Command::new("wat2wasm")
            .arg("-o")
            .arg(wasm.path())
            .arg(watfile.path())
            .status()
            .expect("wat2wasm failed.");

        assert!(wat2wasm_status.success());

        let wasm_interp_output = Command::new("wasm-interp")
            .arg("--host-print")
            .arg("--run-all-exports")
            .arg(wasm.path())
            .output()
            .expect("Failed to execute wasm-interp");

        if !wasm_interp_output.status.success() {
            let stdout = str::from_utf8(&wasm_interp_output.stdout)
                .unwrap()
                .to_string();
            let stderr = str::from_utf8(&wasm_interp_output.stderr)
                .unwrap()
                .to_string();
            println!("{}", stdout);
            println!("{}", stderr);
            panic!();
        }

        let stdout =
            str::from_utf8(&wasm_interp_output.stdout).expect("wasm-interp output was not UTF-8");

        println!("wasm-interp:  {:?}", stdout.trim());

        stdout
            .lines()
            .flat_map(|line| {
                let captures = PRINT_RE.captures(line);
                if let Some(captures) = captures {
                    return Some(captures[1].to_string());
                }

                if line != "main() => " {
                    return None;
                }

                panic!("unexpected line in output: {}", line);
            })
            .collect()
    }

    macro_rules! test {
        (name: $name:ident,input: $input:expr,output: $output:expr,) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected: &[&str] = &$output;
                let actual = run(input);
                if actual != expected {
                    println!("expected:     {}", expected.join(", "));
                    println!("actual:       {}", actual.join(", "));
                    panic!();
                }
            }
        };
    }

    // test! {
    //     name:   empty,
    //     input:  r#"
    //         print 1
    //         if True:
    //             print 7.0
    //         else:
    //             print 8
    //         "#,
    //     output: [[1i32], [7f32],
    // }

    test! {
        name:   empty,
        input:  "",
        output: [],
    }

    test! {
        name:   print_int,
        input:  "print 7",
        output: ["i32:7"],
    }

    test! {
        name:   print_ints,
        input:  "print 7\nprint 1 + 2",
        output: ["i32:7", "i32:3"],
    }

    test! {
        name: print_leq,
        input: "print 8 <= 8",
        output: ["i32:1"],
    }

    test! {
        name: print_div,
        input: "print 9 / 3",
        output: ["i32:3"],
    }

    test! {
        name:   function_call,
        input:  "def f(a, b):\n  return a + b\nprint f(2, 3)",
        output: ["i32:5"],
    }

    test! {
        name: if_else,
        input: "def f(a):\n  if a < 5:\n    return 0\n  else:\n    return 1\nprint f(1)",
        output: ["i32:0"],
    }

    test! {
        name: elif,
        input: "def f(n):\n if n < 5:\n  return 0\n elif n < 10:\n  return 1\n elif n < 15:\
        \n  return 2  \n else:\n  return 3\nprint f(4)\nprint f(8)\nprint f(81)",
        output: ["i32:0", "i32:1", "i32:3"],
    }

    test! {
        name: fib,
        input: "def fib(n):\n  if n < 2:\n    return n\n  else:\n    return fib(n - 2) + fib(n - 1)\nprint fib(4)",
        output: ["i32:3"],
    }
}
