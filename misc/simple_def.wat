(module
    (func $i (import "host" "print") (param i32))
    (func $f
        i32.const 8
        call $i)
    (func (export "main")
    )
)
