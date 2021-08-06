#[doc = "Register `TXDATA` reader"]
pub struct R(crate::R<TXDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATA` writer"]
pub struct W(crate::W<TXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDATA_SPEC>;
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
impl From<crate::W<TXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - Data sent to debugger"]
pub struct TXDATA_R(crate::FieldReader<u32, u32>);
impl TXDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATA` writer - Data sent to debugger"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data sent to debugger"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data sent to debugger"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data sent from the CPU to the debugger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](index.html) module"]
pub struct TXDATA_SPEC;
impl crate::RegisterSpec for TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdata::R](R) reader structure"]
impl crate::Readable for TXDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdata::W](W) writer structure"]
impl crate::Writable for TXDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TXDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
