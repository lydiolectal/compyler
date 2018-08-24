(module
    (func $print (import "host" "print") (param i32))
    (func $f (param $a i32) (param $b i32)
        get_local $a
        ;;local $b i32 ;; hmmmm
        set_local $b i32.const 6
        get_local $b
        i32.sub
        call $print)
    (func (export "main")
    i32.const 3
    i32.const 1
    call $f
    )
)
