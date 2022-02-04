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
#[doc = "Write '1' to enable interrupt for event POFWARN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARN_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<POFWARN_A> for bool {
    #[inline(always)]
    fn from(variant: POFWARN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` reader - Write '1' to enable interrupt for event POFWARN"]
pub struct POFWARN_R(crate::FieldReader<bool, POFWARN_A>);
impl POFWARN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POFWARN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POFWARN_A {
        match self.bits {
            false => POFWARN_A::DISABLED,
            true => POFWARN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == POFWARN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == POFWARN_A::ENABLED
    }
}
impl core::ops::Deref for POFWARN_R {
    type Target = crate::FieldReader<bool, POFWARN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event POFWARN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARN_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<POFWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: POFWARN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` writer - Write '1' to enable interrupt for event POFWARN"]
pub struct POFWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> POFWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFWARN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(POFWARN_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event SLEEPENTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPENTER_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SLEEPENTER_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPENTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPENTER` reader - Write '1' to enable interrupt for event SLEEPENTER"]
pub struct SLEEPENTER_R(crate::FieldReader<bool, SLEEPENTER_A>);
impl SLEEPENTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPENTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPENTER_A {
        match self.bits {
            false => SLEEPENTER_A::DISABLED,
            true => SLEEPENTER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SLEEPENTER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLEEPENTER_A::ENABLED
    }
}
impl core::ops::Deref for SLEEPENTER_R {
    type Target = crate::FieldReader<bool, SLEEPENTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event SLEEPENTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPENTER_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<SLEEPENTER_AW> for bool {
    #[inline(always)]
    fn from(variant: SLEEPENTER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPENTER` writer - Write '1' to enable interrupt for event SLEEPENTER"]
pub struct SLEEPENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPENTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPENTER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SLEEPENTER_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event SLEEPEXIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEXIT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SLEEPEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEXIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEXIT` reader - Write '1' to enable interrupt for event SLEEPEXIT"]
pub struct SLEEPEXIT_R(crate::FieldReader<bool, SLEEPEXIT_A>);
impl SLEEPEXIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPEXIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPEXIT_A {
        match self.bits {
            false => SLEEPEXIT_A::DISABLED,
            true => SLEEPEXIT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SLEEPEXIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLEEPEXIT_A::ENABLED
    }
}
impl core::ops::Deref for SLEEPEXIT_R {
    type Target = crate::FieldReader<bool, SLEEPEXIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to enable interrupt for event SLEEPEXIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEXIT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<SLEEPEXIT_AW> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEXIT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEXIT` writer - Write '1' to enable interrupt for event SLEEPEXIT"]
pub struct SLEEPEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEXIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPEXIT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SLEEPEXIT_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event USBDETECTED\n\nValue on reset: 0"]
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
#[doc = "Field `USBDETECTED` reader - Write '1' to enable interrupt for event USBDETECTED"]
pub struct USBDETECTED_R(crate::FieldReader<bool, USBDETECTED_A>);
impl USBDETECTED_R {
    #[inline(always)]
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
#[doc = "Write '1' to enable interrupt for event USBDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDETECTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<USBDETECTED_AW> for bool {
    #[inline(always)]
    fn from(variant: USBDETECTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDETECTED` writer - Write '1' to enable interrupt for event USBDETECTED"]
pub struct USBDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDETECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBDETECTED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(USBDETECTED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event USBREMOVED\n\nValue on reset: 0"]
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
#[doc = "Field `USBREMOVED` reader - Write '1' to enable interrupt for event USBREMOVED"]
pub struct USBREMOVED_R(crate::FieldReader<bool, USBREMOVED_A>);
impl USBREMOVED_R {
    #[inline(always)]
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
#[doc = "Write '1' to enable interrupt for event USBREMOVED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREMOVED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<USBREMOVED_AW> for bool {
    #[inline(always)]
    fn from(variant: USBREMOVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREMOVED` writer - Write '1' to enable interrupt for event USBREMOVED"]
pub struct USBREMOVED_W<'a> {
    w: &'a mut W,
}
impl<'a> USBREMOVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBREMOVED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(USBREMOVED_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
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
#[doc = "Field `USBPWRRDY` reader - Write '1' to enable interrupt for event USBPWRRDY"]
pub struct USBPWRRDY_R(crate::FieldReader<bool, USBPWRRDY_A>);
impl USBPWRRDY_R {
    #[inline(always)]
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
#[doc = "Write '1' to enable interrupt for event USBPWRRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPWRRDY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<USBPWRRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPWRRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPWRRDY` writer - Write '1' to enable interrupt for event USBPWRRDY"]
pub struct USBPWRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPWRRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPWRRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(USBPWRRDY_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Write '1' to enable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&self) -> POFWARN_R {
        POFWARN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&self) -> SLEEPENTER_R {
        SLEEPENTER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&self) -> SLEEPEXIT_R {
        SLEEPEXIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&self) -> USBDETECTED_R {
        USBDETECTED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&self) -> USBREMOVED_R {
        USBREMOVED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event USBPWRRDY"]
    #[inline(always)]
    pub fn usbpwrrdy(&self) -> USBPWRRDY_R {
        USBPWRRDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Write '1' to enable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> POFWARN_W {
        POFWARN_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&mut self) -> SLEEPENTER_W {
        SLEEPENTER_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&mut self) -> SLEEPEXIT_W {
        SLEEPEXIT_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event USBDETECTED"]
    #[inline(always)]
    pub fn usbdetected(&mut self) -> USBDETECTED_W {
        USBDETECTED_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event USBREMOVED"]
    #[inline(always)]
    pub fn usbremoved(&mut self) -> USBREMOVED_W {
        USBREMOVED_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event USBPWRRDY"]
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
