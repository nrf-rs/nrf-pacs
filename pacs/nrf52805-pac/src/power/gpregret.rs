#[doc = "Register `GPREGRET` reader"]
pub struct R(crate::R<GPREGRET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREGRET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPREGRET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPREGRET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPREGRET` writer"]
pub struct W(crate::W<GPREGRET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREGRET_SPEC>;
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
impl From<crate::W<GPREGRET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPREGRET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPREGRET` reader - General purpose retention register"]
pub struct GPREGRET_R(crate::FieldReader<u8, u8>);
impl GPREGRET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPREGRET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPREGRET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPREGRET` writer - General purpose retention register"]
pub struct GPREGRET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPREGRET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&self) -> GPREGRET_R {
        GPREGRET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&mut self) -> GPREGRET_W {
        GPREGRET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpregret](index.html) module"]
pub struct GPREGRET_SPEC;
impl crate::RegisterSpec for GPREGRET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpregret::R](R) reader structure"]
impl crate::Readable for GPREGRET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpregret::W](W) writer structure"]
impl crate::Writable for GPREGRET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPREGRET to value 0"]
impl crate::Resettable for GPREGRET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
