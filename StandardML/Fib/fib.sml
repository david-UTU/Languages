fun fib n =
    let
        fun fib1 1 = (1, 0) 
        | fib1 n =
            let
                val (a, b) = fib1 (n - 1)
            in
                (a+b, a)
            end
    in
        #1 (fib1 n)
    end;