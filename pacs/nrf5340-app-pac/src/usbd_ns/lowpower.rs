#[doc = "Register `LOWPOWER` reader"]
pub struct R(crate::R<LOWPOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOWPOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOWPOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOWPOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOWPOWER` writer"]
pub struct W(crate::W<LOWPOWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOWPOWER_SPEC>;
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
impl From<crate::W<LOWPOWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOWPOWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWPOWER` reader - Controls USBD peripheral low-power mode during USB suspend"]
pub type LOWPOWER_R = crate::BitReader<LOWPOWER_A>;
#[doc = "Controls USBD peripheral low-power mode during USB suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWER_A {
    #[doc = "0: Software must write this value to exit low power mode and before performing a remote wake-up"]
    FORCE_NORMAL = 0,
    #[doc = "1: Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LOW_POWER = 1,
}
impl From<LOWPOWER_A> for bool {
    #[inline(always)]
    fn from(variant: LOWPOWER_A) -> Self {
        variant as u8 != 0
    }
}
impl LOWPOWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWPOWER_A {
        match self.bits {
            false => LOWPOWER_A::FORCE_NORMAL,
            true => LOWPOWER_A::LOW_POWER,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_NORMAL`"]
    #[inline(always)]
    pub fn is_force_normal(&self) -> bool {
        *self == LOWPOWER_A::FORCE_NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LOWPOWER_A::LOW_POWER
    }
}
#[doc = "Field `LOWPOWER` writer - Controls USBD peripheral low-power mode during USB suspend"]
pub type LOWPOWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPOWER_SPEC, LOWPOWER_A, O>;
impl<'a, const O: u8> LOWPOWER_W<'a, O> {
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    #[inline(always)]
    pub fn force_normal(self) -> &'a mut W {
        self.variant(LOWPOWER_A::FORCE_NORMAL)
    }
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(LOWPOWER_A::LOW_POWER)
    }
}
impl R {
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W<0> {
        LOWPOWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls USBD peripheral low power mode during USB suspend\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpower](index.html) module"]
pub struct LOWPOWER_SPEC;
impl crate::RegisterSpec for LOWPOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lowpower::R](R) reader structure"]
impl crate::Readable for LOWPOWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lowpower::W](W) writer structure"]
impl crate::Writable for LOWPOWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOWPOWER to value 0"]
impl crate::Resettable for LOWPOWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
