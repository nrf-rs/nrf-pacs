#[doc = "Register `EXTREFSEL` reader"]
pub struct R(crate::R<EXTREFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTREFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTREFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTREFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTREFSEL` writer"]
pub struct W(crate::W<EXTREFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTREFSEL_SPEC>;
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
impl From<crate::W<EXTREFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTREFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External analog reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREFSEL_A {
    #[doc = "0: Use AIN0 as external analog reference"]
    ANALOGREFERENCE0 = 0,
    #[doc = "1: Use AIN1 as external analog reference"]
    ANALOGREFERENCE1 = 1,
}
impl From<EXTREFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTREFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTREFSEL` reader - External analog reference select"]
pub struct EXTREFSEL_R(crate::FieldReader<bool, EXTREFSEL_A>);
impl EXTREFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTREFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTREFSEL_A {
        match self.bits {
            false => EXTREFSEL_A::ANALOGREFERENCE0,
            true => EXTREFSEL_A::ANALOGREFERENCE1,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE0`"]
    #[inline(always)]
    pub fn is_analog_reference0(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE0
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE1`"]
    #[inline(always)]
    pub fn is_analog_reference1(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE1
    }
}
impl core::ops::Deref for EXTREFSEL_R {
    type Target = crate::FieldReader<bool, EXTREFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTREFSEL` writer - External analog reference select"]
pub struct EXTREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTREFSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use AIN0 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE1)
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
    #[doc = "Bit 0 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> EXTREFSEL_W {
        EXTREFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External reference select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extrefsel](index.html) module"]
pub struct EXTREFSEL_SPEC;
impl crate::RegisterSpec for EXTREFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extrefsel::R](R) reader structure"]
impl crate::Readable for EXTREFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extrefsel::W](W) writer structure"]
impl crate::Writable for EXTREFSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTREFSEL to value 0"]
impl crate::Resettable for EXTREFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
