fun fact n =
    let
        fun fact1 1 answer = answer 
        | fact 1 n answer = fact1 (n - 1) (n * answer)
    in
        fact1 n 1
    end;