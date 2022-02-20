pub enum Instructions {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Halt,
}

pub enum Token {
    // An instruction because, well, it's an instruction.
    Instruction,

    // A label for identifying variables. You can call it an identifier if you want.
    Label,

    // A number.
    Number,

    // Comments start with a ; and must be on a new line
    Comment,

    // Empty for, well: Empty things.
    Empty,
}