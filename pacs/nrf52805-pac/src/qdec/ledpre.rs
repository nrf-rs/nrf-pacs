#[doc = "Register `LEDPRE` reader"]
pub struct R(crate::R<LEDPRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDPRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDPRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDPRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDPRE` writer"]
pub struct W(crate::W<LEDPRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDPRE_SPEC>;
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
impl From<crate::W<LEDPRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDPRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEDPRE` reader - Period in us the LED is switched on prior to sampling"]
pub struct LEDPRE_R(crate::FieldReader<u16, u16>);
impl LEDPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LEDPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDPRE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDPRE` writer - Period in us the LED is switched on prior to sampling"]
pub struct LEDPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn ledpre(&self) -> LEDPRE_R {
        LEDPRE_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn ledpre(&mut self) -> LEDPRE_W {
        LEDPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time period the LED is switched ON prior to sampling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledpre](index.html) module"]
pub struct LEDPRE_SPEC;
impl crate::RegisterSpec for LEDPRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledpre::R](R) reader structure"]
impl crate::Readable for LEDPRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledpre::W](W) writer structure"]
impl crate::Writable for LEDPRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDPRE to value 0x10"]
impl crate::Resettable for LEDPRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
