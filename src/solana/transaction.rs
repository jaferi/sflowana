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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_transaction_with_expected_fields() {
        let instruction = Instruction {
            program_id: "program1".to_owned(),
            accounts: vec!["account1".to_owned(), "account2".to_owned()],
            data: vec![1, 2, 3],
        };

        let transaction = Transaction {
            signature: "signature1".to_owned(),
            index: 5,
            instructions: vec![instruction],
        };

        assert_eq!(transaction.signature, "signature1");
        assert_eq!(transaction.index, 5);
        assert_eq!(transaction.instructions.len(), 1);
    }

    #[test]
    fn creates_transaction_without_instructions() {
        let transaction = Transaction {
            signature: "signature1".to_owned(),
            index: 0,
            instructions: vec![],
        };

        assert_eq!(transaction.signature, "signature1");
        assert_eq!(transaction.index, 0);
        assert!(transaction.instructions.is_empty());
    }

    #[test]
    fn creates_instruction_with_expected_fields() {
        let instruction = Instruction {
            program_id: "program1".to_owned(),
            accounts: vec!["account1".to_owned(), "account2".to_owned()],
            data: vec![10, 20, 30],
        };

        assert_eq!(instruction.program_id, "program1");
        assert_eq!(
            instruction.accounts,
            vec!["account1".to_owned(), "account2".to_owned()]
        );
        assert_eq!(instruction.data, vec![10, 20, 30]);
    }

    #[test]
    fn creates_instruction_without_accounts_or_data() {
        let instruction = Instruction {
            program_id: "program1".to_owned(),
            accounts: vec![],
            data: vec![],
        };

        assert_eq!(instruction.program_id, "program1");
        assert!(instruction.accounts.is_empty());
        assert!(instruction.data.is_empty());
    }
}
