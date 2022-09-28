#[doc = "Register `USBPULLUP` reader"]
pub struct R(crate::R<USBPULLUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPULLUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPULLUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPULLUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPULLUP` writer"]
pub struct W(crate::W<USBPULLUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPULLUP_SPEC>;
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
impl From<crate::W<USBPULLUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPULLUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONNECT` reader - Control of the USB pull-up on the D+ line"]
pub type CONNECT_R = crate::BitReader<CONNECT_A>;
#[doc = "Control of the USB pull-up on the D+ line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECT_A {
    #[doc = "0: Pull-up is disconnected"]
    DISABLED = 0,
    #[doc = "1: Pull-up is connected to D+"]
    ENABLED = 1,
}
impl From<CONNECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONNECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            false => CONNECT_A::DISABLED,
            true => CONNECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONNECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONNECT_A::ENABLED
    }
}
#[doc = "Field `CONNECT` writer - Control of the USB pull-up on the D+ line"]
pub type CONNECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPULLUP_SPEC, CONNECT_A, O>;
impl<'a, const O: u8> CONNECT_W<'a, O> {
    #[doc = "Pull-up is disconnected"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONNECT_A::DISABLED)
    }
    #[doc = "Pull-up is connected to D+"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONNECT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W<0> {
        CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control of the USB pull-up\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpullup](index.html) module"]
pub struct USBPULLUP_SPEC;
impl crate::RegisterSpec for USBPULLUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpullup::R](R) reader structure"]
impl crate::Readable for USBPULLUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpullup::W](W) writer structure"]
impl crate::Writable for USBPULLUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPULLUP to value 0"]
impl crate::Resettable for USBPULLUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
