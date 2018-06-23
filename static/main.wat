(module
  (func $i (import "host" "print") (param i32))
  (func (export "main")
    i32.const 42
    call $i))
