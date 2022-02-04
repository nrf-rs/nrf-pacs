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
#[doc = "Controls USBD peripheral low-power mode during USB suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWER_A {
    #[doc = "0: Software must write this value to exit low power mode and before performing a remote wake-up"]
    FORCENORMAL = 0,
    #[doc = "1: Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LOWPOWER = 1,
}
impl From<LOWPOWER_A> for bool {
    #[inline(always)]
    fn from(variant: LOWPOWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWPOWER` reader - Controls USBD peripheral low-power mode during USB suspend"]
pub struct LOWPOWER_R(crate::FieldReader<bool, LOWPOWER_A>);
impl LOWPOWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOWPOWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWPOWER_A {
        match self.bits {
            false => LOWPOWER_A::FORCENORMAL,
            true => LOWPOWER_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `FORCENORMAL`"]
    #[inline(always)]
    pub fn is_force_normal(&self) -> bool {
        **self == LOWPOWER_A::FORCENORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        **self == LOWPOWER_A::LOWPOWER
    }
}
impl core::ops::Deref for LOWPOWER_R {
    type Target = crate::FieldReader<bool, LOWPOWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWPOWER` writer - Controls USBD peripheral low-power mode during USB suspend"]
pub struct LOWPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOWPOWER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    #[inline(always)]
    pub fn force_normal(self) -> &'a mut W {
        self.variant(LOWPOWER_A::FORCENORMAL)
    }
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(LOWPOWER_A::LOWPOWER)
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
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W {
        LOWPOWER_W { w: self }
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
