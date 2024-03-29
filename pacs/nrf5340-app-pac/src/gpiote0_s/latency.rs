#[doc = "Register `LATENCY` reader"]
pub struct R(crate::R<LATENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LATENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LATENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LATENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LATENCY` writer"]
pub struct W(crate::W<LATENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LATENCY_SPEC>;
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
impl From<crate::W<LATENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LATENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - Latency setting"]
pub type LATENCY_R = crate::BitReader<LATENCY_A>;
#[doc = "Latency setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATENCY_A {
    #[doc = "0: Low power setting, for signals with minimum hold time tGPIOTE,HOLD,LP; refer to Electrical specification section"]
    LOW_POWER = 0,
    #[doc = "1: Low latency setting, for signals with minimum hold time tGPIOTE,HOLD,LL; refer to Electrical specification section"]
    LOW_LATENCY = 1,
}
impl From<LATENCY_A> for bool {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as u8 != 0
    }
}
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            false => LATENCY_A::LOW_POWER,
            true => LATENCY_A::LOW_LATENCY,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LATENCY_A::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `LOW_LATENCY`"]
    #[inline(always)]
    pub fn is_low_latency(&self) -> bool {
        *self == LATENCY_A::LOW_LATENCY
    }
}
#[doc = "Field `LATENCY` writer - Latency setting"]
pub type LATENCY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LATENCY_SPEC, LATENCY_A, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "Low power setting, for signals with minimum hold time tGPIOTE,HOLD,LP; refer to Electrical specification section"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(LATENCY_A::LOW_POWER)
    }
    #[doc = "Low latency setting, for signals with minimum hold time tGPIOTE,HOLD,LL; refer to Electrical specification section"]
    #[inline(always)]
    pub fn low_latency(self) -> &'a mut W {
        self.variant(LATENCY_A::LOW_LATENCY)
    }
}
impl R {
    #[doc = "Bit 0 - Latency setting"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latency setting"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Latency selection for Event mode (MODE=Event) with rising or falling edge detection on the pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latency](index.html) module"]
pub struct LATENCY_SPEC;
impl crate::RegisterSpec for LATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [latency::R](R) reader structure"]
impl crate::Readable for LATENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [latency::W](W) writer structure"]
impl crate::Writable for LATENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LATENCY to value 0x01"]
impl crate::Resettable for LATENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
