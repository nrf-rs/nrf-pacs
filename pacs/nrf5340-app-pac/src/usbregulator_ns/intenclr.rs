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
#[doc = "Write '1' to disable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDETECTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<USBDETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: USBDETECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDETECTED` reader - Write '1' to disable interrupt for event USBDETECTED"]
pub struct USBDETECTED_R(crate::FieldReader<bool, USBDETECTED_A>);
impl USBDETECTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBDETECTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDETECTED_A {
        match self.bits {
            false => USBDETECTED_A::DISABLED,
            true => USBDETECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USBDETECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USBDETECTED_A::ENABLED
    }
}
impl core::ops::Deref for USBDETECTED_R {
    type Target = crate::FieldReader<bool, USBDETECTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDETECTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<USBDETECTED_AW> for bool {
    #[inline(always)]
    fn from(variant: USBDETECTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDETECTED` writer - Write '1' to disable interrupt for event USBDETECTED"]
pub struct USBDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDETECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDETECTED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBDETECTED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREMOVED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<USBREMOVED_A> for bool {
    #[inline(always)]
    fn from(variant: USBREMOVED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREMOVED` reader - Write '1' to disable interrupt for event USBREMOVED"]
pub struct USBREMOVED_R(crate::FieldReader<bool, USBREMOVED_A>);
impl USBREMOVED_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBREMOVED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBREMOVED_A {
        match self.bits {
            false => USBREMOVED_A::DISABLED,
            true => USBREMOVED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USBREMOVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USBREMOVED_A::ENABLED
    }
}
impl core::ops::Deref for USBREMOVED_R {
    type Target = crate::FieldReader<bool, USBREMOVED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREMOVED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<USBREMOVED_AW> for bool {
    #[inline(always)]
    fn from(variant: USBREMOVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREMOVED` writer - Write '1' to disable interrupt for event USBREMOVED"]
pub struct USBREMOVED_W<'a> {
    w: &'a mut W,
}
impl<'a> USBREMOVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBREMOVED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBREMOVED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPWRRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<USBPWRRDY_A> for bool {
    #[inline(always)]
    fn from(variant: USBPWRRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPWRRDY` reader - Write '1' to disable interrupt for event USBPWRRDY"]
pub struct USBPWRRDY_R(crate::FieldReader<bool, USBPWRRDY_A>);
impl USBPWRRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBPWRRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPWRRDY_A {
        match self.bits {
            false => USBPWRRDY_A::DISABLED,
            true => USBPWRRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USBPWRRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USBPWRRDY_A::ENABLED
    }
}
impl core::ops::Deref for USBPWRRDY_R {
    type Target = crate::FieldReader<bool, USBPWRRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPWRRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<USBPWRRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPWRRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPWRRDY` writer - Write '1' to disable interrupt for event USBPWRRDY"]
pub struct USBPWRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPWRRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPWRRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBPWRRDY_AW::CLEAR)
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
    #[doc = "Bit 0 - Write '1' to disable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&self) -> USBDETECTED_R {
        USBDETECTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&self) -> USBREMOVED_R {
        USBREMOVED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&self) -> USBPWRRDY_R {
        USBPWRRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&mut self) -> USBDETECTED_W {
        USBDETECTED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&mut self) -> USBREMOVED_W {
        USBREMOVED_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&mut self) -> USBPWRRDY_W {
        USBPWRRDY_W { w: self }
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
