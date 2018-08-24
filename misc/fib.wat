(module
    (func $print (import "host" "print") (param i32))
    (func $fib (param $n i32) (result i32)
        get_local $n
        i32.const 2
        i32.lt_s
        ;; result specifies return type of if/else
        if (result i32)
            get_local $n
        else
            get_local $n
            i32.const 2
            i32.sub
            call $fib

            get_local $n
            i32.const 1
            i32.sub
            call $fib

            i32.add
        end)
    (func (export "main")
        i32.const 10
        call $fib
        call $print
    )
)
