(module
  (func $print (import "host" "print") (param i32))
  (func (export "main")
    i32.const 2
    i32.const 3
    i32.lt_s
    call $print))
