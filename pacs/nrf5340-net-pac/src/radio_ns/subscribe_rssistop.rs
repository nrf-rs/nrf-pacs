#[doc = "Register `SUBSCRIBE_RSSISTOP` reader"]
pub struct R(crate::R<SUBSCRIBE_RSSISTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSCRIBE_RSSISTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSCRIBE_RSSISTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSCRIBE_RSSISTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSCRIBE_RSSISTOP` writer"]
pub struct W(crate::W<SUBSCRIBE_RSSISTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSCRIBE_RSSISTOP_SPEC>;
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
impl From<crate::W<SUBSCRIBE_RSSISTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSCRIBE_RSSISTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIDX` reader - DPPI channel that task RSSISTOP will subscribe to"]
pub type CHIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHIDX` writer - DPPI channel that task RSSISTOP will subscribe to"]
pub type CHIDX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SUBSCRIBE_RSSISTOP_SPEC, u8, u8, 8, O>;
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Disable subscription"]
    DISABLED = 0,
    #[doc = "1: Enable subscription"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::ENABLED
    }
}
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUBSCRIBE_RSSISTOP_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Disable subscription"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Enable subscription"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - DPPI channel that task RSSISTOP will subscribe to"]
    #[inline(always)]
    pub fn chidx(&self) -> CHIDX_R {
        CHIDX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DPPI channel that task RSSISTOP will subscribe to"]
    #[inline(always)]
    pub fn chidx(&mut self) -> CHIDX_W<0> {
        CHIDX_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subscribe configuration for task RSSISTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_rssistop](index.html) module"]
pub struct SUBSCRIBE_RSSISTOP_SPEC;
impl crate::RegisterSpec for SUBSCRIBE_RSSISTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subscribe_rssistop::R](R) reader structure"]
impl crate::Readable for SUBSCRIBE_RSSISTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subscribe_rssistop::W](W) writer structure"]
impl crate::Writable for SUBSCRIBE_RSSISTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSCRIBE_RSSISTOP to value 0"]
impl crate::Resettable for SUBSCRIBE_RSSISTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
