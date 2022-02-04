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
#[repr(u8)]
pub enum EXTREFSEL_A {
    #[doc = "0: Use AIN0 as external analog reference"]
    ANALOGREFERENCE0 = 0,
    #[doc = "1: Use AIN1 as external analog reference"]
    ANALOGREFERENCE1 = 1,
    #[doc = "2: Use AIN2 as external analog reference"]
    ANALOGREFERENCE2 = 2,
    #[doc = "3: Use AIN3 as external analog reference"]
    ANALOGREFERENCE3 = 3,
    #[doc = "4: Use AIN4 as external analog reference"]
    ANALOGREFERENCE4 = 4,
    #[doc = "5: Use AIN5 as external analog reference"]
    ANALOGREFERENCE5 = 5,
    #[doc = "6: Use AIN6 as external analog reference"]
    ANALOGREFERENCE6 = 6,
    #[doc = "7: Use AIN7 as external analog reference"]
    ANALOGREFERENCE7 = 7,
}
impl From<EXTREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTREFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTREFSEL` reader - External analog reference select"]
pub struct EXTREFSEL_R(crate::FieldReader<u8, EXTREFSEL_A>);
impl EXTREFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTREFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTREFSEL_A {
        match self.bits {
            0 => EXTREFSEL_A::ANALOGREFERENCE0,
            1 => EXTREFSEL_A::ANALOGREFERENCE1,
            2 => EXTREFSEL_A::ANALOGREFERENCE2,
            3 => EXTREFSEL_A::ANALOGREFERENCE3,
            4 => EXTREFSEL_A::ANALOGREFERENCE4,
            5 => EXTREFSEL_A::ANALOGREFERENCE5,
            6 => EXTREFSEL_A::ANALOGREFERENCE6,
            7 => EXTREFSEL_A::ANALOGREFERENCE7,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE2`"]
    #[inline(always)]
    pub fn is_analog_reference2(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE2
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE3`"]
    #[inline(always)]
    pub fn is_analog_reference3(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE3
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE4`"]
    #[inline(always)]
    pub fn is_analog_reference4(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE4
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE5`"]
    #[inline(always)]
    pub fn is_analog_reference5(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE5
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE6`"]
    #[inline(always)]
    pub fn is_analog_reference6(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE6
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE7`"]
    #[inline(always)]
    pub fn is_analog_reference7(&self) -> bool {
        **self == EXTREFSEL_A::ANALOGREFERENCE7
    }
}
impl core::ops::Deref for EXTREFSEL_R {
    type Target = crate::FieldReader<u8, EXTREFSEL_A>;
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
        self.bits(variant.into())
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
    #[doc = "Use AIN2 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference2(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE2)
    }
    #[doc = "Use AIN3 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference3(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE3)
    }
    #[doc = "Use AIN4 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference4(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE4)
    }
    #[doc = "Use AIN5 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference5(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE5)
    }
    #[doc = "Use AIN6 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference6(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE6)
    }
    #[doc = "Use AIN7 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference7(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External analog reference select"]
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
