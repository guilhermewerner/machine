use crate::Machine;

pub fn Nothing(vm: &mut Machine) -> bool {
    vm.Next();
    true
}

pub fn LoadRegister(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);
    let addr = vm.ReadWord(None);
    let val = vm.ReadWord(Some(addr));

    vm.registry.Set(reg, val);
    vm.Walk(5);

    true
}

pub fn SaveRegister(vm: &mut Machine) -> bool {
    let addr = vm.ReadWord(None);
    let reg = vm.ReadByte(None);
    let val = vm.registry.Get(reg);

    vm.WriteWord(Some(addr), val);
    vm.Walk(5);

    true
}

#[allow(dead_code)]
pub fn Move(_vm: &mut Machine) -> bool {
    false
}

pub fn Add(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 + op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn AddAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 + op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Subtract(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 - op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn SubtractAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 - op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Multiply(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 * op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn MultiplyAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 * op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Divide(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 / op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn DivideAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 / op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Remainder(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 % op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn RemainderAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 % op2).to_be_bytes());
    vm.Walk(2);

    true
}

#[allow(dead_code)]
pub fn Neg(_vm: &mut Machine) -> bool {
    false
}

pub fn And(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 & op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn AndAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 & op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Or(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 | op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn OrAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 | op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Xor(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, (op1 ^ op2).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn XorAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg1, (op1 ^ op2).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Not(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);
    let op = u32::from_be_bytes(vm.registry.Get(reg));

    vm.registry.Set(reg, (!op).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn ShiftLeft(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));

    vm.registry.Set(reg, (op1 << 1).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn ShiftLeftAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));

    vm.registry.Set(reg1, (op1 << 1).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn ShiftRight(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));

    vm.registry.Set(reg, (op1 >> 1).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn ShiftRightAssign(vm: &mut Machine) -> bool {
    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));

    vm.registry.Set(reg1, (op1 >> 1).to_be_bytes());
    vm.Walk(2);

    true
}

pub fn Equals(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, ((op1 == op2) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn NotEquals(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, ((op1 != op2) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn LessThan(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, ((op1 < op2) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn LessEquals(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, ((op1 <= op2) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn GreaterThan(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, ((op1 > op2) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn GreaterEquals(vm: &mut Machine) -> bool {
    let reg = vm.ReadByte(None);

    let reg1 = vm.ReadByte(None);
    let op1 = u32::from_be_bytes(vm.registry.Get(reg1));
    let reg2 = vm.ReadByte(None);
    let op2 = u32::from_be_bytes(vm.registry.Get(reg2));

    vm.registry.Set(reg, ((op1 >= op2) as u32).to_be_bytes());
    vm.Walk(3);

    true
}

pub fn Halt(_: &mut Machine) -> bool {
    false
}
