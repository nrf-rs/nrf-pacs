#[doc = "Register `MCLKCONFIG` reader"]
pub struct R(crate::R<MCLKCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKCONFIG` writer"]
pub struct W(crate::W<MCLKCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCONFIG_SPEC>;
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
impl From<crate::W<MCLKCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - Master clock source selection"]
pub type SRC_R = crate::BitReader<SRC_A>;
#[doc = "Master clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: 32 MHz peripheral clock"]
    PCLK32M = 0,
    #[doc = "1: Audio PLL clock"]
    ACLK = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::PCLK32M,
            true => SRC_A::ACLK,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK32M`"]
    #[inline(always)]
    pub fn is_pclk32m(&self) -> bool {
        *self == SRC_A::PCLK32M
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == SRC_A::ACLK
    }
}
#[doc = "Field `SRC` writer - Master clock source selection"]
pub type SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCLKCONFIG_SPEC, SRC_A, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "32 MHz peripheral clock"]
    #[inline(always)]
    pub fn pclk32m(self) -> &'a mut W {
        self.variant(SRC_A::PCLK32M)
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(SRC_A::ACLK)
    }
}
impl R {
    #[doc = "Bit 0 - Master clock source selection"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock source selection"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master clock generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkconfig](index.html) module"]
pub struct MCLKCONFIG_SPEC;
impl crate::RegisterSpec for MCLKCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkconfig::R](R) reader structure"]
impl crate::Readable for MCLKCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkconfig::W](W) writer structure"]
impl crate::Writable for MCLKCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKCONFIG to value 0"]
impl crate::Resettable for MCLKCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
