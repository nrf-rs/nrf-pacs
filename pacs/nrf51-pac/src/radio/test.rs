#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTCARRIER` reader - Constant carrier. Decision point: TXEN task."]
pub type CONSTCARRIER_R = crate::BitReader<CONSTCARRIER_A>;
#[doc = "Constant carrier. Decision point: TXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONSTCARRIER_A {
    #[doc = "0: Constant carrier disabled."]
    DISABLED = 0,
    #[doc = "1: Constant carrier enabled."]
    ENABLED = 1,
}
impl From<CONSTCARRIER_A> for bool {
    #[inline(always)]
    fn from(variant: CONSTCARRIER_A) -> Self {
        variant as u8 != 0
    }
}
impl CONSTCARRIER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONSTCARRIER_A {
        match self.bits {
            false => CONSTCARRIER_A::DISABLED,
            true => CONSTCARRIER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONSTCARRIER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONSTCARRIER_A::ENABLED
    }
}
#[doc = "Field `CONSTCARRIER` writer - Constant carrier. Decision point: TXEN task."]
pub type CONSTCARRIER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, CONSTCARRIER_A, O>;
impl<'a, const O: u8> CONSTCARRIER_W<'a, O> {
    #[doc = "Constant carrier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONSTCARRIER_A::DISABLED)
    }
    #[doc = "Constant carrier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONSTCARRIER_A::ENABLED)
    }
}
#[doc = "Field `PLLLOCK` reader - PLL lock. Decision point: TXEN or RXEN task."]
pub type PLLLOCK_R = crate::BitReader<PLLLOCK_A>;
#[doc = "PLL lock. Decision point: TXEN or RXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLOCK_A {
    #[doc = "0: PLL lock disabled."]
    DISABLED = 0,
    #[doc = "1: PLL lock enabled."]
    ENABLED = 1,
}
impl From<PLLLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOCK_A {
        match self.bits {
            false => PLLLOCK_A::DISABLED,
            true => PLLLOCK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLLOCK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLLOCK_A::ENABLED
    }
}
#[doc = "Field `PLLLOCK` writer - PLL lock. Decision point: TXEN or RXEN task."]
pub type PLLLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, PLLLOCK_A, O>;
impl<'a, const O: u8> PLLLOCK_W<'a, O> {
    #[doc = "PLL lock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLLOCK_A::DISABLED)
    }
    #[doc = "PLL lock enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLLOCK_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline(always)]
    pub fn constcarrier(&self) -> CONSTCARRIER_R {
        CONSTCARRIER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn plllock(&self) -> PLLLOCK_R {
        PLLLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline(always)]
    pub fn constcarrier(&mut self) -> CONSTCARRIER_W<0> {
        CONSTCARRIER_W::new(self)
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn plllock(&mut self) -> PLLLOCK_W<1> {
        PLLLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test features enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
