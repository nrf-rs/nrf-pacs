#[doc = "Register `BYPASS` reader"]
pub struct R(crate::R<BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYPASS` writer"]
pub struct W(crate::W<BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYPASS_SPEC>;
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
impl From<crate::W<BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable (use with crystal or low-swing external source)"]
    DISABLED = 0,
    #[doc = "1: Enable (use with rail-to-rail external source)"]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::ENABLED
    }
}
#[doc = "Field `BYPASS` writer - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYPASS_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "Disable (use with crystal or low-swing external source)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bypass](index.html) module"]
pub struct BYPASS_SPEC;
impl crate::RegisterSpec for BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bypass::R](R) reader structure"]
impl crate::Readable for BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bypass::W](W) writer structure"]
impl crate::Writable for BYPASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BYPASS to value 0"]
impl crate::Resettable for BYPASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
