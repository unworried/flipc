use flipvm::op::Instruction::*;
use flipvm::op::{Literal12Bit, Literal7Bit, Nibble};
use flipvm::Register::*;

use self::common::{init_machine, run, SIGHALT};

mod common;

#[test]
fn add() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(11).unwrap()),
        Imm(B, Literal12Bit::new_checked(15).unwrap()),
        Add(A, B, C),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, 26);
}

#[test]
fn sub() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(20).unwrap()),
        Imm(B, Literal12Bit::new_checked(15).unwrap()),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, 5);
}

#[test]
fn sub_overflow() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(1).unwrap()),
        Imm(B, Literal12Bit::new_checked(57).unwrap()),
        Sub(A, B, C),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, u16::MAX - 55);
}

#[test]
fn add_imm() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(11).unwrap()),
        AddImm(A, Literal7Bit::new_checked(4).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, A, 15);
}

#[test]
fn add_imm_signed() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(21).unwrap()),
        AddImmSigned(A, Literal7Bit::from_signed(-4).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, A, 17);
}

#[test]
fn add_imm_signed_to_zero() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(C, Literal12Bit::new_checked(21).unwrap()),
        AddImmSigned(C, Literal7Bit::from_signed(-21).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, 0);
}

#[test]
fn shift_left() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(C, Literal12Bit::new_checked(0xff).unwrap()),
        ShiftLeft(C, B, Nibble::new_checked(4).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, B, 0xff0);
}

#[test]
fn shift_right_logical() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(B, Literal12Bit::new_checked(0x8fc).unwrap()),
        ShiftLeft(B, B, Nibble::new_checked(4).unwrap()),
        AddImm(B, Literal7Bit::new_checked(0x7).unwrap()),
        // 0x8fc7
        ShiftRightLogical(B, A, Nibble::new_checked(3).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, A, 0x11f8);
}

#[test]
fn shift_right_arithmetic() {
    let mut m = init_machine(1024 * 4);
    let program = vec![
        Imm(A, Literal12Bit::new_checked(0xff0).unwrap()),
        ShiftLeft(A, A, Nibble::new_checked(4).unwrap()),
        AddImm(A, Literal7Bit::new_checked(0x70).unwrap()),
        // 0xff70
        ShiftRightArithmetic(A, C, Nibble::new_checked(2).unwrap()),
        System(Zero, Zero, Nibble::new_checked(SIGHALT).unwrap()),
    ];
    run(&mut m, &program).unwrap();
    assert_reg_eq!(m, C, 0xffdc);
}
