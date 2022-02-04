#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDOPERATION_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<INVALIDOPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: INVALIDOPERATION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDOPERATION` reader - Write '1' to disable interrupt for event INVALIDOPERATION"]
pub struct INVALIDOPERATION_R(crate::FieldReader<bool, INVALIDOPERATION_A>);
impl INVALIDOPERATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVALIDOPERATION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVALIDOPERATION_A {
        match self.bits {
            false => INVALIDOPERATION_A::DISABLED,
            true => INVALIDOPERATION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INVALIDOPERATION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INVALIDOPERATION_A::ENABLED
    }
}
impl core::ops::Deref for INVALIDOPERATION_R {
    type Target = crate::FieldReader<bool, INVALIDOPERATION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDOPERATION_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<INVALIDOPERATION_AW> for bool {
    #[inline(always)]
    fn from(variant: INVALIDOPERATION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDOPERATION` writer - Write '1' to disable interrupt for event INVALIDOPERATION"]
pub struct INVALIDOPERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALIDOPERATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVALIDOPERATION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INVALIDOPERATION_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDEBYZERO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DIVIDEBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: DIVIDEBYZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVIDEBYZERO` reader - Write '1' to disable interrupt for event DIVIDEBYZERO"]
pub struct DIVIDEBYZERO_R(crate::FieldReader<bool, DIVIDEBYZERO_A>);
impl DIVIDEBYZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIVIDEBYZERO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVIDEBYZERO_A {
        match self.bits {
            false => DIVIDEBYZERO_A::DISABLED,
            true => DIVIDEBYZERO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DIVIDEBYZERO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DIVIDEBYZERO_A::ENABLED
    }
}
impl core::ops::Deref for DIVIDEBYZERO_R {
    type Target = crate::FieldReader<bool, DIVIDEBYZERO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDEBYZERO_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DIVIDEBYZERO_AW> for bool {
    #[inline(always)]
    fn from(variant: DIVIDEBYZERO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVIDEBYZERO` writer - Write '1' to disable interrupt for event DIVIDEBYZERO"]
pub struct DIVIDEBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDEBYZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVIDEBYZERO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DIVIDEBYZERO_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - Write '1' to disable interrupt for event OVERFLOW"]
pub struct OVERFLOW_R(crate::FieldReader<bool, OVERFLOW_A>);
impl OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::DISABLED,
            true => OVERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVERFLOW_A::ENABLED
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, OVERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<OVERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` writer - Write '1' to disable interrupt for event OVERFLOW"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFLOW` reader - Write '1' to disable interrupt for event UNDERFLOW"]
pub struct UNDERFLOW_R(crate::FieldReader<bool, UNDERFLOW_A>);
impl UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_A {
        match self.bits {
            false => UNDERFLOW_A::DISABLED,
            true => UNDERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UNDERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UNDERFLOW_A::ENABLED
    }
}
impl core::ops::Deref for UNDERFLOW_R {
    type Target = crate::FieldReader<bool, UNDERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<UNDERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFLOW` writer - Write '1' to disable interrupt for event UNDERFLOW"]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDERFLOW_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UNDERFLOW_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEXACT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<INEXACT_A> for bool {
    #[inline(always)]
    fn from(variant: INEXACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEXACT` reader - Write '1' to disable interrupt for event INEXACT"]
pub struct INEXACT_R(crate::FieldReader<bool, INEXACT_A>);
impl INEXACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INEXACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEXACT_A {
        match self.bits {
            false => INEXACT_A::DISABLED,
            true => INEXACT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INEXACT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INEXACT_A::ENABLED
    }
}
impl core::ops::Deref for INEXACT_R {
    type Target = crate::FieldReader<bool, INEXACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEXACT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<INEXACT_AW> for bool {
    #[inline(always)]
    fn from(variant: INEXACT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEXACT` writer - Write '1' to disable interrupt for event INEXACT"]
pub struct INEXACT_W<'a> {
    w: &'a mut W,
}
impl<'a> INEXACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INEXACT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INEXACT_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENORMALINPUT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DENORMALINPUT_A> for bool {
    #[inline(always)]
    fn from(variant: DENORMALINPUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENORMALINPUT` reader - Write '1' to disable interrupt for event DENORMALINPUT"]
pub struct DENORMALINPUT_R(crate::FieldReader<bool, DENORMALINPUT_A>);
impl DENORMALINPUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DENORMALINPUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DENORMALINPUT_A {
        match self.bits {
            false => DENORMALINPUT_A::DISABLED,
            true => DENORMALINPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DENORMALINPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DENORMALINPUT_A::ENABLED
    }
}
impl core::ops::Deref for DENORMALINPUT_R {
    type Target = crate::FieldReader<bool, DENORMALINPUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENORMALINPUT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DENORMALINPUT_AW> for bool {
    #[inline(always)]
    fn from(variant: DENORMALINPUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENORMALINPUT` writer - Write '1' to disable interrupt for event DENORMALINPUT"]
pub struct DENORMALINPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DENORMALINPUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DENORMALINPUT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DENORMALINPUT_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&self) -> INVALIDOPERATION_R {
        INVALIDOPERATION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&self) -> DIVIDEBYZERO_R {
        DIVIDEBYZERO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&self) -> INEXACT_R {
        INEXACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&self) -> DENORMALINPUT_R {
        DENORMALINPUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&mut self) -> INVALIDOPERATION_W {
        INVALIDOPERATION_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&mut self) -> DIVIDEBYZERO_W {
        DIVIDEBYZERO_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&mut self) -> INEXACT_W {
        INEXACT_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&mut self) -> DENORMALINPUT_W {
        DENORMALINPUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
