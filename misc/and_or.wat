(module
 (func $print (import "host" "print") (param i32)) 
 (func (export "main")
 i32.const 1
 i32.const 2
 i32.ge_s
 i32.const 2
 i32.const 7
 i32.lt_s
 i32.and
 call $print
 ))
