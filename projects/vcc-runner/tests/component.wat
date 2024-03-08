(component $Root
    (import "wasi:io/error@0.2.0" (instance $wasi:io/error@0.2.0
        ;; class std::io::Error
        (export "error" (type (sub resource)))
    ))
    (alias export $wasi:io/error@0.2.0 "error" (type $io-error))

    ;; variant std::io::StreamError
    (type $stream-error (variant
        (case "last-operation-failed" (own $io-error))
        (case "closed")
    ))

    (import "wasi:io/streams@0.2.0" (instance $wasi:io/streams@0.2.0
        (export $output-stream "output-stream" (type (sub resource)))

        (alias outer $Root $io-error (type $io-error))
        (export $stream-error0 "stream-error" (type (eq $stream-error)))
        (export $stream-error1 "alias0" (type (eq $stream-error)))

        (export "[method]output-stream.blocking-write-and-flush"
            ;; class std::io::OutputStream
            ;; method blocking-write-and-flush(self) -> Result<(), std::io::Error>
            (func (param "self" (borrow $output-stream)) (param "contents" (list u8)) (result (result (error $stream-error0))))
        )
    ))
    (alias export $wasi:io/streams@0.2.0 "output-stream" (type $output-stream))
    (alias export $wasi:io/streams@0.2.0 "alias0" (type $output-stream0))

    (import "wasi:cli/stdout@0.2.0" (instance $wasi:cli/stdout@0.2.0
        (alias outer $Root $output-stream (type $output-stream))
        (export "get-stdout"
            ;; class std::io::OutputStream
            ;; static stdout() -> Self
            (func (result (own $output-stream)))
        )
    ))
)
