#[doc = "Register `PSEL` reader"]
pub struct R(crate::R<PSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSEL` writer"]
pub struct W(crate::W<PSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSEL_SPEC>;
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
impl From<crate::W<PSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSEL` reader - Analog pin select"]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "Analog pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: AIN0 selected as analog input"]
    ANALOG_INPUT0 = 0,
    #[doc = "1: AIN1 selected as analog input"]
    ANALOG_INPUT1 = 1,
    #[doc = "2: AIN2 selected as analog input"]
    ANALOG_INPUT2 = 2,
    #[doc = "3: AIN3 selected as analog input"]
    ANALOG_INPUT3 = 3,
    #[doc = "7: VDDH/5 selected as analog input"]
    VDDH_DIV5 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSEL_A> {
        match self.bits {
            0 => Some(PSEL_A::ANALOG_INPUT0),
            1 => Some(PSEL_A::ANALOG_INPUT1),
            2 => Some(PSEL_A::ANALOG_INPUT2),
            3 => Some(PSEL_A::ANALOG_INPUT3),
            7 => Some(PSEL_A::VDDH_DIV5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT3
    }
    #[doc = "Checks if the value of the field is `VDDH_DIV5`"]
    #[inline(always)]
    pub fn is_vddh_div5(&self) -> bool {
        *self == PSEL_A::VDDH_DIV5
    }
}
#[doc = "Field `PSEL` writer - Analog pin select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSEL_SPEC, u8, PSEL_A, 3, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "AIN0 selected as analog input"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT0)
    }
    #[doc = "AIN1 selected as analog input"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT1)
    }
    #[doc = "AIN2 selected as analog input"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT2)
    }
    #[doc = "AIN3 selected as analog input"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT3)
    }
    #[doc = "VDDH/5 selected as analog input"]
    #[inline(always)]
    pub fn vddh_div5(self) -> &'a mut W {
        self.variant(PSEL_A::VDDH_DIV5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W<0> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psel](index.html) module"]
pub struct PSEL_SPEC;
impl crate::RegisterSpec for PSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psel::R](R) reader structure"]
impl crate::Readable for PSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psel::W](W) writer structure"]
impl crate::Writable for PSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSEL to value 0"]
impl crate::Resettable for PSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
