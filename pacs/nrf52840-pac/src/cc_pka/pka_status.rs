#[doc = "Register `PKA_STATUS` reader"]
pub type R = crate::R<PkaStatusSpec>;
#[doc = "Field `ALU_MSB_4BITS` reader - The most significant 4-bits of the operand updated in shift operation."]
pub type AluMsb4bitsR = crate::FieldReader;
#[doc = "Field `ALU_LSB_4BITS` reader - The least significant 4-bits of the operand updated in shift operation."]
pub type AluLsb4bitsR = crate::FieldReader;
#[doc = "Field `ALU_SIGN_OUT` reader - Indicates the MSB sign of the last operation."]
pub type AluSignOutR = crate::BitReader;
#[doc = "Field `ALU_CARRY` reader - Holds the carry of the last ALU operation."]
pub type AluCarryR = crate::BitReader;
#[doc = "Field `ALU_CARRY_MOD` reader - Holds the carry of the last modular operation."]
pub type AluCarryModR = crate::BitReader;
#[doc = "Field `ALU_SUB_IS_ZERO` reader - Indicates the last subtraction operation sign."]
pub type AluSubIsZeroR = crate::BitReader;
#[doc = "Field `ALU_OUT_ZERO` reader - Indicates if the result of ALU OUT is zero."]
pub type AluOutZeroR = crate::BitReader;
#[doc = "Field `ALU_MODOVRFLW` reader - Modular overflow flag."]
pub type AluModovrflwR = crate::BitReader;
#[doc = "Field `DIV_BY_ZERO` reader - Indication if the division is done by zero."]
pub type DivByZeroR = crate::BitReader;
#[doc = "Field `MODINV_OF_ZERO` reader - Indicates the modular inverse of zero."]
pub type ModinvOfZeroR = crate::BitReader;
#[doc = "Field `OPCODE` reader - Opcode of the last operation"]
pub type OpcodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - The most significant 4-bits of the operand updated in shift operation."]
    #[inline(always)]
    pub fn alu_msb_4bits(&self) -> AluMsb4bitsR {
        AluMsb4bitsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The least significant 4-bits of the operand updated in shift operation."]
    #[inline(always)]
    pub fn alu_lsb_4bits(&self) -> AluLsb4bitsR {
        AluLsb4bitsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Indicates the MSB sign of the last operation."]
    #[inline(always)]
    pub fn alu_sign_out(&self) -> AluSignOutR {
        AluSignOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Holds the carry of the last ALU operation."]
    #[inline(always)]
    pub fn alu_carry(&self) -> AluCarryR {
        AluCarryR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Holds the carry of the last modular operation."]
    #[inline(always)]
    pub fn alu_carry_mod(&self) -> AluCarryModR {
        AluCarryModR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates the last subtraction operation sign."]
    #[inline(always)]
    pub fn alu_sub_is_zero(&self) -> AluSubIsZeroR {
        AluSubIsZeroR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates if the result of ALU OUT is zero."]
    #[inline(always)]
    pub fn alu_out_zero(&self) -> AluOutZeroR {
        AluOutZeroR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Modular overflow flag."]
    #[inline(always)]
    pub fn alu_modovrflw(&self) -> AluModovrflwR {
        AluModovrflwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indication if the division is done by zero."]
    #[inline(always)]
    pub fn div_by_zero(&self) -> DivByZeroR {
        DivByZeroR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates the modular inverse of zero."]
    #[inline(always)]
    pub fn modinv_of_zero(&self) -> ModinvOfZeroR {
        ModinvOfZeroR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Opcode of the last operation"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "This register holds the status for the PKA pipeline.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaStatusSpec;
impl crate::RegisterSpec for PkaStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_status::R`](R) reader structure"]
impl crate::Readable for PkaStatusSpec {}
#[doc = "`reset()` method sets PKA_STATUS to value 0x1000"]
impl crate::Resettable for PkaStatusSpec {
    const RESET_VALUE: u32 = 0x1000;
}
