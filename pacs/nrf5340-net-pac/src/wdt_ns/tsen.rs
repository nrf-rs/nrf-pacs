#[doc = "Register `TSEN` writer"]
pub struct W(crate::W<TSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEN_SPEC>;
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
impl From<crate::W<TSEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allow stopping WDT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TSEN_AW {
    #[doc = "1850885685: Value to allow stopping WDT"]
    ENABLE = 1850885685,
}
impl From<TSEN_AW> for u32 {
    #[inline(always)]
    fn from(variant: TSEN_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `TSEN` writer - Allow stopping WDT"]
pub type TSEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSEN_SPEC, u32, TSEN_AW, 32, O>;
impl<'a, const O: u8> TSEN_W<'a, O> {
    #[doc = "Value to allow stopping WDT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSEN_AW::ENABLE)
    }
}
impl W {
    #[doc = "Bits 0:31 - Allow stopping WDT"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<0> {
        TSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task stop enable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsen](index.html) module"]
pub struct TSEN_SPEC;
impl crate::RegisterSpec for TSEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tsen::W](W) writer structure"]
impl crate::Writable for TSEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSEN to value 0"]
impl crate::Resettable for TSEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
