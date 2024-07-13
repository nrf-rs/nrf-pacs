#[doc = "Register `ERASEPAGEPARTIALCFG` reader"]
pub struct R(crate::R<ERASEPAGEPARTIALCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEPAGEPARTIALCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEPAGEPARTIALCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEPAGEPARTIALCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEPAGEPARTIALCFG` writer"]
pub struct W(crate::W<ERASEPAGEPARTIALCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPAGEPARTIALCFG_SPEC>;
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
impl From<crate::W<ERASEPAGEPARTIALCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPAGEPARTIALCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DURATION` reader - Duration of the partial erase in milliseconds"]
pub type DURATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DURATION` writer - Duration of the partial erase in milliseconds"]
pub type DURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ERASEPAGEPARTIALCFG_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub fn duration(&self) -> DURATION_R {
        DURATION_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub fn duration(&mut self) -> DURATION_W<0> {
        DURATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for partial erase configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepagepartialcfg](index.html) module"]
pub struct ERASEPAGEPARTIALCFG_SPEC;
impl crate::RegisterSpec for ERASEPAGEPARTIALCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erasepagepartialcfg::R](R) reader structure"]
impl crate::Readable for ERASEPAGEPARTIALCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erasepagepartialcfg::W](W) writer structure"]
impl crate::Writable for ERASEPAGEPARTIALCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEPAGEPARTIALCFG to value 0x0a"]
impl crate::Resettable for ERASEPAGEPARTIALCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
