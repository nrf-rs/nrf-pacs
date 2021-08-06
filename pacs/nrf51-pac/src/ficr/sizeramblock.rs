#[doc = "Register `SIZERAMBLOCK[%s]` reader"]
pub struct R(crate::R<SIZERAMBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZERAMBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZERAMBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZERAMBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIZERAMBLOCK[%s]` writer"]
pub struct W(crate::W<SIZERAMBLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZERAMBLOCK_SPEC>;
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
impl From<crate::W<SIZERAMBLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZERAMBLOCK_SPEC>) -> Self {
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
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sizeramblock](index.html) module"]
pub struct SIZERAMBLOCK_SPEC;
impl crate::RegisterSpec for SIZERAMBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sizeramblock::R](R) reader structure"]
impl crate::Readable for SIZERAMBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sizeramblock::W](W) writer structure"]
impl crate::Writable for SIZERAMBLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIZERAMBLOCK[%s]
to value 0xffff_ffff"]
impl crate::Resettable for SIZERAMBLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
