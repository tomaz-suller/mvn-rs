use std::fmt;

use utils::types::Token;

use crate::types::{Operation, AddressPosition};

use super::MachineAddress;

#[derive(Debug, PartialEq)]
pub struct AddressedLine<'a> {
    pub address: Token<MachineAddress>,
    pub operation: Operation<'a>,
    pub relational_annotation: Option<assembler::types::Line<'a>>,
}

impl<'a> AddressedLine<'a> {
    pub fn new(address: Token<MachineAddress>, operation: Operation<'a>, relational_annotation: Option<assembler::types::Line<'a>>) -> Self{
        Self { address, operation, relational_annotation }
    }

    // FIXME Modify API to get rid of this method
    pub fn destruct(self) -> (assembler::types::Label<'a>, AddressPosition) {
        let annotation = self.relational_annotation.unwrap();
        let label: assembler::types::Label = annotation.operation.operand.value.try_into().unwrap();
        let position: AddressPosition = self.operation.operand.value.try_into().unwrap();
        (label, position)
    }
}

impl fmt::Display for AddressedLine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Operation { instruction, operand } = &self.operation;
        write!(f, "{:X} {:X}{:}", self.address, instruction, operand)?;
        if let Some(annotation) = &self.relational_annotation {
            write!(f, " ; {annotation}")
        } else {
            Ok(())
        }
    }
}