Push 0
Push 0

label loop
    -- [accumulator, index]
    Get 0
    Get 1
    -- [accumulator, index, accumulator, index]
    Add
    -- [accumulator, index, accumulator + index]
    Set 0
    Pop
    -- [accumulator + index, index]

    -- [accumulator, index]
    Incr
    -- [accumulator, index + 1]

    -- [accumulator, index]
    Get 1
    Push 100
    Sub
    -- [accumulator, index, index - 100]
    JNE loop
Pop

Get 0
Print
Push 10
PrintC
