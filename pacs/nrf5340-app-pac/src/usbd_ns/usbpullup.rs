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
#[doc = "Field `CONNECT` reader - Control of the USB pull-up on the D+ line"]
pub struct CONNECT_R(crate::FieldReader<bool, CONNECT_A>);
impl CONNECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONNECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CONNECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CONNECT_A::ENABLED
    }
}
impl core::ops::Deref for CONNECT_R {
    type Target = crate::FieldReader<bool, CONNECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONNECT` writer - Control of the USB pull-up on the D+ line"]
pub struct CONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONNECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
impl R {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W {
        CONNECT_W { w: self }
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
