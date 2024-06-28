#[doc = "Register `XOSC32M` reader"]
pub struct R(crate::R<XOSC32M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC32M` writer"]
pub struct W(crate::W<XOSC32M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC32M_SPEC>;
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
impl From<crate::W<XOSC32M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC32M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL` reader - Pierce current DAC control signals"]
pub type CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRL` writer - Pierce current DAC control signals"]
pub type CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XOSC32M_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Pierce current DAC control signals"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pierce current DAC control signals"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W<0> {
        CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32m](index.html) module"]
pub struct XOSC32M_SPEC;
impl crate::RegisterSpec for XOSC32M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xosc32m::R](R) reader structure"]
impl crate::Readable for XOSC32M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc32m::W](W) writer structure"]
impl crate::Writable for XOSC32M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSC32M to value 0xffff_ffcf"]
impl crate::Resettable for XOSC32M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffcf
    }
}
