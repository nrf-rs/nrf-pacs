#[doc = "Register `DFECTRL2` reader"]
pub struct R(crate::R<DFECTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFECTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFECTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFECTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFECTRL2` writer"]
pub struct W(crate::W<DFECTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFECTRL2_SPEC>;
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
impl From<crate::W<DFECTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFECTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSWITCHOFFSET` reader - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
pub type TSWITCHOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSWITCHOFFSET` writer - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
pub type TSWITCHOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFECTRL2_SPEC, u16, u16, 13, O>;
#[doc = "Field `TSAMPLEOFFSET` reader - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
pub type TSAMPLEOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSAMPLEOFFSET` writer - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
pub type TSAMPLEOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFECTRL2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&self) -> TSWITCHOFFSET_R {
        TSWITCHOFFSET_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
    #[inline(always)]
    pub fn tsampleoffset(&self) -> TSAMPLEOFFSET_R {
        TSAMPLEOFFSET_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&mut self) -> TSWITCHOFFSET_W<0> {
        TSWITCHOFFSET_W::new(self)
    }
    #[doc = "Bits 16:27 - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
    #[inline(always)]
    pub fn tsampleoffset(&mut self) -> TSAMPLEOFFSET_W<16> {
        TSAMPLEOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start offset for Direction finding\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfectrl2](index.html) module"]
pub struct DFECTRL2_SPEC;
impl crate::RegisterSpec for DFECTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfectrl2::R](R) reader structure"]
impl crate::Readable for DFECTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfectrl2::W](W) writer structure"]
impl crate::Writable for DFECTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFECTRL2 to value 0"]
impl crate::Resettable for DFECTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
