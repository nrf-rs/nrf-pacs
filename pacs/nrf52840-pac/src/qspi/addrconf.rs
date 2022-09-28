#[doc = "Register `ADDRCONF` reader"]
pub struct R(crate::R<ADDRCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRCONF` writer"]
pub struct W(crate::W<ADDRCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADDRCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - Opcode that enters the 32-bit addressing mode."]
pub type OPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCODE` writer - Opcode that enters the 32-bit addressing mode."]
pub type OPCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRCONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `BYTE0` reader - Byte 0 following opcode."]
pub type BYTE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE0` writer - Byte 0 following opcode."]
pub type BYTE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRCONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `BYTE1` reader - Byte 1 following byte 0."]
pub type BYTE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYTE1` writer - Byte 1 following byte 0."]
pub type BYTE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRCONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `MODE` reader - Extended addressing mode."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Extended addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Do not send any instruction."]
    NO_INSTR = 0,
    #[doc = "1: Send opcode."]
    OPCODE = 1,
    #[doc = "2: Send opcode, byte0."]
    OP_BYTE0 = 2,
    #[doc = "3: Send opcode, byte0, byte1."]
    ALL = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NO_INSTR,
            1 => MODE_A::OPCODE,
            2 => MODE_A::OP_BYTE0,
            3 => MODE_A::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_INSTR`"]
    #[inline(always)]
    pub fn is_no_instr(&self) -> bool {
        *self == MODE_A::NO_INSTR
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == MODE_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `OP_BYTE0`"]
    #[inline(always)]
    pub fn is_op_byte0(&self) -> bool {
        *self == MODE_A::OP_BYTE0
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == MODE_A::ALL
    }
}
#[doc = "Field `MODE` writer - Extended addressing mode."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ADDRCONF_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Do not send any instruction."]
    #[inline(always)]
    pub fn no_instr(self) -> &'a mut W {
        self.variant(MODE_A::NO_INSTR)
    }
    #[doc = "Send opcode."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut W {
        self.variant(MODE_A::OPCODE)
    }
    #[doc = "Send opcode, byte0."]
    #[inline(always)]
    pub fn op_byte0(self) -> &'a mut W {
        self.variant(MODE_A::OP_BYTE0)
    }
    #[doc = "Send opcode, byte0, byte1."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(MODE_A::ALL)
    }
}
#[doc = "Field `WIPWAIT` reader - Wait for write complete before sending command."]
pub type WIPWAIT_R = crate::BitReader<WIPWAIT_A>;
#[doc = "Wait for write complete before sending command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPWAIT_A {
    #[doc = "0: No wait."]
    DISABLE = 0,
    #[doc = "1: Wait."]
    ENABLE = 1,
}
impl From<WIPWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WIPWAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WIPWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIPWAIT_A {
        match self.bits {
            false => WIPWAIT_A::DISABLE,
            true => WIPWAIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WIPWAIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WIPWAIT_A::ENABLE
    }
}
#[doc = "Field `WIPWAIT` writer - Wait for write complete before sending command."]
pub type WIPWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDRCONF_SPEC, WIPWAIT_A, O>;
impl<'a, const O: u8> WIPWAIT_W<'a, O> {
    #[doc = "No wait."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WIPWAIT_A::DISABLE)
    }
    #[doc = "Wait."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WIPWAIT_A::ENABLE)
    }
}
#[doc = "Field `WREN` reader - Send WREN (write enable opcode 0x06) before instruction."]
pub type WREN_R = crate::BitReader<WREN_A>;
#[doc = "Send WREN (write enable opcode 0x06) before instruction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WREN_A {
    #[doc = "0: Do not send WREN."]
    DISABLE = 0,
    #[doc = "1: Send WREN."]
    ENABLE = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
impl WREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::DISABLE,
            true => WREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WREN_A::ENABLE
    }
}
#[doc = "Field `WREN` writer - Send WREN (write enable opcode 0x06) before instruction."]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDRCONF_SPEC, WREN_A, O>;
impl<'a, const O: u8> WREN_W<'a, O> {
    #[doc = "Do not send WREN."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WREN_A::DISABLE)
    }
    #[doc = "Send WREN."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WREN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WIPWAIT_R {
        WIPWAIT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W<0> {
        OPCODE_W::new(self)
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W<8> {
        BYTE0_W::new(self)
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W<16> {
        BYTE1_W::new(self)
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&mut self) -> WIPWAIT_W<26> {
        WIPWAIT_W::new(self)
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<27> {
        WREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended address configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrconf](index.html) module"]
pub struct ADDRCONF_SPEC;
impl crate::RegisterSpec for ADDRCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrconf::R](R) reader structure"]
impl crate::Readable for ADDRCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrconf::W](W) writer structure"]
impl crate::Writable for ADDRCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRCONF to value 0xb7"]
impl crate::Resettable for ADDRCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb7
    }
}
