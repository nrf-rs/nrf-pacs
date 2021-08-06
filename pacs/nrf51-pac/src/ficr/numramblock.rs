#[doc = "Register `NUMRAMBLOCK` reader"]
pub struct R(crate::R<NUMRAMBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NUMRAMBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NUMRAMBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NUMRAMBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NUMRAMBLOCK` writer"]
pub struct W(crate::W<NUMRAMBLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NUMRAMBLOCK_SPEC>;
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
impl From<crate::W<NUMRAMBLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NUMRAMBLOCK_SPEC>) -> Self {
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
#[doc = "Number of individualy controllable RAM blocks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [numramblock](index.html) module"]
pub struct NUMRAMBLOCK_SPEC;
impl crate::RegisterSpec for NUMRAMBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [numramblock::R](R) reader structure"]
impl crate::Readable for NUMRAMBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [numramblock::W](W) writer structure"]
impl crate::Writable for NUMRAMBLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NUMRAMBLOCK to value 0xffff_ffff"]
impl crate::Resettable for NUMRAMBLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
