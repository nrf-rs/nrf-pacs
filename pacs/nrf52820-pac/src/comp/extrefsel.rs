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
#[doc = "Field `EXTREFSEL` reader - External analog reference select"]
pub type EXTREFSEL_R = crate::FieldReader<u8, EXTREFSEL_A>;
#[doc = "External analog reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTREFSEL_A {
    #[doc = "0: Use AIN0 as external analog reference"]
    ANALOG_REFERENCE0 = 0,
    #[doc = "1: Use AIN1 as external analog reference"]
    ANALOG_REFERENCE1 = 1,
    #[doc = "2: Use AIN2 as external analog reference"]
    ANALOG_REFERENCE2 = 2,
    #[doc = "3: Use AIN3 as external analog reference"]
    ANALOG_REFERENCE3 = 3,
}
impl From<EXTREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTREFSEL_A) -> Self {
        variant as _
    }
}
impl EXTREFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTREFSEL_A> {
        match self.bits {
            0 => Some(EXTREFSEL_A::ANALOG_REFERENCE0),
            1 => Some(EXTREFSEL_A::ANALOG_REFERENCE1),
            2 => Some(EXTREFSEL_A::ANALOG_REFERENCE2),
            3 => Some(EXTREFSEL_A::ANALOG_REFERENCE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_REFERENCE0`"]
    #[inline(always)]
    pub fn is_analog_reference0(&self) -> bool {
        *self == EXTREFSEL_A::ANALOG_REFERENCE0
    }
    #[doc = "Checks if the value of the field is `ANALOG_REFERENCE1`"]
    #[inline(always)]
    pub fn is_analog_reference1(&self) -> bool {
        *self == EXTREFSEL_A::ANALOG_REFERENCE1
    }
    #[doc = "Checks if the value of the field is `ANALOG_REFERENCE2`"]
    #[inline(always)]
    pub fn is_analog_reference2(&self) -> bool {
        *self == EXTREFSEL_A::ANALOG_REFERENCE2
    }
    #[doc = "Checks if the value of the field is `ANALOG_REFERENCE3`"]
    #[inline(always)]
    pub fn is_analog_reference3(&self) -> bool {
        *self == EXTREFSEL_A::ANALOG_REFERENCE3
    }
}
#[doc = "Field `EXTREFSEL` writer - External analog reference select"]
pub type EXTREFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTREFSEL_SPEC, u8, EXTREFSEL_A, 3, O>;
impl<'a, const O: u8> EXTREFSEL_W<'a, O> {
    #[doc = "Use AIN0 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOG_REFERENCE0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOG_REFERENCE1)
    }
    #[doc = "Use AIN2 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference2(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOG_REFERENCE2)
    }
    #[doc = "Use AIN3 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference3(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOG_REFERENCE3)
    }
}
impl R {
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> EXTREFSEL_W<0> {
        EXTREFSEL_W::new(self)
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
