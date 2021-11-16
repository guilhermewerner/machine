use crate::Machine;

pub fn Nothing(vm: &mut Machine) -> bool {
    vm.program += 1;
    false
}

pub fn LoadRegister(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1];
    let a = vm.memory[vm.program + 2];

    vm.registers[r as usize] = vm.memory[a as usize];

    vm.program += 3;

    true
}

pub fn SaveRegister(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1];
    let a = vm.memory[vm.program + 2];

    vm.memory[a as usize] = vm.registers[r as usize];

    vm.program += 3;

    true
}

pub fn Move(vm: &mut Machine) -> bool {
    false
}

pub fn Add(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a + b;
    vm.program += 4;

    true
}

pub fn AddAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] += a;
    vm.program += 3;

    true
}

pub fn Subtract(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a - b;
    vm.program += 4;

    true
}

pub fn SubtractAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] -= a;
    vm.program += 3;

    true
}

pub fn Multiply(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a * b;
    vm.program += 4;

    true
}

pub fn MultiplyAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] *= a;
    vm.program += 3;

    true
}

pub fn Divide(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a / b;
    vm.program += 4;

    true
}

pub fn DivideAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] /= a;
    vm.program += 3;

    true
}

pub fn Remainder(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a % b;
    vm.program += 4;

    true
}

pub fn RemainderAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] %= a;
    vm.program += 3;

    true
}

pub fn Neg(vm: &mut Machine) -> bool {
    false
}

pub fn And(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a & b;
    vm.program += 4;

    true
}

pub fn AndAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] &= a;
    vm.program += 3;

    true
}

pub fn Or(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a | b;
    vm.program += 4;

    true
}

pub fn OrAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] |= a;
    vm.program += 3;

    true
}

pub fn Xor(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a ^ b;
    vm.program += 4;

    true
}

pub fn XorAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] ^= a;
    vm.program += 3;

    true
}

pub fn Not(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] = !a;
    vm.program += 3;

    true
}

pub fn ShiftLeft(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a << b;
    vm.program += 4;

    true
}

pub fn ShiftLeftAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] <<= a;
    vm.program += 3;

    true
}

pub fn ShiftRight(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = a >> b;
    vm.program += 4;

    true
}

pub fn ShiftRightAssign(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];

    vm.registers[r] >>= a;
    vm.program += 3;

    true
}

pub fn Equals(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = (a == b) as u8;
    vm.program += 4;

    true
}

pub fn NotEquals(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = (a != b) as u8;
    vm.program += 4;

    true
}

pub fn LessThan(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = (a < b) as u8;
    vm.program += 4;

    true
}

pub fn LessEquals(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = (a <= b) as u8;
    vm.program += 4;

    true
}

pub fn GreaterThan(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = (a > b) as u8;
    vm.program += 4;

    true
}

pub fn GreaterEquals(vm: &mut Machine) -> bool {
    let r = vm.memory[vm.program + 1] as usize;
    let a = vm.registers[vm.memory[vm.program + 2] as usize];
    let b = vm.registers[vm.memory[vm.program + 3] as usize];

    vm.registers[r] = (a >= b) as u8;
    vm.program += 4;

    true
}

pub fn Halt(vm: &mut Machine) -> bool {
    false
}
