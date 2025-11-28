pub struct Transaction {
    pub signature: String,
    pub index: usize,
    pub instructions: Vec<Instruction>,
}

pub struct Instruction {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub data: Vec<u8>,
}
