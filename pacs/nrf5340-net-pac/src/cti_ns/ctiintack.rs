#[doc = "Register `CTIINTACK` writer"]
pub struct W(crate::W<CTIINTACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIINTACK_SPEC>;
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
impl From<crate::W<CTIINTACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIINTACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGREQ_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<DEBUGREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: DEBUGREQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGREQ` writer - Processor debug request"]
pub struct DEBUGREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUGREQ_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(DEBUGREQ_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURESTART_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<CPURESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: CPURESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURESTART` writer - Processor Restart"]
pub struct CPURESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CPURESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPURESTART_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(CPURESTART_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED0_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED0` writer - N/A"]
pub struct UNUSED0_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED0_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED1_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED1_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED1` writer - N/A"]
pub struct UNUSED1_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED1_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED2_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED2_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED2` writer - N/A"]
pub struct UNUSED2_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED2_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED3_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED3_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED3` writer - N/A"]
pub struct UNUSED3_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED3_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED4_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED4_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED4` writer - N/A"]
pub struct UNUSED4_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED4_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED5_AW {
    #[doc = "1: Clears the ctitrigout."]
    ACKNOWLEDGE = 1,
}
impl From<UNUSED5_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED5` writer - N/A"]
pub struct UNUSED5_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ctitrigout."]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED5_AW::ACKNOWLEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&mut self) -> DEBUGREQ_W {
        DEBUGREQ_W { w: self }
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&mut self) -> CPURESTART_W {
        CPURESTART_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&mut self) -> UNUSED0_W {
        UNUSED0_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&mut self) -> UNUSED1_W {
        UNUSED1_W { w: self }
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused2(&mut self) -> UNUSED2_W {
        UNUSED2_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused3(&mut self) -> UNUSED3_W {
        UNUSED3_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused4(&mut self) -> UNUSED4_W {
        UNUSED4_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused5(&mut self) -> UNUSED5_W {
        UNUSED5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTI Interrupt Acknowledge register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiintack](index.html) module"]
pub struct CTIINTACK_SPEC;
impl crate::RegisterSpec for CTIINTACK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctiintack::W](W) writer structure"]
impl crate::Writable for CTIINTACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIINTACK to value 0"]
impl crate::Resettable for CTIINTACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
