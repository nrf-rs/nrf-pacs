#[doc = "Register `OPCODE` reader"]
pub type R = crate::R<OpcodeSpec>;
#[doc = "Register `OPCODE` writer"]
pub type W = crate::W<OpcodeSpec>;
#[doc = "Field `TAG` reader - Holds the operation tag or the operand C virtual register index."]
pub type TagR = crate::FieldReader;
#[doc = "Field `TAG` writer - Holds the operation tag or the operand C virtual register index."]
pub type TagW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REG_R` reader - Result register virtual register index."]
pub type RegRR = crate::FieldReader;
#[doc = "Field `REG_R` writer - Result register virtual register index."]
pub type RegRW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "This field controls the interpretation of REG_R.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscardR {
    #[doc = "0: REG_R is intepreted as a register index."]
    Register = 0,
    #[doc = "1: Result is discarded."]
    Discard = 1,
}
impl From<DiscardR> for bool {
    #[inline(always)]
    fn from(variant: DiscardR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCARD_R` reader - This field controls the interpretation of REG_R."]
pub type DiscardRR = crate::BitReader<DiscardR>;
impl DiscardRR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DiscardR {
        match self.bits {
            false => DiscardR::Register,
            true => DiscardR::Discard,
        }
    }
    #[doc = "REG_R is intepreted as a register index."]
    #[inline(always)]
    pub fn is_register(&self) -> bool {
        *self == DiscardR::Register
    }
    #[doc = "Result is discarded."]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == DiscardR::Discard
    }
}
#[doc = "Field `DISCARD_R` writer - This field controls the interpretation of REG_R."]
pub type DiscardRW<'a, REG> = crate::BitWriter<'a, REG, DiscardR>;
impl<'a, REG> DiscardRW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REG_R is intepreted as a register index."]
    #[inline(always)]
    pub fn register(self) -> &'a mut crate::W<REG> {
        self.variant(DiscardR::Register)
    }
    #[doc = "Result is discarded."]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(DiscardR::Discard)
    }
}
#[doc = "Field `REG_B` reader - Operand B virtual register index."]
pub type RegBR = crate::FieldReader;
#[doc = "Field `REG_B` writer - Operand B virtual register index."]
pub type RegBW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "This field controls the interpretation of REG_B.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstB {
    #[doc = "0: REG_B is intepreted as a register index."]
    Register = 0,
    #[doc = "1: REG_B is intepreted as a constant."]
    Constant = 1,
}
impl From<ConstB> for bool {
    #[inline(always)]
    fn from(variant: ConstB) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONST_B` reader - This field controls the interpretation of REG_B."]
pub type ConstBR = crate::BitReader<ConstB>;
impl ConstBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConstB {
        match self.bits {
            false => ConstB::Register,
            true => ConstB::Constant,
        }
    }
    #[doc = "REG_B is intepreted as a register index."]
    #[inline(always)]
    pub fn is_register(&self) -> bool {
        *self == ConstB::Register
    }
    #[doc = "REG_B is intepreted as a constant."]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == ConstB::Constant
    }
}
#[doc = "Field `CONST_B` writer - This field controls the interpretation of REG_B."]
pub type ConstBW<'a, REG> = crate::BitWriter<'a, REG, ConstB>;
impl<'a, REG> ConstBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REG_B is intepreted as a register index."]
    #[inline(always)]
    pub fn register(self) -> &'a mut crate::W<REG> {
        self.variant(ConstB::Register)
    }
    #[doc = "REG_B is intepreted as a constant."]
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(ConstB::Constant)
    }
}
#[doc = "Field `REG_A` reader - Operand A virtual register index."]
pub type RegAR = crate::FieldReader;
#[doc = "Field `REG_A` writer - Operand A virtual register index."]
pub type RegAW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "This field controls the interpretation of REG_A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstA {
    #[doc = "0: REG_A is intepreted as a register index."]
    Register = 0,
    #[doc = "1: REG_A is intepreted as a constant."]
    Constant = 1,
}
impl From<ConstA> for bool {
    #[inline(always)]
    fn from(variant: ConstA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONST_A` reader - This field controls the interpretation of REG_A."]
pub type ConstAR = crate::BitReader<ConstA>;
impl ConstAR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConstA {
        match self.bits {
            false => ConstA::Register,
            true => ConstA::Constant,
        }
    }
    #[doc = "REG_A is intepreted as a register index."]
    #[inline(always)]
    pub fn is_register(&self) -> bool {
        *self == ConstA::Register
    }
    #[doc = "REG_A is intepreted as a constant."]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == ConstA::Constant
    }
}
#[doc = "Field `CONST_A` writer - This field controls the interpretation of REG_A."]
pub type ConstAW<'a, REG> = crate::BitWriter<'a, REG, ConstA>;
impl<'a, REG> ConstAW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REG_A is intepreted as a register index."]
    #[inline(always)]
    pub fn register(self) -> &'a mut crate::W<REG> {
        self.variant(ConstA::Register)
    }
    #[doc = "REG_A is intepreted as a constant."]
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(ConstA::Constant)
    }
}
#[doc = "Field `LEN` reader - The length of the operands. This value serves as an PKA length register index. E.g.: if LEN field value is set to 0, PKA_L\\[0\\] holds the size of the operands."]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - The length of the operands. This value serves as an PKA length register index. E.g.: if LEN field value is set to 0, PKA_L\\[0\\] holds the size of the operands."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Operation code to be executed by the PKA engine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    #[doc = "0: Terminate operation"]
    Terminate = 0,
    #[doc = "4: Add or Increment"]
    AddInc = 4,
    #[doc = "5: Subtract, Decrement, or Negate"]
    SubDecNeg = 5,
    #[doc = "6: Modular Add or Modular Increment"]
    ModAddInc = 6,
    #[doc = "7: Modular Subtract, Modular Decrement, or Modular Negate"]
    ModSubDecNeg = 7,
    #[doc = "8: Perform AND, test, or clear"]
    Andtst0clr0 = 8,
    #[doc = "9: Perform OR, copy, or set bits"]
    Orcopyset0 = 9,
    #[doc = "10: Perform XOR, flip bits, invert, or compare"]
    Xorflp0invcmp = 10,
    #[doc = "12: Shift right 0 operation"]
    Shr0 = 12,
    #[doc = "13: Shift right 1 operation"]
    Shr1 = 13,
    #[doc = "14: Shift left 0 operation"]
    Shl0 = 14,
    #[doc = "15: Shift left 1 operation"]
    Shl1 = 15,
    #[doc = "16: Multiply low operation"]
    MulLow = 16,
    #[doc = "17: Modular multiply operation"]
    ModMul = 17,
    #[doc = "18: Modular multiply N operation"]
    ModMulN = 18,
    #[doc = "19: Modular exponentiation operation"]
    ModExp = 19,
    #[doc = "20: Division operation"]
    Division = 20,
    #[doc = "21: Modular inversion operation"]
    ModInv = 21,
    #[doc = "22: Modular division operation"]
    ModDiv = 22,
    #[doc = "23: Multiply high operation"]
    MulHigh = 23,
    #[doc = "24: Modular multiplication acceleration"]
    ModMlac = 24,
    #[doc = "25: Modular multiplication acceleration where final reduction is omitted"]
    ModMlacnr = 25,
    #[doc = "27: Reduction operation"]
    Reduction = 27,
}
impl From<Opcode> for u8 {
    #[inline(always)]
    fn from(variant: Opcode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opcode {
    type Ux = u8;
}
impl crate::IsEnum for Opcode {}
#[doc = "Field `OPCODE` reader - Operation code to be executed by the PKA engine"]
pub type OpcodeR = crate::FieldReader<Opcode>;
impl OpcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opcode> {
        match self.bits {
            0 => Some(Opcode::Terminate),
            4 => Some(Opcode::AddInc),
            5 => Some(Opcode::SubDecNeg),
            6 => Some(Opcode::ModAddInc),
            7 => Some(Opcode::ModSubDecNeg),
            8 => Some(Opcode::Andtst0clr0),
            9 => Some(Opcode::Orcopyset0),
            10 => Some(Opcode::Xorflp0invcmp),
            12 => Some(Opcode::Shr0),
            13 => Some(Opcode::Shr1),
            14 => Some(Opcode::Shl0),
            15 => Some(Opcode::Shl1),
            16 => Some(Opcode::MulLow),
            17 => Some(Opcode::ModMul),
            18 => Some(Opcode::ModMulN),
            19 => Some(Opcode::ModExp),
            20 => Some(Opcode::Division),
            21 => Some(Opcode::ModInv),
            22 => Some(Opcode::ModDiv),
            23 => Some(Opcode::MulHigh),
            24 => Some(Opcode::ModMlac),
            25 => Some(Opcode::ModMlacnr),
            27 => Some(Opcode::Reduction),
            _ => None,
        }
    }
    #[doc = "Terminate operation"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == Opcode::Terminate
    }
    #[doc = "Add or Increment"]
    #[inline(always)]
    pub fn is_add_inc(&self) -> bool {
        *self == Opcode::AddInc
    }
    #[doc = "Subtract, Decrement, or Negate"]
    #[inline(always)]
    pub fn is_sub_dec_neg(&self) -> bool {
        *self == Opcode::SubDecNeg
    }
    #[doc = "Modular Add or Modular Increment"]
    #[inline(always)]
    pub fn is_mod_add_inc(&self) -> bool {
        *self == Opcode::ModAddInc
    }
    #[doc = "Modular Subtract, Modular Decrement, or Modular Negate"]
    #[inline(always)]
    pub fn is_mod_sub_dec_neg(&self) -> bool {
        *self == Opcode::ModSubDecNeg
    }
    #[doc = "Perform AND, test, or clear"]
    #[inline(always)]
    pub fn is_andtst0clr0(&self) -> bool {
        *self == Opcode::Andtst0clr0
    }
    #[doc = "Perform OR, copy, or set bits"]
    #[inline(always)]
    pub fn is_orcopyset0(&self) -> bool {
        *self == Opcode::Orcopyset0
    }
    #[doc = "Perform XOR, flip bits, invert, or compare"]
    #[inline(always)]
    pub fn is_xorflp0invcmp(&self) -> bool {
        *self == Opcode::Xorflp0invcmp
    }
    #[doc = "Shift right 0 operation"]
    #[inline(always)]
    pub fn is_shr0(&self) -> bool {
        *self == Opcode::Shr0
    }
    #[doc = "Shift right 1 operation"]
    #[inline(always)]
    pub fn is_shr1(&self) -> bool {
        *self == Opcode::Shr1
    }
    #[doc = "Shift left 0 operation"]
    #[inline(always)]
    pub fn is_shl0(&self) -> bool {
        *self == Opcode::Shl0
    }
    #[doc = "Shift left 1 operation"]
    #[inline(always)]
    pub fn is_shl1(&self) -> bool {
        *self == Opcode::Shl1
    }
    #[doc = "Multiply low operation"]
    #[inline(always)]
    pub fn is_mul_low(&self) -> bool {
        *self == Opcode::MulLow
    }
    #[doc = "Modular multiply operation"]
    #[inline(always)]
    pub fn is_mod_mul(&self) -> bool {
        *self == Opcode::ModMul
    }
    #[doc = "Modular multiply N operation"]
    #[inline(always)]
    pub fn is_mod_mul_n(&self) -> bool {
        *self == Opcode::ModMulN
    }
    #[doc = "Modular exponentiation operation"]
    #[inline(always)]
    pub fn is_mod_exp(&self) -> bool {
        *self == Opcode::ModExp
    }
    #[doc = "Division operation"]
    #[inline(always)]
    pub fn is_division(&self) -> bool {
        *self == Opcode::Division
    }
    #[doc = "Modular inversion operation"]
    #[inline(always)]
    pub fn is_mod_inv(&self) -> bool {
        *self == Opcode::ModInv
    }
    #[doc = "Modular division operation"]
    #[inline(always)]
    pub fn is_mod_div(&self) -> bool {
        *self == Opcode::ModDiv
    }
    #[doc = "Multiply high operation"]
    #[inline(always)]
    pub fn is_mul_high(&self) -> bool {
        *self == Opcode::MulHigh
    }
    #[doc = "Modular multiplication acceleration"]
    #[inline(always)]
    pub fn is_mod_mlac(&self) -> bool {
        *self == Opcode::ModMlac
    }
    #[doc = "Modular multiplication acceleration where final reduction is omitted"]
    #[inline(always)]
    pub fn is_mod_mlacnr(&self) -> bool {
        *self == Opcode::ModMlacnr
    }
    #[doc = "Reduction operation"]
    #[inline(always)]
    pub fn is_reduction(&self) -> bool {
        *self == Opcode::Reduction
    }
}
#[doc = "Field `OPCODE` writer - Operation code to be executed by the PKA engine"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Opcode>;
impl<'a, REG> OpcodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Terminate operation"]
    #[inline(always)]
    pub fn terminate(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Terminate)
    }
    #[doc = "Add or Increment"]
    #[inline(always)]
    pub fn add_inc(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::AddInc)
    }
    #[doc = "Subtract, Decrement, or Negate"]
    #[inline(always)]
    pub fn sub_dec_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::SubDecNeg)
    }
    #[doc = "Modular Add or Modular Increment"]
    #[inline(always)]
    pub fn mod_add_inc(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModAddInc)
    }
    #[doc = "Modular Subtract, Modular Decrement, or Modular Negate"]
    #[inline(always)]
    pub fn mod_sub_dec_neg(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModSubDecNeg)
    }
    #[doc = "Perform AND, test, or clear"]
    #[inline(always)]
    pub fn andtst0clr0(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Andtst0clr0)
    }
    #[doc = "Perform OR, copy, or set bits"]
    #[inline(always)]
    pub fn orcopyset0(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Orcopyset0)
    }
    #[doc = "Perform XOR, flip bits, invert, or compare"]
    #[inline(always)]
    pub fn xorflp0invcmp(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Xorflp0invcmp)
    }
    #[doc = "Shift right 0 operation"]
    #[inline(always)]
    pub fn shr0(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shr0)
    }
    #[doc = "Shift right 1 operation"]
    #[inline(always)]
    pub fn shr1(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shr1)
    }
    #[doc = "Shift left 0 operation"]
    #[inline(always)]
    pub fn shl0(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shl0)
    }
    #[doc = "Shift left 1 operation"]
    #[inline(always)]
    pub fn shl1(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shl1)
    }
    #[doc = "Multiply low operation"]
    #[inline(always)]
    pub fn mul_low(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::MulLow)
    }
    #[doc = "Modular multiply operation"]
    #[inline(always)]
    pub fn mod_mul(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModMul)
    }
    #[doc = "Modular multiply N operation"]
    #[inline(always)]
    pub fn mod_mul_n(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModMulN)
    }
    #[doc = "Modular exponentiation operation"]
    #[inline(always)]
    pub fn mod_exp(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModExp)
    }
    #[doc = "Division operation"]
    #[inline(always)]
    pub fn division(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Division)
    }
    #[doc = "Modular inversion operation"]
    #[inline(always)]
    pub fn mod_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModInv)
    }
    #[doc = "Modular division operation"]
    #[inline(always)]
    pub fn mod_div(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModDiv)
    }
    #[doc = "Multiply high operation"]
    #[inline(always)]
    pub fn mul_high(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::MulHigh)
    }
    #[doc = "Modular multiplication acceleration"]
    #[inline(always)]
    pub fn mod_mlac(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModMlac)
    }
    #[doc = "Modular multiplication acceleration where final reduction is omitted"]
    #[inline(always)]
    pub fn mod_mlacnr(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::ModMlacnr)
    }
    #[doc = "Reduction operation"]
    #[inline(always)]
    pub fn reduction(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Reduction)
    }
}
impl R {
    #[doc = "Bits 0:5 - Holds the operation tag or the operand C virtual register index."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - Result register virtual register index."]
    #[inline(always)]
    pub fn reg_r(&self) -> RegRR {
        RegRR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - This field controls the interpretation of REG_R."]
    #[inline(always)]
    pub fn discard_r(&self) -> DiscardRR {
        DiscardRR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - Operand B virtual register index."]
    #[inline(always)]
    pub fn reg_b(&self) -> RegBR {
        RegBR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - This field controls the interpretation of REG_B."]
    #[inline(always)]
    pub fn const_b(&self) -> ConstBR {
        ConstBR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - Operand A virtual register index."]
    #[inline(always)]
    pub fn reg_a(&self) -> RegAR {
        RegAR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - This field controls the interpretation of REG_A."]
    #[inline(always)]
    pub fn const_a(&self) -> ConstAR {
        ConstAR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - The length of the operands. This value serves as an PKA length register index. E.g.: if LEN field value is set to 0, PKA_L\\[0\\] holds the size of the operands."]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - Operation code to be executed by the PKA engine"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Holds the operation tag or the operand C virtual register index."]
    #[inline(always)]
    pub fn tag(&mut self) -> TagW<'_, OpcodeSpec> {
        TagW::new(self, 0)
    }
    #[doc = "Bits 6:10 - Result register virtual register index."]
    #[inline(always)]
    pub fn reg_r(&mut self) -> RegRW<'_, OpcodeSpec> {
        RegRW::new(self, 6)
    }
    #[doc = "Bit 11 - This field controls the interpretation of REG_R."]
    #[inline(always)]
    pub fn discard_r(&mut self) -> DiscardRW<'_, OpcodeSpec> {
        DiscardRW::new(self, 11)
    }
    #[doc = "Bits 12:16 - Operand B virtual register index."]
    #[inline(always)]
    pub fn reg_b(&mut self) -> RegBW<'_, OpcodeSpec> {
        RegBW::new(self, 12)
    }
    #[doc = "Bit 17 - This field controls the interpretation of REG_B."]
    #[inline(always)]
    pub fn const_b(&mut self) -> ConstBW<'_, OpcodeSpec> {
        ConstBW::new(self, 17)
    }
    #[doc = "Bits 18:22 - Operand A virtual register index."]
    #[inline(always)]
    pub fn reg_a(&mut self) -> RegAW<'_, OpcodeSpec> {
        RegAW::new(self, 18)
    }
    #[doc = "Bit 23 - This field controls the interpretation of REG_A."]
    #[inline(always)]
    pub fn const_a(&mut self) -> ConstAW<'_, OpcodeSpec> {
        ConstAW::new(self, 23)
    }
    #[doc = "Bits 24:26 - The length of the operands. This value serves as an PKA length register index. E.g.: if LEN field value is set to 0, PKA_L\\[0\\] holds the size of the operands."]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<'_, OpcodeSpec> {
        LenW::new(self, 24)
    }
    #[doc = "Bits 27:31 - Operation code to be executed by the PKA engine"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<'_, OpcodeSpec> {
        OpcodeW::new(self, 27)
    }
}
#[doc = "Operation code to be executed by the PKA engine. Writing to this register triggers the PKA operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`opcode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpcodeSpec;
impl crate::RegisterSpec for OpcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcode::R`](R) reader structure"]
impl crate::Readable for OpcodeSpec {}
#[doc = "`write(|w| ..)` method takes [`opcode::W`](W) writer structure"]
impl crate::Writable for OpcodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPCODE to value 0"]
impl crate::Resettable for OpcodeSpec {}
