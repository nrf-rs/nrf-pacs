#[doc = "Register `LFCLKALWAYSRUN` reader"]
pub struct R(crate::R<LFCLKALWAYSRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKALWAYSRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKALWAYSRUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKALWAYSRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCLKALWAYSRUN` writer"]
pub struct W(crate::W<LFCLKALWAYSRUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCLKALWAYSRUN_SPEC>;
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
impl From<crate::W<LFCLKALWAYSRUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCLKALWAYSRUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALWAYSRUN` reader - Ensure clock is always running"]
pub type ALWAYSRUN_R = crate::BitReader<ALWAYSRUN_A>;
#[doc = "Ensure clock is always running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALWAYSRUN_A {
    #[doc = "0: Use automatic clock control"]
    AUTOMATIC = 0,
    #[doc = "1: Ensure clock is always running"]
    ALWAYS_RUN = 1,
}
impl From<ALWAYSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: ALWAYSRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALWAYSRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALWAYSRUN_A {
        match self.bits {
            false => ALWAYSRUN_A::AUTOMATIC,
            true => ALWAYSRUN_A::ALWAYS_RUN,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == ALWAYSRUN_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `ALWAYS_RUN`"]
    #[inline(always)]
    pub fn is_always_run(&self) -> bool {
        *self == ALWAYSRUN_A::ALWAYS_RUN
    }
}
#[doc = "Field `ALWAYSRUN` writer - Ensure clock is always running"]
pub type ALWAYSRUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LFCLKALWAYSRUN_SPEC, ALWAYSRUN_A, O>;
impl<'a, const O: u8> ALWAYSRUN_W<'a, O> {
    #[doc = "Use automatic clock control"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(ALWAYSRUN_A::AUTOMATIC)
    }
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub fn always_run(self) -> &'a mut W {
        self.variant(ALWAYSRUN_A::ALWAYS_RUN)
    }
}
impl R {
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&self) -> ALWAYSRUN_R {
        ALWAYSRUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&mut self) -> ALWAYSRUN_W<0> {
        ALWAYSRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic or manual control of LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkalwaysrun](index.html) module"]
pub struct LFCLKALWAYSRUN_SPEC;
impl crate::RegisterSpec for LFCLKALWAYSRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclkalwaysrun::R](R) reader structure"]
impl crate::Readable for LFCLKALWAYSRUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfclkalwaysrun::W](W) writer structure"]
impl crate::Writable for LFCLKALWAYSRUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFCLKALWAYSRUN to value 0"]
impl crate::Resettable for LFCLKALWAYSRUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
