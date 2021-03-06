pub enum Instructions {
    // Move into memory
    MOV,

    // Increment/decrement memory
    INC,
    DEC,

    // Add/remove hex from memory
    ADD,
    SUB,

    // Convert hex to decimal & vice versa
    INT,
    HEX,

    // Logic gates
    ORR,
    EOR,
    AND,
    NOT,
}

pub enum Token {
    // An instruction because, well, it's an instruction.
    Instruction,

    // A register for identifying registers. You can call it an identifier if you want.
    Register,
    Memory,

    // Hex code
    Value,

    // A comment
    Comment,

    // Unknown token
    Unknown,

    // Empty for, well: Empty things.
    Empty,
}
