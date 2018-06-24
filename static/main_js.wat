;;  |a|
;;  |b|
;; sub pops a, pops b from stack.
;; computes a - b

(module
  (func $i (import "imports" "print") (param i32))
  (func (export "main")
    i32.const 3
    i32.const 2
    i32.const 2
    i32.sub
    i32.add
    call $i
    i32.const 5
    call $i))
