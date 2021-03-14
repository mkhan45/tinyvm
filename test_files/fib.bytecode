Push 0
Push 1
Push 1
-- [i, a, b]

Proc fibStep
    GetArg 0
    GetArg 1
    -- [a, b, | b, a]
    Add
    -- [a, b, | b + a]
    GetArg 0
    -- [a, b, | b + a, b]
    SetArg 1
    Pop
    -- [b, b, | b + a]
    SetArg 0
    Pop
    -- [b, b + a | ]
    Ret
End


label loop
    -- [i, a, b]
    -- print b \n
    Print
    Push 10
    PrintC
    Pop
    
    Call fibStep
    -- [i, a, b]
    Get 0
    Push 1
    -- [i, a, b, i, 1]
    Add
    -- [i, a, b, i + 1]
    Set 0
    Pop
    -- [i + 1, a, b]
    -- [i, a, b]
    Get 0
    Push 40
    Sub
    -- [i, a, b, i - 40]
    JNE loop
    Pop
-- [i, a, b]
