#[doc = "Register `PSELCSN` reader"]
pub struct R(crate::R<PSELCSN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELCSN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELCSN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELCSN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELCSN` writer"]
pub struct W(crate::W<PSELCSN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELCSN_SPEC>;
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
impl From<crate::W<PSELCSN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELCSN_SPEC>) -> Self {
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
#[doc = "Pin select for CSN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselcsn](index.html) module"]
pub struct PSELCSN_SPEC;
impl crate::RegisterSpec for PSELCSN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselcsn::R](R) reader structure"]
impl crate::Readable for PSELCSN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselcsn::W](W) writer structure"]
impl crate::Writable for PSELCSN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELCSN to value 0xffff_ffff"]
impl crate::Resettable for PSELCSN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
