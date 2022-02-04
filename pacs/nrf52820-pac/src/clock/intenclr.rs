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
#[doc = "Write '1' to disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` reader - Write '1' to disable interrupt for event HFCLKSTARTED"]
pub struct HFCLKSTARTED_R(crate::FieldReader<bool, HFCLKSTARTED_A>);
impl HFCLKSTARTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HFCLKSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::DISABLED,
            true => HFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HFCLKSTARTED_A::ENABLED
    }
}
impl core::ops::Deref for HFCLKSTARTED_R {
    type Target = crate::FieldReader<bool, HFCLKSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<HFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` writer - Write '1' to disable interrupt for event HFCLKSTARTED"]
pub struct HFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCLKSTARTED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HFCLKSTARTED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Write '1' to disable interrupt for event LFCLKSTARTED"]
pub struct LFCLKSTARTED_R(crate::FieldReader<bool, LFCLKSTARTED_A>);
impl LFCLKSTARTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LFCLKSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::DISABLED,
            true => LFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LFCLKSTARTED_A::ENABLED
    }
}
impl core::ops::Deref for LFCLKSTARTED_R {
    type Target = crate::FieldReader<bool, LFCLKSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<LFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Write '1' to disable interrupt for event LFCLKSTARTED"]
pub struct LFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> LFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFCLKSTARTED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LFCLKSTARTED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Write '1' to disable interrupt for event DONE"]
pub struct DONE_R(crate::FieldReader<bool, DONE_A>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::DISABLED,
            true => DONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DONE_A::ENABLED
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DONE_AW> for bool {
    #[inline(always)]
    fn from(variant: DONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to disable interrupt for event DONE"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DONE_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTTO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTTO_A> for bool {
    #[inline(always)]
    fn from(variant: CTTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTO` reader - Write '1' to disable interrupt for event CTTO"]
pub struct CTTO_R(crate::FieldReader<bool, CTTO_A>);
impl CTTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTTO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTTO_A {
        match self.bits {
            false => CTTO_A::DISABLED,
            true => CTTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTTO_A::ENABLED
    }
}
impl core::ops::Deref for CTTO_R {
    type Target = crate::FieldReader<bool, CTTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTTO_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CTTO_AW> for bool {
    #[inline(always)]
    fn from(variant: CTTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTO` writer - Write '1' to disable interrupt for event CTTO"]
pub struct CTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTTO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTTO_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CTSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: CTSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSTARTED` reader - Write '1' to disable interrupt for event CTSTARTED"]
pub struct CTSTARTED_R(crate::FieldReader<bool, CTSTARTED_A>);
impl CTSTARTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSTARTED_A {
        match self.bits {
            false => CTSTARTED_A::DISABLED,
            true => CTSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTSTARTED_A::ENABLED
    }
}
impl core::ops::Deref for CTSTARTED_R {
    type Target = crate::FieldReader<bool, CTSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CTSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CTSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSTARTED` writer - Write '1' to disable interrupt for event CTSTARTED"]
pub struct CTSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSTARTED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSTARTED_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CTSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTSTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: CTSTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSTOPPED` reader - Write '1' to disable interrupt for event CTSTOPPED"]
pub struct CTSTOPPED_R(crate::FieldReader<bool, CTSTOPPED_A>);
impl CTSTOPPED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSTOPPED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSTOPPED_A {
        match self.bits {
            false => CTSTOPPED_A::DISABLED,
            true => CTSTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTSTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTSTOPPED_A::ENABLED
    }
}
impl core::ops::Deref for CTSTOPPED_R {
    type Target = crate::FieldReader<bool, CTSTOPPED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CTSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CTSTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSTOPPED` writer - Write '1' to disable interrupt for event CTSTOPPED"]
pub struct CTSTOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSTOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSTOPPED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSTOPPED_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event CTTO"]
    #[inline(always)]
    pub fn ctto(&self) -> CTTO_R {
        CTTO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event CTSTARTED"]
    #[inline(always)]
    pub fn ctstarted(&self) -> CTSTARTED_R {
        CTSTARTED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event CTSTOPPED"]
    #[inline(always)]
    pub fn ctstopped(&self) -> CTSTOPPED_R {
        CTSTOPPED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&mut self) -> HFCLKSTARTED_W {
        HFCLKSTARTED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LFCLKSTARTED_W {
        LFCLKSTARTED_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event CTTO"]
    #[inline(always)]
    pub fn ctto(&mut self) -> CTTO_W {
        CTTO_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event CTSTARTED"]
    #[inline(always)]
    pub fn ctstarted(&mut self) -> CTSTARTED_W {
        CTSTARTED_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event CTSTOPPED"]
    #[inline(always)]
    pub fn ctstopped(&mut self) -> CTSTOPPED_W {
        CTSTOPPED_W { w: self }
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
