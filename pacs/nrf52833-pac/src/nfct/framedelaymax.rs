#[doc = "Register `FRAMEDELAYMAX` reader"]
pub struct R(crate::R<FRAMEDELAYMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEDELAYMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEDELAYMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEDELAYMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEDELAYMAX` writer"]
pub struct W(crate::W<FRAMEDELAYMAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEDELAYMAX_SPEC>;
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
impl From<crate::W<FRAMEDELAYMAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEDELAYMAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYMAX` reader - Maximum frame delay in number of 13.56 MHz clocks"]
pub struct FRAMEDELAYMAX_R(crate::FieldReader<u32, u32>);
impl FRAMEDELAYMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FRAMEDELAYMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMEDELAYMAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMEDELAYMAX` writer - Maximum frame delay in number of 13.56 MHz clocks"]
pub struct FRAMEDELAYMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Maximum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymax(&self) -> FRAMEDELAYMAX_R {
        FRAMEDELAYMAX_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Maximum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymax(&mut self) -> FRAMEDELAYMAX_W {
        FRAMEDELAYMAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymax](index.html) module"]
pub struct FRAMEDELAYMAX_SPEC;
impl crate::RegisterSpec for FRAMEDELAYMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framedelaymax::R](R) reader structure"]
impl crate::Readable for FRAMEDELAYMAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framedelaymax::W](W) writer structure"]
impl crate::Writable for FRAMEDELAYMAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMEDELAYMAX to value 0x1000"]
impl crate::Resettable for FRAMEDELAYMAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
