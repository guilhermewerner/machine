use crate::Machine;
use crate::Types::{Byte, Word};

pub type RegisterAddress = (Byte, Word);
pub type AddressRegister = (Word, Byte);
pub type Register = Byte;
pub type TwoRegisters = (Byte, Byte);
pub type ThreeRegisters = (Byte, Byte, Byte);
pub type FourRegisters = (Byte, Byte, Byte, Byte);

#[inline]
pub fn GetRegisterAddress(vm: &mut Machine) -> RegisterAddress {
    (vm.ReadByte(None), vm.ReadWord(None))
}

#[inline]
pub fn GetAddressRegister(vm: &mut Machine) -> AddressRegister {
    (vm.ReadWord(None), vm.ReadByte(None))
}

#[inline]
pub fn GetRegister(vm: &mut Machine) -> Register {
    vm.ReadByte(None)
}

#[inline]
pub fn GetTwoRegisters(vm: &mut Machine) -> TwoRegisters {
    (vm.ReadByte(None), vm.ReadByte(None))
}

#[inline]
pub fn GetThreeRegisters(vm: &mut Machine) -> ThreeRegisters {
    (vm.ReadByte(None), vm.ReadByte(None), vm.ReadByte(None))
}

#[inline]
pub fn GetFourRegisters(vm: &mut Machine) -> FourRegisters {
    (
        vm.ReadByte(None),
        vm.ReadByte(None),
        vm.ReadByte(None),
        vm.ReadByte(None),
    )
}