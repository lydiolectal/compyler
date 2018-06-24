(module
  (func $i (import "host" "print") (param i32 f32))
  (func (export "main")
    i32.const 1
    i32.const 3
    i32.add
    f32.const 7
    call $i
    ))
