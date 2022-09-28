#[doc = "Register `DST` reader"]
pub struct R(crate::R<DST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DST` writer"]
pub struct W(crate::W<DST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DST_SPEC>;
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
impl From<crate::W<DST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST` reader - Word-aligned RAM destination address."]
pub type DST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DST` writer - Word-aligned RAM destination address."]
pub type DST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Word-aligned RAM destination address."]
    #[inline(always)]
    pub fn dst(&self) -> DST_R {
        DST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Word-aligned RAM destination address."]
    #[inline(always)]
    pub fn dst(&mut self) -> DST_W<0> {
        DST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM destination address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst](index.html) module"]
pub struct DST_SPEC;
impl crate::RegisterSpec for DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst::R](R) reader structure"]
impl crate::Readable for DST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dst::W](W) writer structure"]
impl crate::Writable for DST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DST to value 0"]
impl crate::Resettable for DST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
