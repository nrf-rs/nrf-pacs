#[doc = "Register `ENDDELAY` reader"]
pub struct R(crate::R<ENDDELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDDELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDDELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDDELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDDELAY` writer"]
pub struct W(crate::W<ENDDELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDDELAY_SPEC>;
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
impl From<crate::W<ENDDELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDDELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Time added after the sequence in PWM periods"]
pub struct CNT_R(crate::FieldReader<u32, u32>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Time added after the sequence in PWM periods"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Time added after the sequence in PWM periods"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time added after the sequence in PWM periods"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Time added after the sequence\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enddelay](index.html) module"]
pub struct ENDDELAY_SPEC;
impl crate::RegisterSpec for ENDDELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enddelay::R](R) reader structure"]
impl crate::Readable for ENDDELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enddelay::W](W) writer structure"]
impl crate::Writable for ENDDELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDDELAY to value 0"]
impl crate::Resettable for ENDDELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
