#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDECB_A> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` reader - Write '1' to enable interrupt for event ENDECB"]
pub struct ENDECB_R(crate::FieldReader<bool, ENDECB_A>);
impl ENDECB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDECB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDECB_A {
        match self.bits {
            false => ENDECB_A::DISABLED,
            true => ENDECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDECB_A::ENABLED
    }
}
impl core::ops::Deref for ENDECB_R {
    type Target = crate::FieldReader<bool, ENDECB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event ENDECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` writer - Write '1' to enable interrupt for event ENDECB"]
pub struct ENDECB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDECB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDECB_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDECB_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event ERRORECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERRORECB_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` reader - Write '1' to enable interrupt for event ERRORECB"]
pub struct ERRORECB_R(crate::FieldReader<bool, ERRORECB_A>);
impl ERRORECB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRORECB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORECB_A {
        match self.bits {
            false => ERRORECB_A::DISABLED,
            true => ERRORECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRORECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRORECB_A::ENABLED
    }
}
impl core::ops::Deref for ERRORECB_R {
    type Target = crate::FieldReader<bool, ERRORECB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event ERRORECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ERRORECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` writer - Write '1' to enable interrupt for event ERRORECB"]
pub struct ERRORECB_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORECB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRORECB_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERRORECB_AW::SET)
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
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event ENDECB"]
    #[inline(always)]
    pub fn endecb(&self) -> ENDECB_R {
        ENDECB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ERRORECB"]
    #[inline(always)]
    pub fn errorecb(&self) -> ERRORECB_R {
        ERRORECB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event ENDECB"]
    #[inline(always)]
    pub fn endecb(&mut self) -> ENDECB_W {
        ENDECB_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ERRORECB"]
    #[inline(always)]
    pub fn errorecb(&mut self) -> ERRORECB_W {
        ERRORECB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
