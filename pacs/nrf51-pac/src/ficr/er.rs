#[doc = "Register `ER[%s]` reader"]
pub struct R(crate::R<ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ER[%s]` writer"]
pub struct W(crate::W<ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ER_SPEC>;
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
impl From<crate::W<ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ER_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption root.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [er](index.html) module"]
pub struct ER_SPEC;
impl crate::RegisterSpec for ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [er::R](R) reader structure"]
impl crate::Readable for ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [er::W](W) writer structure"]
impl crate::Writable for ER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ER[%s]
to value 0xffff_ffff"]
impl crate::Resettable for ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
