(module
    (func $print (import "host" "print") (param i32))
    (func $f (param $n i32) (result i32)
        get_local $n
        i32.const 5
        i32.lt_s
        if (result i32)
            i32.const 0
        else
            get_local $n
            i32.const 10
            i32.lt_s
            if (result i32)
                i32.const 1
            else
                i32.const 2
            end
        end)
    (func (export "main")
        i32.const 4
        call $f
        call $print
        i32.const 8
        call $f
        call $print
        i32.const 11
        call $f
        call $print
    )
)
