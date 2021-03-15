Proc ackermann
    GetArg 1
    -- [m, n | m]
    JE m0

    GetArg 0
    -- [m, n | m, n]
    JE n0

    -- [m, n | m, n]
    Decr
    Call ackermann
    -- [m, n | A(m, n - 1)]
    GetArg 1
    Decr
    -- [m, n | A(m, n - 1), m - 1]
    Get 0
    -- [m, n | A(m, n - 1), m - 1, A(m, n - 1)]
    Call ackermann
    -- [m, n | A(m, n - 1), A(m - 1, A(m, n - 1))]
    Set 0
    Pop
    -- [m, n | A(m - 1, A(m, n - 1))]
    SetArg 1
    Pop
    Pop
    Ret

    label m0
        -- [m, n |]
        GetArg 0
        Incr
        -- [m, n, | n + 1]
        SetArg 1
        Pop
        Pop
        Ret

    label n0
        -- [m, n, | m]
        Push 1
        -- [m, n, | m, 1]
        Get 0
        Decr
        Set 0
        Pop
        -- [m, n, | m - 1, 1]
        Call ackermann
        SetArg 1
        Pop
        Pop
        Ret
End

Push 3
Push 10
Call ackermann
PrintStack
