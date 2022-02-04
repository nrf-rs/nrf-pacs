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
#[doc = "Write '1' to enable interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<KEYSLOT_PUSHED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_PUSHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` reader - Write '1' to enable interrupt for event KEYSLOT_PUSHED"]
pub struct KEYSLOT_PUSHED_R(crate::FieldReader<bool, KEYSLOT_PUSHED_A>);
impl KEYSLOT_PUSHED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYSLOT_PUSHED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_PUSHED_A {
        match self.bits {
            false => KEYSLOT_PUSHED_A::DISABLED,
            true => KEYSLOT_PUSHED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == KEYSLOT_PUSHED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == KEYSLOT_PUSHED_A::ENABLED
    }
}
impl core::ops::Deref for KEYSLOT_PUSHED_R {
    type Target = crate::FieldReader<bool, KEYSLOT_PUSHED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<KEYSLOT_PUSHED_AW> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_PUSHED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` writer - Write '1' to enable interrupt for event KEYSLOT_PUSHED"]
pub struct KEYSLOT_PUSHED_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSLOT_PUSHED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSLOT_PUSHED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(KEYSLOT_PUSHED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<KEYSLOT_REVOKED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_REVOKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` reader - Write '1' to enable interrupt for event KEYSLOT_REVOKED"]
pub struct KEYSLOT_REVOKED_R(crate::FieldReader<bool, KEYSLOT_REVOKED_A>);
impl KEYSLOT_REVOKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYSLOT_REVOKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_REVOKED_A {
        match self.bits {
            false => KEYSLOT_REVOKED_A::DISABLED,
            true => KEYSLOT_REVOKED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == KEYSLOT_REVOKED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == KEYSLOT_REVOKED_A::ENABLED
    }
}
impl core::ops::Deref for KEYSLOT_REVOKED_R {
    type Target = crate::FieldReader<bool, KEYSLOT_REVOKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<KEYSLOT_REVOKED_AW> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_REVOKED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` writer - Write '1' to enable interrupt for event KEYSLOT_REVOKED"]
pub struct KEYSLOT_REVOKED_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSLOT_REVOKED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSLOT_REVOKED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(KEYSLOT_REVOKED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<KEYSLOT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` reader - Write '1' to enable interrupt for event KEYSLOT_ERROR"]
pub struct KEYSLOT_ERROR_R(crate::FieldReader<bool, KEYSLOT_ERROR_A>);
impl KEYSLOT_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYSLOT_ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_ERROR_A {
        match self.bits {
            false => KEYSLOT_ERROR_A::DISABLED,
            true => KEYSLOT_ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == KEYSLOT_ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == KEYSLOT_ERROR_A::ENABLED
    }
}
impl core::ops::Deref for KEYSLOT_ERROR_R {
    type Target = crate::FieldReader<bool, KEYSLOT_ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<KEYSLOT_ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` writer - Write '1' to enable interrupt for event KEYSLOT_ERROR"]
pub struct KEYSLOT_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSLOT_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSLOT_ERROR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(KEYSLOT_ERROR_AW::SET)
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
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KEYSLOT_PUSHED_R {
        KEYSLOT_PUSHED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KEYSLOT_REVOKED_R {
        KEYSLOT_REVOKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KEYSLOT_ERROR_R {
        KEYSLOT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&mut self) -> KEYSLOT_PUSHED_W {
        KEYSLOT_PUSHED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&mut self) -> KEYSLOT_REVOKED_W {
        KEYSLOT_REVOKED_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&mut self) -> KEYSLOT_ERROR_W {
        KEYSLOT_ERROR_W { w: self }
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
