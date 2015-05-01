use cpu::registers::{
  Flags, Reg16,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};
use cpu::test::run_test;
use std::num::Wrapping;

fn test_add16_sp_e<F: Fn(Flags) -> bool>(sp: u16, e: i8, check_flags: F) -> bool {
  let cpu = run_test(
    &[0xe8, e as u8],
    |cpu| {
      cpu.regs.write16(Reg16::SP, sp);
    }
  );
  let expected = (Wrapping(sp) + Wrapping(e as i16 as u16)).0;
  cpu.clock_cycles() == 16 &&
    cpu.regs.read16(Reg16::SP) == expected &&
    check_flags(cpu.regs.f)
}

#[quickcheck]
fn test_e8(sp: u16, e: i8) -> bool {
  test_add16_sp_e(sp, e, |_| true)
}

#[test]
fn test_e8_overflow_inc() {
  assert!(test_add16_sp_e(0xffff, 1, |_| true))
}

#[test]
fn test_e8_overflow_dec() {
  assert!(test_add16_sp_e(0x0000, -1, |_| true))
}
