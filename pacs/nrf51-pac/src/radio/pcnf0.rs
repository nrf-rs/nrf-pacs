#[doc = "Register `PCNF0` reader"]
pub struct R(crate::R<PCNF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNF0` writer"]
pub struct W(crate::W<PCNF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNF0_SPEC>;
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
impl From<crate::W<PCNF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFLEN` reader - Length of length field in number of bits. Decision point: START task."]
pub struct LFLEN_R(crate::FieldReader<u8, u8>);
impl LFLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LFLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFLEN` writer - Length of length field in number of bits. Decision point: START task."]
pub struct LFLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `S0LEN` reader - Length of S0 field in number of bytes. Decision point: START task."]
pub struct S0LEN_R(crate::FieldReader<bool, bool>);
impl S0LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S0LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S0LEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S0LEN` writer - Length of S0 field in number of bytes. Decision point: START task."]
pub struct S0LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S0LEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `S1LEN` reader - Length of S1 field in number of bits. Decision point: START task."]
pub struct S1LEN_R(crate::FieldReader<u8, u8>);
impl S1LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        S1LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S1LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S1LEN` writer - Length of S1 field in number of bits. Decision point: START task."]
pub struct S1LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S1LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&self) -> LFLEN_R {
        LFLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&self) -> S0LEN_R {
        S0LEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&self) -> S1LEN_R {
        S1LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&mut self) -> LFLEN_W {
        LFLEN_W { w: self }
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0LEN_W {
        S0LEN_W { w: self }
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1LEN_W {
        S1LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet configuration 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf0](index.html) module"]
pub struct PCNF0_SPEC;
impl crate::RegisterSpec for PCNF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnf0::R](R) reader structure"]
impl crate::Readable for PCNF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnf0::W](W) writer structure"]
impl crate::Writable for PCNF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCNF0 to value 0"]
impl crate::Resettable for PCNF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
