#[doc = "Register `CSNPOL` reader"]
pub struct R(crate::R<CSNPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSNPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSNPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSNPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSNPOL` writer"]
pub struct W(crate::W<CSNPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSNPOL_SPEC>;
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
impl From<crate::W<CSNPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSNPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Polarity of CSN output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSNPOL_A {
    #[doc = "0: Active low (idle state high)"]
    LOW = 0,
    #[doc = "1: Active high (idle state low)"]
    HIGH = 1,
}
impl From<CSNPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CSNPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSNPOL` reader - Polarity of CSN output"]
pub struct CSNPOL_R(crate::FieldReader<bool, CSNPOL_A>);
impl CSNPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSNPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSNPOL_A {
        match self.bits {
            false => CSNPOL_A::LOW,
            true => CSNPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CSNPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CSNPOL_A::HIGH
    }
}
impl core::ops::Deref for CSNPOL_R {
    type Target = crate::FieldReader<bool, CSNPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSNPOL` writer - Polarity of CSN output"]
pub struct CSNPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSNPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSNPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active low (idle state high)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CSNPOL_A::LOW)
    }
    #[doc = "Active high (idle state low)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CSNPOL_A::HIGH)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(&self) -> CSNPOL_R {
        CSNPOL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(&mut self) -> CSNPOL_W {
        CSNPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polarity of CSN output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csnpol](index.html) module"]
pub struct CSNPOL_SPEC;
impl crate::RegisterSpec for CSNPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csnpol::R](R) reader structure"]
impl crate::Readable for CSNPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csnpol::W](W) writer structure"]
impl crate::Writable for CSNPOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSNPOL to value 0"]
impl crate::Resettable for CSNPOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
