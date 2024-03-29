#[doc = "Register `XTALFREQ` reader"]
pub struct R(crate::R<XTALFREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALFREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALFREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALFREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALFREQ` writer"]
pub struct W(crate::W<XTALFREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALFREQ_SPEC>;
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
impl From<crate::W<XTALFREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALFREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTALFREQ` reader - Reset value for CLOCK XTALFREQ register."]
pub type XTALFREQ_R = crate::FieldReader<u8, XTALFREQ_A>;
#[doc = "Reset value for CLOCK XTALFREQ register.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTALFREQ_A {
    #[doc = "255: 16MHz Xtal is used."]
    _16MHZ = 255,
    #[doc = "0: 32MHz Xtal is used."]
    _32MHZ = 0,
}
impl From<XTALFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: XTALFREQ_A) -> Self {
        variant as _
    }
}
impl XTALFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XTALFREQ_A> {
        match self.bits {
            255 => Some(XTALFREQ_A::_16MHZ),
            0 => Some(XTALFREQ_A::_32MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == XTALFREQ_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == XTALFREQ_A::_32MHZ
    }
}
#[doc = "Field `XTALFREQ` writer - Reset value for CLOCK XTALFREQ register."]
pub type XTALFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTALFREQ_SPEC, u8, XTALFREQ_A, 8, O>;
impl<'a, const O: u8> XTALFREQ_W<'a, O> {
    #[doc = "16MHz Xtal is used."]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(XTALFREQ_A::_16MHZ)
    }
    #[doc = "32MHz Xtal is used."]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(XTALFREQ_A::_32MHZ)
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset value for CLOCK XTALFREQ register."]
    #[inline(always)]
    pub fn xtalfreq(&self) -> XTALFREQ_R {
        XTALFREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reset value for CLOCK XTALFREQ register."]
    #[inline(always)]
    pub fn xtalfreq(&mut self) -> XTALFREQ_W<0> {
        XTALFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset value for CLOCK XTALFREQ register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalfreq](index.html) module"]
pub struct XTALFREQ_SPEC;
impl crate::RegisterSpec for XTALFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalfreq::R](R) reader structure"]
impl crate::Readable for XTALFREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalfreq::W](W) writer structure"]
impl crate::Writable for XTALFREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALFREQ to value 0xffff_ffff"]
impl crate::Resettable for XTALFREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
