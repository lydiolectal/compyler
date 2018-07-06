(module
  (func $i32.print (import "host" "print") (param i32))
  (func $f32.print (import "host" "print") (param f32))
  (func $foo (param i32) i32.const 7 call $bar)
  (func $bar (param i32))
  (func (export "main")
    i32.const 1
    i32.const 3
    i32.add
    f32.const 7
    call $f32.print
    call $foo
    ))
