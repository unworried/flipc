use flipvm::op::{Instruction::*, Literal10Bit, Literal12Bit, Nibble, TestOp};
use flipvm::Machine;
use flipvm::Register::*;

use self::common::{run, SIGHALT};

mod common;

#[test]
fn jump() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(PC, Literal12Bit::new_checked(10).unwrap()),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, PC, 12);
}

#[test]
fn jump_offset() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Add(Zero, Zero, Zero),
        Add(Zero, Zero, Zero),
        Add(Zero, Zero, Zero),
        JumpOffset(Literal10Bit::new_checked(10).unwrap()),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, PC, 18);
}

#[test]
fn branch() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(12).unwrap()),
        Imm(B, Literal12Bit::new_checked(13).unwrap()),
        Test(A, B, TestOp::Neq),
        AddIf(PC, PC, Nibble::new_checked(0x4).unwrap()),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut vm, &program).unwrap();
}

#[test]
fn branch_without_test() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(12).unwrap()),
        Imm(B, Literal12Bit::new_checked(13).unwrap()),
        Test(A, B, TestOp::Neq),
        AddIf(PC, PC, Nibble::new_checked(0x3).unwrap()),
        Invalid(0),
        Invalid(0),
        AddIf(PC, PC, Nibble::new_checked(0xf).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut vm, &program).unwrap();
}

#[test]
fn jump_and_link_set() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(B, Literal12Bit::new_checked(4).unwrap()),
        SetAndSave(PC, B, C),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, C, 2);
}

#[test]
fn jump_and_link_add() {
    let mut vm = Machine::new(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(8).unwrap()),
        AddAndSave(PC, A, B),
        Invalid(0),
        Invalid(0),
        Invalid(0),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut vm, &program).unwrap();
    assert_reg_eq!(vm, B, 2);
}
