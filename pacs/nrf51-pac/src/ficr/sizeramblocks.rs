#[doc = "Register `SIZERAMBLOCKS` reader"]
pub struct R(crate::R<SIZERAMBLOCKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZERAMBLOCKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZERAMBLOCKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZERAMBLOCKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIZERAMBLOCKS` writer"]
pub struct W(crate::W<SIZERAMBLOCKS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZERAMBLOCKS_SPEC>;
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
impl From<crate::W<SIZERAMBLOCKS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZERAMBLOCKS_SPEC>) -> Self {
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
#[doc = "Size of RAM blocks in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sizeramblocks](index.html) module"]
pub struct SIZERAMBLOCKS_SPEC;
impl crate::RegisterSpec for SIZERAMBLOCKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sizeramblocks::R](R) reader structure"]
impl crate::Readable for SIZERAMBLOCKS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sizeramblocks::W](W) writer structure"]
impl crate::Writable for SIZERAMBLOCKS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIZERAMBLOCKS to value 0xffff_ffff"]
impl crate::Resettable for SIZERAMBLOCKS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
