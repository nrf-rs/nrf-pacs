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
#[doc = "Field `POFWARN` reader - Write '1' to disable interrupt for POFWARN event"]
pub type POFWARN_R = crate::BitReader<POFWARN_A>;
#[doc = "Write '1' to disable interrupt for POFWARN event\n\nValue on reset: 0"]
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
impl POFWARN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == POFWARN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POFWARN_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for POFWARN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARN_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<POFWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: POFWARN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` writer - Write '1' to disable interrupt for POFWARN event"]
pub type POFWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, POFWARN_AW, O>;
impl<'a, const O: u8> POFWARN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(POFWARN_AW::CLEAR)
    }
}
#[doc = "Field `SLEEPENTER` reader - Write '1' to disable interrupt for SLEEPENTER event"]
pub type SLEEPENTER_R = crate::BitReader<SLEEPENTER_A>;
#[doc = "Write '1' to disable interrupt for SLEEPENTER event\n\nValue on reset: 0"]
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
impl SLEEPENTER_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SLEEPENTER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEPENTER_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for SLEEPENTER event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPENTER_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SLEEPENTER_AW> for bool {
    #[inline(always)]
    fn from(variant: SLEEPENTER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPENTER` writer - Write '1' to disable interrupt for SLEEPENTER event"]
pub type SLEEPENTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SLEEPENTER_AW, O>;
impl<'a, const O: u8> SLEEPENTER_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SLEEPENTER_AW::CLEAR)
    }
}
#[doc = "Field `SLEEPEXIT` reader - Write '1' to disable interrupt for SLEEPEXIT event"]
pub type SLEEPEXIT_R = crate::BitReader<SLEEPEXIT_A>;
#[doc = "Write '1' to disable interrupt for SLEEPEXIT event\n\nValue on reset: 0"]
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
impl SLEEPEXIT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SLEEPEXIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEPEXIT_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for SLEEPEXIT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEXIT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SLEEPEXIT_AW> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEXIT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEXIT` writer - Write '1' to disable interrupt for SLEEPEXIT event"]
pub type SLEEPEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SLEEPEXIT_AW, O>;
impl<'a, const O: u8> SLEEPEXIT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SLEEPEXIT_AW::CLEAR)
    }
}
#[doc = "Field `USBDETECTED` reader - Write '1' to disable interrupt for USBDETECTED event"]
pub type USBDETECTED_R = crate::BitReader<USBDETECTED_A>;
#[doc = "Write '1' to disable interrupt for USBDETECTED event\n\nValue on reset: 0"]
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
impl USBDETECTED_R {
    #[doc = "Get enumerated values variant"]
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
        *self == USBDETECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBDETECTED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for USBDETECTED event\n\nValue on reset: 0"]
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
#[doc = "Field `USBDETECTED` writer - Write '1' to disable interrupt for USBDETECTED event"]
pub type USBDETECTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, USBDETECTED_AW, O>;
impl<'a, const O: u8> USBDETECTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBDETECTED_AW::CLEAR)
    }
}
#[doc = "Field `USBREMOVED` reader - Write '1' to disable interrupt for USBREMOVED event"]
pub type USBREMOVED_R = crate::BitReader<USBREMOVED_A>;
#[doc = "Write '1' to disable interrupt for USBREMOVED event\n\nValue on reset: 0"]
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
impl USBREMOVED_R {
    #[doc = "Get enumerated values variant"]
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
        *self == USBREMOVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBREMOVED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for USBREMOVED event\n\nValue on reset: 0"]
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
#[doc = "Field `USBREMOVED` writer - Write '1' to disable interrupt for USBREMOVED event"]
pub type USBREMOVED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, USBREMOVED_AW, O>;
impl<'a, const O: u8> USBREMOVED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBREMOVED_AW::CLEAR)
    }
}
#[doc = "Field `USBPWRRDY` reader - Write '1' to disable interrupt for USBPWRRDY event"]
pub type USBPWRRDY_R = crate::BitReader<USBPWRRDY_A>;
#[doc = "Write '1' to disable interrupt for USBPWRRDY event\n\nValue on reset: 0"]
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
impl USBPWRRDY_R {
    #[doc = "Get enumerated values variant"]
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
        *self == USBPWRRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBPWRRDY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for USBPWRRDY event\n\nValue on reset: 0"]
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
#[doc = "Field `USBPWRRDY` writer - Write '1' to disable interrupt for USBPWRRDY event"]
pub type USBPWRRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, USBPWRRDY_AW, O>;
impl<'a, const O: u8> USBPWRRDY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBPWRRDY_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 2 - Write '1' to disable interrupt for POFWARN event"]
    #[inline(always)]
    pub fn pofwarn(&self) -> POFWARN_R {
        POFWARN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for SLEEPENTER event"]
    #[inline(always)]
    pub fn sleepenter(&self) -> SLEEPENTER_R {
        SLEEPENTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for SLEEPEXIT event"]
    #[inline(always)]
    pub fn sleepexit(&self) -> SLEEPEXIT_R {
        SLEEPEXIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for USBDETECTED event"]
    #[inline(always)]
    pub fn usbdetected(&self) -> USBDETECTED_R {
        USBDETECTED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for USBREMOVED event"]
    #[inline(always)]
    pub fn usbremoved(&self) -> USBREMOVED_R {
        USBREMOVED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for USBPWRRDY event"]
    #[inline(always)]
    pub fn usbpwrrdy(&self) -> USBPWRRDY_R {
        USBPWRRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Write '1' to disable interrupt for POFWARN event"]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> POFWARN_W<2> {
        POFWARN_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for SLEEPENTER event"]
    #[inline(always)]
    pub fn sleepenter(&mut self) -> SLEEPENTER_W<5> {
        SLEEPENTER_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for SLEEPEXIT event"]
    #[inline(always)]
    pub fn sleepexit(&mut self) -> SLEEPEXIT_W<6> {
        SLEEPEXIT_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for USBDETECTED event"]
    #[inline(always)]
    pub fn usbdetected(&mut self) -> USBDETECTED_W<7> {
        USBDETECTED_W::new(self)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for USBREMOVED event"]
    #[inline(always)]
    pub fn usbremoved(&mut self) -> USBREMOVED_W<8> {
        USBREMOVED_W::new(self)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for USBPWRRDY event"]
    #[inline(always)]
    pub fn usbpwrrdy(&mut self) -> USBPWRRDY_W<9> {
        USBPWRRDY_W::new(self)
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
