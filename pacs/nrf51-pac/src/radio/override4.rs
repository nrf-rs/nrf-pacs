#[doc = "Register `OVERRIDE4` reader"]
pub struct R(crate::R<OVERRIDE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERRIDE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERRIDE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERRIDE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERRIDE4` writer"]
pub struct W(crate::W<OVERRIDE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERRIDE4_SPEC>;
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
impl From<crate::W<OVERRIDE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVERRIDE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERRIDE4` reader - Trim value override 4."]
pub type OVERRIDE4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OVERRIDE4` writer - Trim value override 4."]
pub type OVERRIDE4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OVERRIDE4_SPEC, u32, u32, 28, O>;
#[doc = "Field `ENABLE` reader - Enable or disable override of default trim values."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable or disable override of default trim values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Override trim values disabled."]
    DISABLED = 0,
    #[doc = "1: Override trim values enabled."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - Enable or disable override of default trim values."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVERRIDE4_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Override trim values disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Override trim values enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&self) -> OVERRIDE4_R {
        OVERRIDE4_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&mut self) -> OVERRIDE4_W<0> {
        OVERRIDE4_W::new(self)
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim value override register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [override4](index.html) module"]
pub struct OVERRIDE4_SPEC;
impl crate::RegisterSpec for OVERRIDE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [override4::R](R) reader structure"]
impl crate::Readable for OVERRIDE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [override4::W](W) writer structure"]
impl crate::Writable for OVERRIDE4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVERRIDE4 to value 0"]
impl crate::Resettable for OVERRIDE4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
