#[doc = "Register `NFCID1_2ND_LAST` reader"]
pub struct R(crate::R<NFCID1_2ND_LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCID1_2ND_LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCID1_2ND_LAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCID1_2ND_LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCID1_2ND_LAST` writer"]
pub struct W(crate::W<NFCID1_2ND_LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCID1_2ND_LAST_SPEC>;
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
impl From<crate::W<NFCID1_2ND_LAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCID1_2ND_LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NFCID1_V` reader - NFCID1 byte V"]
pub struct NFCID1_V_R(crate::FieldReader<u8, u8>);
impl NFCID1_V_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_V_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_V` writer - NFCID1 byte V"]
pub struct NFCID1_V_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_V_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `NFCID1_U` reader - NFCID1 byte U"]
pub struct NFCID1_U_R(crate::FieldReader<u8, u8>);
impl NFCID1_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_U` writer - NFCID1 byte U"]
pub struct NFCID1_U_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `NFCID1_T` reader - NFCID1 byte T"]
pub struct NFCID1_T_R(crate::FieldReader<u8, u8>);
impl NFCID1_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_T` writer - NFCID1 byte T"]
pub struct NFCID1_T_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn nfcid1_v(&self) -> NFCID1_V_R {
        NFCID1_V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn nfcid1_u(&self) -> NFCID1_U_R {
        NFCID1_U_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn nfcid1_t(&self) -> NFCID1_T_R {
        NFCID1_T_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn nfcid1_v(&mut self) -> NFCID1_V_W {
        NFCID1_V_W { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn nfcid1_u(&mut self) -> NFCID1_U_W {
        NFCID1_U_W { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn nfcid1_t(&mut self) -> NFCID1_T_W {
        NFCID1_T_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_2nd_last](index.html) module"]
pub struct NFCID1_2ND_LAST_SPEC;
impl crate::RegisterSpec for NFCID1_2ND_LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcid1_2nd_last::R](R) reader structure"]
impl crate::Readable for NFCID1_2ND_LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcid1_2nd_last::W](W) writer structure"]
impl crate::Writable for NFCID1_2ND_LAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NFCID1_2ND_LAST to value 0"]
impl crate::Resettable for NFCID1_2ND_LAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
