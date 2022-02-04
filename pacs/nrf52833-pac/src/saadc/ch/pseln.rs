#[doc = "Register `PSELN` reader"]
pub struct R(crate::R<PSELN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELN` writer"]
pub struct W(crate::W<PSELN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELN_SPEC>;
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
impl From<crate::W<PSELN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog negative input, enables differential channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSELN_A {
    #[doc = "0: Not connected"]
    NC = 0,
    #[doc = "1: AIN0"]
    ANALOGINPUT0 = 1,
    #[doc = "2: AIN1"]
    ANALOGINPUT1 = 2,
    #[doc = "3: AIN2"]
    ANALOGINPUT2 = 3,
    #[doc = "4: AIN3"]
    ANALOGINPUT3 = 4,
    #[doc = "5: AIN4"]
    ANALOGINPUT4 = 5,
    #[doc = "6: AIN5"]
    ANALOGINPUT5 = 6,
    #[doc = "7: AIN6"]
    ANALOGINPUT6 = 7,
    #[doc = "8: AIN7"]
    ANALOGINPUT7 = 8,
    #[doc = "9: VDD"]
    VDD = 9,
    #[doc = "13: VDDH/5"]
    VDDHDIV5 = 13,
}
impl From<PSELN_A> for u8 {
    #[inline(always)]
    fn from(variant: PSELN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSELN` reader - Analog negative input, enables differential channel"]
pub struct PSELN_R(crate::FieldReader<u8, PSELN_A>);
impl PSELN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSELN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELN_A> {
        match self.bits {
            0 => Some(PSELN_A::NC),
            1 => Some(PSELN_A::ANALOGINPUT0),
            2 => Some(PSELN_A::ANALOGINPUT1),
            3 => Some(PSELN_A::ANALOGINPUT2),
            4 => Some(PSELN_A::ANALOGINPUT3),
            5 => Some(PSELN_A::ANALOGINPUT4),
            6 => Some(PSELN_A::ANALOGINPUT5),
            7 => Some(PSELN_A::ANALOGINPUT6),
            8 => Some(PSELN_A::ANALOGINPUT7),
            9 => Some(PSELN_A::VDD),
            13 => Some(PSELN_A::VDDHDIV5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == PSELN_A::NC
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT4`"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT5`"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT6`"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT7`"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        **self == PSELN_A::ANALOGINPUT7
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        **self == PSELN_A::VDD
    }
    #[doc = "Checks if the value of the field is `VDDHDIV5`"]
    #[inline(always)]
    pub fn is_vddhdiv5(&self) -> bool {
        **self == PSELN_A::VDDHDIV5
    }
}
impl core::ops::Deref for PSELN_R {
    type Target = crate::FieldReader<u8, PSELN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSELN` writer - Analog negative input, enables differential channel"]
pub struct PSELN_W<'a> {
    w: &'a mut W,
}
impl<'a> PSELN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSELN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(PSELN_A::NC)
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT0)
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT1)
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT2)
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT3)
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT4)
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT5)
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT6)
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSELN_A::ANALOGINPUT7)
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(PSELN_A::VDD)
    }
    #[doc = "VDDH/5"]
    #[inline(always)]
    pub fn vddhdiv5(self) -> &'a mut W {
        self.variant(PSELN_A::VDDHDIV5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog negative input, enables differential channel"]
    #[inline(always)]
    pub fn pseln(&self) -> PSELN_R {
        PSELN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog negative input, enables differential channel"]
    #[inline(always)]
    pub fn pseln(&mut self) -> PSELN_W {
        PSELN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pseln](index.html) module"]
pub struct PSELN_SPEC;
impl crate::RegisterSpec for PSELN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pseln::R](R) reader structure"]
impl crate::Readable for PSELN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pseln::W](W) writer structure"]
impl crate::Writable for PSELN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELN to value 0"]
impl crate::Resettable for PSELN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
