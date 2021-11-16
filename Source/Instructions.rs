use crate::Machine;

pub fn Nothing(vm: &mut Machine) -> bool {
    vm.WalkAddress(1);
    false
}

pub fn LoadRegister(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let addr = vm.GetMemory(pc + 2);

    let val = vm.GetMemory(addr);

    vm.SetRegister(reg, val);
    vm.WalkAddress(3);

    true
}

pub fn SaveRegister(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let addr = vm.GetMemory(pc + 1);
    let reg = vm.GetMemory(pc + 2);

    let val = vm.GetRegister(reg);

    vm.SetMemory(addr, val);
    vm.WalkAddress(3);

    true
}

#[allow(dead_code)]
pub fn Move(_vm: &mut Machine) -> bool {
    false
}

pub fn Add(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 + op2);
    vm.WalkAddress(4);

    true
}

pub fn AddAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 + op2);
    vm.WalkAddress(3);

    true
}

pub fn Subtract(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 - op2);
    vm.WalkAddress(4);

    true
}

pub fn SubtractAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 - op2);
    vm.WalkAddress(3);

    true
}

pub fn Multiply(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 * op2);
    vm.WalkAddress(4);

    true
}

pub fn MultiplyAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 * op2);
    vm.WalkAddress(3);

    true
}

pub fn Divide(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 / op2);
    vm.WalkAddress(4);

    true
}

pub fn DivideAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 / op2);
    vm.WalkAddress(3);

    true
}

pub fn Remainder(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 % op2);
    vm.WalkAddress(4);

    true
}

pub fn RemainderAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 % op2);
    vm.WalkAddress(3);

    true
}

#[allow(dead_code)]
pub fn Neg(_vm: &mut Machine) -> bool {
    false
}

pub fn And(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 & op2);
    vm.WalkAddress(4);

    true
}

pub fn AndAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 & op2);
    vm.WalkAddress(3);

    true
}

pub fn Or(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 | op2);
    vm.WalkAddress(4);

    true
}

pub fn OrAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 | op2);
    vm.WalkAddress(3);

    true
}

pub fn Xor(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, op1 ^ op2);
    vm.WalkAddress(4);

    true
}

pub fn XorAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);

    let op1 = vm.GetRegister(reg);
    let op2 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 ^ op2);
    vm.WalkAddress(3);

    true
}

pub fn Not(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, !op1);
    vm.WalkAddress(3);

    true
}

pub fn ShiftLeft(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 << 1);
    vm.WalkAddress(4);

    true
}

pub fn ShiftLeftAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(reg);

    vm.SetRegister(reg, op1 << 1);
    vm.WalkAddress(2);

    true
}

pub fn ShiftRight(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));

    vm.SetRegister(reg, op1 >> 1);
    vm.WalkAddress(4);

    true
}

pub fn ShiftRightAssign(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(reg);

    vm.SetRegister(reg, op1 >> 1);
    vm.WalkAddress(2);

    true
}

pub fn Equals(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, (op1 == op2) as u8);
    vm.WalkAddress(4);

    true
}

pub fn NotEquals(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, (op1 != op2) as u8);
    vm.WalkAddress(4);

    true
}

pub fn LessThan(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, (op1 < op2) as u8);
    vm.WalkAddress(4);

    true
}

pub fn LessEquals(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, (op1 <= op2) as u8);
    vm.WalkAddress(4);

    true
}

pub fn GreaterThan(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, (op1 > op2) as u8);
    vm.WalkAddress(4);

    true
}

pub fn GreaterEquals(vm: &mut Machine) -> bool {
    let pc = vm.GetAddress();

    let reg = vm.GetMemory(pc + 1);
    let op1 = vm.GetRegister(vm.GetMemory(pc + 2));
    let op2 = vm.GetRegister(vm.GetMemory(pc + 3));

    vm.SetRegister(reg, (op1 >= op2) as u8);
    vm.WalkAddress(4);

    true
}

pub fn Halt(_: &mut Machine) -> bool {
    false
}
