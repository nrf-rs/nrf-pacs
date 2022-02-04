#[doc = "Register `FRAMEDELAYMIN` reader"]
pub struct R(crate::R<FRAMEDELAYMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEDELAYMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEDELAYMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEDELAYMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEDELAYMIN` writer"]
pub struct W(crate::W<FRAMEDELAYMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEDELAYMIN_SPEC>;
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
impl From<crate::W<FRAMEDELAYMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEDELAYMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYMIN` reader - Minimum frame delay in number of 13.56 MHz clocks"]
pub struct FRAMEDELAYMIN_R(crate::FieldReader<u16, u16>);
impl FRAMEDELAYMIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMEDELAYMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMEDELAYMIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMEDELAYMIN` writer - Minimum frame delay in number of 13.56 MHz clocks"]
pub struct FRAMEDELAYMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymin(&self) -> FRAMEDELAYMIN_R {
        FRAMEDELAYMIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymin(&mut self) -> FRAMEDELAYMIN_W {
        FRAMEDELAYMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Minimum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymin](index.html) module"]
pub struct FRAMEDELAYMIN_SPEC;
impl crate::RegisterSpec for FRAMEDELAYMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framedelaymin::R](R) reader structure"]
impl crate::Readable for FRAMEDELAYMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framedelaymin::W](W) writer structure"]
impl crate::Writable for FRAMEDELAYMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMEDELAYMIN to value 0x0480"]
impl crate::Resettable for FRAMEDELAYMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0480
    }
}
