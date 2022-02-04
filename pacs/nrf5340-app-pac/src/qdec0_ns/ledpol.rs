#[doc = "Register `LEDPOL` reader"]
pub struct R(crate::R<LEDPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDPOL` writer"]
pub struct W(crate::W<LEDPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDPOL_SPEC>;
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
impl From<crate::W<LEDPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LED output pin polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDPOL_A {
    #[doc = "0: Led active on output pin low"]
    ACTIVELOW = 0,
    #[doc = "1: Led active on output pin high"]
    ACTIVEHIGH = 1,
}
impl From<LEDPOL_A> for bool {
    #[inline(always)]
    fn from(variant: LEDPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDPOL` reader - LED output pin polarity"]
pub struct LEDPOL_R(crate::FieldReader<bool, LEDPOL_A>);
impl LEDPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEDPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDPOL_A {
        match self.bits {
            false => LEDPOL_A::ACTIVELOW,
            true => LEDPOL_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == LEDPOL_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == LEDPOL_A::ACTIVEHIGH
    }
}
impl core::ops::Deref for LEDPOL_R {
    type Target = crate::FieldReader<bool, LEDPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDPOL` writer - LED output pin polarity"]
pub struct LEDPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Led active on output pin low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(LEDPOL_A::ACTIVELOW)
    }
    #[doc = "Led active on output pin high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(LEDPOL_A::ACTIVEHIGH)
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
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&self) -> LEDPOL_R {
        LEDPOL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&mut self) -> LEDPOL_W {
        LEDPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED output pin polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledpol](index.html) module"]
pub struct LEDPOL_SPEC;
impl crate::RegisterSpec for LEDPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledpol::R](R) reader structure"]
impl crate::Readable for LEDPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledpol::W](W) writer structure"]
impl crate::Writable for LEDPOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEDPOL to value 0"]
impl crate::Resettable for LEDPOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
