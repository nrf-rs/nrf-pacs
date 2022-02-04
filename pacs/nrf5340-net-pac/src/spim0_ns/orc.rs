#[doc = "Register `ORC` reader"]
pub struct R(crate::R<ORC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ORC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ORC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ORC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ORC` writer"]
pub struct W(crate::W<ORC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ORC_SPEC>;
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
impl From<crate::W<ORC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ORC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ORC` reader - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
pub struct ORC_R(crate::FieldReader<u8, u8>);
impl ORC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ORC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ORC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ORC` writer - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
pub struct ORC_W<'a> {
    w: &'a mut W,
}
impl<'a> ORC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
    #[inline(always)]
    pub fn orc(&self) -> ORC_R {
        ORC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
    #[inline(always)]
    pub fn orc(&mut self) -> ORC_W {
        ORC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](index.html) module"]
pub struct ORC_SPEC;
impl crate::RegisterSpec for ORC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [orc::R](R) reader structure"]
impl crate::Readable for ORC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [orc::W](W) writer structure"]
impl crate::Writable for ORC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ORC to value 0"]
impl crate::Resettable for ORC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
