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
#[doc = "Analog pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: AIN0 selected as analog input"]
    ANALOGINPUT0 = 0,
    #[doc = "1: AIN1 selected as analog input"]
    ANALOGINPUT1 = 1,
    #[doc = "2: AIN2 selected as analog input"]
    ANALOGINPUT2 = 2,
    #[doc = "3: AIN3 selected as analog input"]
    ANALOGINPUT3 = 3,
    #[doc = "4: AIN4 selected as analog input"]
    ANALOGINPUT4 = 4,
    #[doc = "5: AIN5 selected as analog input"]
    ANALOGINPUT5 = 5,
    #[doc = "6: AIN6 selected as analog input"]
    ANALOGINPUT6 = 6,
    #[doc = "7: AIN7 selected as analog input"]
    ANALOGINPUT7 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSEL` reader - Analog pin select"]
pub struct PSEL_R(crate::FieldReader<u8, PSEL_A>);
impl PSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::ANALOGINPUT0,
            1 => PSEL_A::ANALOGINPUT1,
            2 => PSEL_A::ANALOGINPUT2,
            3 => PSEL_A::ANALOGINPUT3,
            4 => PSEL_A::ANALOGINPUT4,
            5 => PSEL_A::ANALOGINPUT5,
            6 => PSEL_A::ANALOGINPUT6,
            7 => PSEL_A::ANALOGINPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT4`"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT5`"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT6`"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT7`"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        **self == PSEL_A::ANALOGINPUT7
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, PSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - Analog pin select"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AIN0 selected as analog input"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT0)
    }
    #[doc = "AIN1 selected as analog input"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT1)
    }
    #[doc = "AIN2 selected as analog input"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT2)
    }
    #[doc = "AIN3 selected as analog input"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT3)
    }
    #[doc = "AIN4 selected as analog input"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT4)
    }
    #[doc = "AIN5 selected as analog input"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT5)
    }
    #[doc = "AIN6 selected as analog input"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT6)
    }
    #[doc = "AIN7 selected as analog input"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
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
