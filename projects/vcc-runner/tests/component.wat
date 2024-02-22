(component
    (import "wasi:random/random@0.2.0" (instance $wasi_random
        (export "get-random-u64" (func (result u64)))
    ))
    (core func $get_random_u64 (canon lower
        (func $wasi_random "get-random-u64")
    ))
    (core module $Library
        (import "wasi:random/random@0.2.0" "get-random-u64" (func $get_random_u64 (result i64)))
        (func $main (export "random-i32")
            (result i32)
            (i32.wrap_i64 (call $get_random_u64))
        )
    )
    (core instance $library (instantiate $Library
        (with "wasi:random/random@0.2.0" (instance (export "get-random-u64" (func $get_random_u64))))
    ))
    (func $random-i32 (export "random-i32")
        (result s32)
        (canon lift (core func $library "random-i32"))
    )
)