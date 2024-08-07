use flipvm::op::Instruction::*;
use flipvm::op::{Literal12Bit, Literal7Bit, Nibble, TestOp};
use flipvm::Register::*;

use self::common::{run, SIGHALT};
use crate::common::init_machine;

mod common;

#[test]
fn loop_control() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(5).unwrap()),
        // Start = 2
        Test(A, Zero, TestOp::Neq),
        AddIf(PC, PC, Nibble::new_checked(2).unwrap()),
        Imm(PC, Literal12Bit::new_checked(14).unwrap()),
        AddImmSigned(A, Literal7Bit::from_signed(-1).unwrap()),
        AddImm(B, Literal7Bit::new_checked(1).unwrap()),
        Imm(PC, Literal12Bit::new_checked(2).unwrap()),
        // End = 14
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, B, 5);
}
