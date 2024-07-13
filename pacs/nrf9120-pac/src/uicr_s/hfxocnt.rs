#[doc = "Register `HFXOCNT` reader"]
pub struct R(crate::R<HFXOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFXOCNT` writer"]
pub struct W(crate::W<HFXOCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOCNT_SPEC>;
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
impl From<crate::W<HFXOCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFXOCNT` reader - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
pub type HFXOCNT_R = crate::FieldReader<u8, HFXOCNT_A>;
#[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXOCNT_A {
    #[doc = "0: Min debounce time = (0*64 us + 0.5 us)"]
    MIN_DEBOUNCE_TIME = 0,
    #[doc = "255: Max debounce time = (255*64 us + 0.5 us)"]
    MAX_DEBOUNCE_TIME = 255,
}
impl From<HFXOCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOCNT_A) -> Self {
        variant as _
    }
}
impl HFXOCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFXOCNT_A> {
        match self.bits {
            0 => Some(HFXOCNT_A::MIN_DEBOUNCE_TIME),
            255 => Some(HFXOCNT_A::MAX_DEBOUNCE_TIME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN_DEBOUNCE_TIME`"]
    #[inline(always)]
    pub fn is_min_debounce_time(&self) -> bool {
        *self == HFXOCNT_A::MIN_DEBOUNCE_TIME
    }
    #[doc = "Checks if the value of the field is `MAX_DEBOUNCE_TIME`"]
    #[inline(always)]
    pub fn is_max_debounce_time(&self) -> bool {
        *self == HFXOCNT_A::MAX_DEBOUNCE_TIME
    }
}
#[doc = "Field `HFXOCNT` writer - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
pub type HFXOCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HFXOCNT_SPEC, u8, HFXOCNT_A, 8, O>;
impl<'a, const O: u8> HFXOCNT_W<'a, O> {
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn min_debounce_time(self) -> &'a mut W {
        self.variant(HFXOCNT_A::MIN_DEBOUNCE_TIME)
    }
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn max_debounce_time(self) -> &'a mut W {
        self.variant(HFXOCNT_A::MAX_DEBOUNCE_TIME)
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub fn hfxocnt(&self) -> HFXOCNT_R {
        HFXOCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub fn hfxocnt(&mut self) -> HFXOCNT_W<0> {
        HFXOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFXO startup counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxocnt](index.html) module"]
pub struct HFXOCNT_SPEC;
impl crate::RegisterSpec for HFXOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxocnt::R](R) reader structure"]
impl crate::Readable for HFXOCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxocnt::W](W) writer structure"]
impl crate::Writable for HFXOCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFXOCNT to value 0xffff_ffff"]
impl crate::Resettable for HFXOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
