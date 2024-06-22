//! This is the submodule for all of the hardware structs and such.

/// The maximum memory for the architecture. Makes it easier to update in various places 
/// in the program.
pub const MAXIMUM_MEMORY: usize = 1024;

#[derive(Copy, Clone, Debug)]
struct CPU {
    registers: Registers,
} 
impl CPU {
    pub fn new() -> Self {
        Self { registers: [Register::new(); 16] }
    }
}

/// This is just an array of registers for the architecture
type Registers = [Register; 16];

/// This is the register type for this architecture. Values will be stored as a Rust u16, and 
/// the struct will have various ways of manipulating what is inside of it.
#[derive(Copy, Clone, Debug)]
struct Register {
    value: u16,
}
impl Register {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

/// Memory struct. Holds the memory for the program. 
#[derive(Copy, Clone, Debug)]
struct Memory {
    memory: [u16; MAXIMUM_MEMORY]
}
impl Memory {
    pub fn new() -> Self {
        Self { memory: [0; MAXIMUM_MEMORY] }
    }
}

