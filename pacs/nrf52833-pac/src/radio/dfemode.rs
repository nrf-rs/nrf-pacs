#[doc = "Register `DFEMODE` reader"]
pub struct R(crate::R<DFEMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFEMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFEMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFEMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFEMODE` writer"]
pub struct W(crate::W<DFEMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFEMODE_SPEC>;
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
impl From<crate::W<DFEMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFEMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFEOPMODE` reader - Direction finding operation mode"]
pub type DFEOPMODE_R = crate::FieldReader<u8, DFEOPMODE_A>;
#[doc = "Direction finding operation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFEOPMODE_A {
    #[doc = "0: Direction finding mode disabled"]
    DISABLED = 0,
    #[doc = "2: Direction finding mode set to AoD"]
    AO_D = 2,
    #[doc = "3: Direction finding mode set to AoA"]
    AO_A = 3,
}
impl From<DFEOPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DFEOPMODE_A) -> Self {
        variant as _
    }
}
impl DFEOPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFEOPMODE_A> {
        match self.bits {
            0 => Some(DFEOPMODE_A::DISABLED),
            2 => Some(DFEOPMODE_A::AO_D),
            3 => Some(DFEOPMODE_A::AO_A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFEOPMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `AO_D`"]
    #[inline(always)]
    pub fn is_ao_d(&self) -> bool {
        *self == DFEOPMODE_A::AO_D
    }
    #[doc = "Checks if the value of the field is `AO_A`"]
    #[inline(always)]
    pub fn is_ao_a(&self) -> bool {
        *self == DFEOPMODE_A::AO_A
    }
}
#[doc = "Field `DFEOPMODE` writer - Direction finding operation mode"]
pub type DFEOPMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFEMODE_SPEC, u8, DFEOPMODE_A, 2, O>;
impl<'a, const O: u8> DFEOPMODE_W<'a, O> {
    #[doc = "Direction finding mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFEOPMODE_A::DISABLED)
    }
    #[doc = "Direction finding mode set to AoD"]
    #[inline(always)]
    pub fn ao_d(self) -> &'a mut W {
        self.variant(DFEOPMODE_A::AO_D)
    }
    #[doc = "Direction finding mode set to AoA"]
    #[inline(always)]
    pub fn ao_a(self) -> &'a mut W {
        self.variant(DFEOPMODE_A::AO_A)
    }
}
impl R {
    #[doc = "Bits 0:1 - Direction finding operation mode"]
    #[inline(always)]
    pub fn dfeopmode(&self) -> DFEOPMODE_R {
        DFEOPMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Direction finding operation mode"]
    #[inline(always)]
    pub fn dfeopmode(&mut self) -> DFEOPMODE_W<0> {
        DFEOPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfemode](index.html) module"]
pub struct DFEMODE_SPEC;
impl crate::RegisterSpec for DFEMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfemode::R](R) reader structure"]
impl crate::Readable for DFEMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfemode::W](W) writer structure"]
impl crate::Writable for DFEMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFEMODE to value 0"]
impl crate::Resettable for DFEMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
