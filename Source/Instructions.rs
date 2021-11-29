use crate::Payload::{ThreeRegisters, TwoRegisters};
use crate::{Machine, Payload};

pub fn Nothing(vm: &mut Machine) -> bool {
    vm.Next();
    true
}

pub fn LoadRegister(vm: &mut Machine) -> bool {
    let (reg, addr) = Payload::GetRegisterAddress(vm);

    let data = vm.ReadWord(Some(addr));

    vm.registry.Set(reg, data);
    vm.Walk(5);

    true
}

pub fn StoreRegister(vm: &mut Machine) -> bool {
    let (addr, reg) = Payload::GetAddressRegister(vm);

    let data = vm.registry.Get(reg);

    vm.WriteWord(Some(addr), data);
    vm.Walk(5);

    true
}

pub fn IncrementRegister(vm: &mut Machine) -> bool {
    let r0 = Payload::GetRegister(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r0));

    vm.registry.Set(r0, (a + 1).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn DecrementRegister(vm: &mut Machine) -> bool {
    let r0 = Payload::GetRegister(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r0));

    vm.registry.Set(r0, (a - 1).to_be_bytes());
    vm.Walk(2);

    true
}

#[allow(dead_code)]
pub fn Move(_vm: &mut Machine) -> bool {
    false
}

pub fn Add(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Add(vm, payload)
}

pub fn AddAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Add(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Add(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a + b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Subtract(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Subtract(vm, payload)
}

pub fn SubtractAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Subtract(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Subtract(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a - b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Multiply(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Multiply(vm, payload)
}

pub fn MultiplyAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Multiply(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Multiply(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a * b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Divide(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Divide(vm, payload)
}

pub fn DivideAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Divide(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Divide(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a * b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Remainder(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Remainder(vm, payload)
}

pub fn RemainderAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Remainder(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Remainder(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a * b).to_be_bytes());
    vm.Walk(3);

    true
}

#[allow(dead_code)]
pub fn Neg(_vm: &mut Machine) -> bool {
    false
}

pub fn And(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _And(vm, payload)
}

pub fn AndAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _And(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _And(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a & b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Or(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Or(vm, payload)
}

pub fn OrAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Or(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Or(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a | b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Xor(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);
    _Xor(vm, payload)
}

pub fn XorAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _Xor(vm, (payload.0, payload.0, payload.1))
}

#[inline]
fn _Xor(vm: &mut Machine, payload: ThreeRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a ^ b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Not(vm: &mut Machine) -> bool {
    let r0 = Payload::GetRegister(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r0));

    vm.registry.Set(r0, (!a).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn ShiftLeft(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _ShiftLeft(vm, payload)
}

pub fn ShiftLeftAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetRegister(vm);
    _ShiftLeft(vm, (payload, payload))
}

#[inline]
fn _ShiftLeft(vm: &mut Machine, payload: TwoRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));

    vm.registry.Set(payload.0, (a << 1).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn ShiftRight(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);
    _ShiftRight(vm, payload)
}

pub fn ShiftRightAssign(vm: &mut Machine) -> bool {
    let payload = Payload::GetRegister(vm);
    _ShiftRight(vm, (payload, payload))
}

#[inline]
fn _ShiftRight(vm: &mut Machine, payload: TwoRegisters) -> bool {
    let a = u32::from_be_bytes(vm.registry.Get(payload.1));

    vm.registry.Set(payload.0, (a >> 1).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Equals(vm: &mut Machine) -> bool {
    let (r0, r1, r2) = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r1));
    let b = u32::from_be_bytes(vm.registry.Get(r2));

    vm.registry.Set(r0, ((a == b) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn NotEquals(vm: &mut Machine) -> bool {
    let (r0, r1, r2) = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r1));
    let b = u32::from_be_bytes(vm.registry.Get(r2));

    vm.registry.Set(r0, ((a != b) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn LessThan(vm: &mut Machine) -> bool {
    let (r0, r1, r2) = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r1));
    let b = u32::from_be_bytes(vm.registry.Get(r2));

    vm.registry.Set(r0, ((a < b) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn LessEquals(vm: &mut Machine) -> bool {
    let (r0, r1, r2) = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r1));
    let b = u32::from_be_bytes(vm.registry.Get(r2));

    vm.registry.Set(r0, ((a <= b) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn GreaterThan(vm: &mut Machine) -> bool {
    let (r0, r1, r2) = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r1));
    let b = u32::from_be_bytes(vm.registry.Get(r2));

    vm.registry.Set(r0, ((a > b) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn GreaterEquals(vm: &mut Machine) -> bool {
    let (r0, r1, r2) = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(r1));
    let b = u32::from_be_bytes(vm.registry.Get(r2));

    vm.registry.Set(r0, ((a >= b) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Halt(_: &mut Machine) -> bool {
    false
}
