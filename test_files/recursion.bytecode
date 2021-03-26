Proc factorial
    JE retOne

    -- [x]
    GetArg 0
    Push 1
    Sub
    -- [x, x - 1]
    Call factorial
    -- [x, factorial(x - 1)]                                           
    Mul
    Ret

    label retOne
        Push 1
        Ret

End

Push 10
Call factorial

Get 0
Print
Push 10
PrintC
