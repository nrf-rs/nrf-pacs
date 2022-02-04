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
pub struct OPCODE_R(crate::FieldReader<u8, u8>);
impl OPCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPCODE` writer - Opcode that enters the 32-bit addressing mode."]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BYTE0` reader - Byte 0 following opcode."]
pub struct BYTE0_R(crate::FieldReader<u8, u8>);
impl BYTE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE0` writer - Byte 0 following opcode."]
pub struct BYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE1` reader - Byte 1 following byte 0."]
pub struct BYTE1_R(crate::FieldReader<u8, u8>);
impl BYTE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE1` writer - Byte 1 following byte 0."]
pub struct BYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Extended addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Do not send any instruction."]
    NOINSTR = 0,
    #[doc = "1: Send opcode."]
    OPCODE = 1,
    #[doc = "2: Send opcode, byte0."]
    OPBYTE0 = 2,
    #[doc = "3: Send opcode, byte0, byte1."]
    ALL = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Extended addressing mode."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NOINSTR,
            1 => MODE_A::OPCODE,
            2 => MODE_A::OPBYTE0,
            3 => MODE_A::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOINSTR`"]
    #[inline(always)]
    pub fn is_no_instr(&self) -> bool {
        **self == MODE_A::NOINSTR
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        **self == MODE_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPBYTE0`"]
    #[inline(always)]
    pub fn is_op_byte0(&self) -> bool {
        **self == MODE_A::OPBYTE0
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == MODE_A::ALL
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Extended addressing mode."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do not send any instruction."]
    #[inline(always)]
    pub fn no_instr(self) -> &'a mut W {
        self.variant(MODE_A::NOINSTR)
    }
    #[doc = "Send opcode."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut W {
        self.variant(MODE_A::OPCODE)
    }
    #[doc = "Send opcode, byte0."]
    #[inline(always)]
    pub fn op_byte0(self) -> &'a mut W {
        self.variant(MODE_A::OPBYTE0)
    }
    #[doc = "Send opcode, byte0, byte1."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(MODE_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
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
#[doc = "Field `WIPWAIT` reader - Wait for write complete before sending command."]
pub struct WIPWAIT_R(crate::FieldReader<bool, WIPWAIT_A>);
impl WIPWAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIPWAIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == WIPWAIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WIPWAIT_A::ENABLE
    }
}
impl core::ops::Deref for WIPWAIT_R {
    type Target = crate::FieldReader<bool, WIPWAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIPWAIT` writer - Wait for write complete before sending command."]
pub struct WIPWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WIPWAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIPWAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
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
#[doc = "Field `WREN` reader - Send WREN (write enable opcode 0x06) before instruction."]
pub struct WREN_R(crate::FieldReader<bool, WREN_A>);
impl WREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == WREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WREN_A::ENABLE
    }
}
impl core::ops::Deref for WREN_R {
    type Target = crate::FieldReader<bool, WREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WREN` writer - Send WREN (write enable opcode 0x06) before instruction."]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
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
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WIPWAIT_R {
        WIPWAIT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W {
        BYTE0_W { w: self }
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W { w: self }
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&mut self) -> WIPWAIT_W {
        WIPWAIT_W { w: self }
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
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
