#[doc = "Register `PSELP` reader"]
pub struct R(crate::R<PSELP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELP` writer"]
pub struct W(crate::W<PSELP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELP_SPEC>;
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
impl From<crate::W<PSELP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSELP` reader - Analog positive input channel"]
pub type PSELP_R = crate::FieldReader<u8, PSELP_A>;
#[doc = "Analog positive input channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSELP_A {
    #[doc = "0: Not connected"]
    NC = 0,
    #[doc = "1: AIN0"]
    ANALOG_INPUT0 = 1,
    #[doc = "2: AIN1"]
    ANALOG_INPUT1 = 2,
    #[doc = "3: AIN2"]
    ANALOG_INPUT2 = 3,
    #[doc = "4: AIN3"]
    ANALOG_INPUT3 = 4,
    #[doc = "5: AIN4"]
    ANALOG_INPUT4 = 5,
    #[doc = "6: AIN5"]
    ANALOG_INPUT5 = 6,
    #[doc = "7: AIN6"]
    ANALOG_INPUT6 = 7,
    #[doc = "8: AIN7"]
    ANALOG_INPUT7 = 8,
    #[doc = "9: VDD"]
    VDD = 9,
    #[doc = "13: VDDH/5"]
    VDDHDIV5 = 13,
}
impl From<PSELP_A> for u8 {
    #[inline(always)]
    fn from(variant: PSELP_A) -> Self {
        variant as _
    }
}
impl PSELP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELP_A> {
        match self.bits {
            0 => Some(PSELP_A::NC),
            1 => Some(PSELP_A::ANALOG_INPUT0),
            2 => Some(PSELP_A::ANALOG_INPUT1),
            3 => Some(PSELP_A::ANALOG_INPUT2),
            4 => Some(PSELP_A::ANALOG_INPUT3),
            5 => Some(PSELP_A::ANALOG_INPUT4),
            6 => Some(PSELP_A::ANALOG_INPUT5),
            7 => Some(PSELP_A::ANALOG_INPUT6),
            8 => Some(PSELP_A::ANALOG_INPUT7),
            9 => Some(PSELP_A::VDD),
            13 => Some(PSELP_A::VDDHDIV5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == PSELP_A::NC
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT4`"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT5`"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT6`"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT7`"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == PSELP_A::ANALOG_INPUT7
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == PSELP_A::VDD
    }
    #[doc = "Checks if the value of the field is `VDDHDIV5`"]
    #[inline(always)]
    pub fn is_vddhdiv5(&self) -> bool {
        *self == PSELP_A::VDDHDIV5
    }
}
#[doc = "Field `PSELP` writer - Analog positive input channel"]
pub type PSELP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSELP_SPEC, u8, PSELP_A, 5, O>;
impl<'a, const O: u8> PSELP_W<'a, O> {
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(PSELP_A::NC)
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT0)
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT1)
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT2)
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT3)
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT4)
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT5)
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT6)
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSELP_A::ANALOG_INPUT7)
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(PSELP_A::VDD)
    }
    #[doc = "VDDH/5"]
    #[inline(always)]
    pub fn vddhdiv5(self) -> &'a mut W {
        self.variant(PSELP_A::VDDHDIV5)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog positive input channel"]
    #[inline(always)]
    pub fn pselp(&self) -> PSELP_R {
        PSELP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog positive input channel"]
    #[inline(always)]
    pub fn pselp(&mut self) -> PSELP_W<0> {
        PSELP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselp](index.html) module"]
pub struct PSELP_SPEC;
impl crate::RegisterSpec for PSELP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselp::R](R) reader structure"]
impl crate::Readable for PSELP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselp::W](W) writer structure"]
impl crate::Writable for PSELP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELP to value 0"]
impl crate::Resettable for PSELP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
