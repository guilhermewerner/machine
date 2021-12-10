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

pub fn Neg(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);

    let a = i32::from_be_bytes(vm.registry.Get(payload.1));

    vm.registry.Set(payload.0, (-a).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Add(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a + b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Subtract(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a - b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Multiply(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a * b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Divide(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a * b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Remainder(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a * b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn And(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a & b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Or(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (a | b).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Xor(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

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

pub fn Nand(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (!(a & b)).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Nor(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (!(a | b)).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Xnor(vm: &mut Machine) -> bool {
    let payload = Payload::GetThreeRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));
    let b = u32::from_be_bytes(vm.registry.Get(payload.2));

    vm.registry.Set(payload.0, (!(a ^ b)).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn ShiftLeft(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);

    let a = u32::from_be_bytes(vm.registry.Get(payload.1));

    vm.registry.Set(payload.0, (a << 1).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn ShiftRight(vm: &mut Machine) -> bool {
    let payload = Payload::GetTwoRegisters(vm);

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
