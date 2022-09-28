#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES` reader - ADC resolution."]
pub type RES_R = crate::FieldReader<u8, RES_A>;
#[doc = "ADC resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 8bit ADC resolution."]
    _8BIT = 0,
    #[doc = "1: 9bit ADC resolution."]
    _9BIT = 1,
    #[doc = "2: 10bit ADC resolution."]
    _10BIT = 2,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RES_A> {
        match self.bits {
            0 => Some(RES_A::_8BIT),
            1 => Some(RES_A::_9BIT),
            2 => Some(RES_A::_10BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_9BIT`"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == RES_A::_9BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == RES_A::_10BIT
    }
}
#[doc = "Field `RES` writer - ADC resolution."]
pub type RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, RES_A, 2, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    #[doc = "8bit ADC resolution."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "9bit ADC resolution."]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut W {
        self.variant(RES_A::_9BIT)
    }
    #[doc = "10bit ADC resolution."]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(RES_A::_10BIT)
    }
}
#[doc = "Field `INPSEL` reader - ADC input selection."]
pub type INPSEL_R = crate::FieldReader<u8, INPSEL_A>;
#[doc = "ADC input selection.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPSEL_A {
    #[doc = "0: Analog input specified by PSEL with no prescaling used as input for the conversion."]
    ANALOG_INPUT_NO_PRESCALING = 0,
    #[doc = "1: Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    ANALOG_INPUT_TWO_THIRDS_PRESCALING = 1,
    #[doc = "2: Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    ANALOG_INPUT_ONE_THIRD_PRESCALING = 2,
    #[doc = "5: Supply voltage with 2/3 prescaling used as input for the conversion."]
    SUPPLY_TWO_THIRDS_PRESCALING = 5,
    #[doc = "6: Supply voltage with 1/3 prescaling used as input for the conversion."]
    SUPPLY_ONE_THIRD_PRESCALING = 6,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
impl INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPSEL_A> {
        match self.bits {
            0 => Some(INPSEL_A::ANALOG_INPUT_NO_PRESCALING),
            1 => Some(INPSEL_A::ANALOG_INPUT_TWO_THIRDS_PRESCALING),
            2 => Some(INPSEL_A::ANALOG_INPUT_ONE_THIRD_PRESCALING),
            5 => Some(INPSEL_A::SUPPLY_TWO_THIRDS_PRESCALING),
            6 => Some(INPSEL_A::SUPPLY_ONE_THIRD_PRESCALING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT_NO_PRESCALING`"]
    #[inline(always)]
    pub fn is_analog_input_no_prescaling(&self) -> bool {
        *self == INPSEL_A::ANALOG_INPUT_NO_PRESCALING
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT_TWO_THIRDS_PRESCALING`"]
    #[inline(always)]
    pub fn is_analog_input_two_thirds_prescaling(&self) -> bool {
        *self == INPSEL_A::ANALOG_INPUT_TWO_THIRDS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT_ONE_THIRD_PRESCALING`"]
    #[inline(always)]
    pub fn is_analog_input_one_third_prescaling(&self) -> bool {
        *self == INPSEL_A::ANALOG_INPUT_ONE_THIRD_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_TWO_THIRDS_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_two_thirds_prescaling(&self) -> bool {
        *self == INPSEL_A::SUPPLY_TWO_THIRDS_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_ONE_THIRD_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        *self == INPSEL_A::SUPPLY_ONE_THIRD_PRESCALING
    }
}
#[doc = "Field `INPSEL` writer - ADC input selection."]
pub type INPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, INPSEL_A, 3, O>;
impl<'a, const O: u8> INPSEL_W<'a, O> {
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_no_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::ANALOG_INPUT_NO_PRESCALING)
    }
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_two_thirds_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::ANALOG_INPUT_TWO_THIRDS_PRESCALING)
    }
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_one_third_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::ANALOG_INPUT_ONE_THIRD_PRESCALING)
    }
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn supply_two_thirds_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::SUPPLY_TWO_THIRDS_PRESCALING)
    }
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn supply_one_third_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::SUPPLY_ONE_THIRD_PRESCALING)
    }
}
#[doc = "Field `REFSEL` reader - ADC reference selection."]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "ADC reference selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Use internal 1.2V bandgap voltage as reference for conversion."]
    VBG = 0,
    #[doc = "1: Use external source configured by EXTREFSEL as reference for conversion."]
    EXTERNAL = 1,
    #[doc = "2: Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    SUPPLY_ONE_HALF_PRESCALING = 2,
    #[doc = "3: Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    SUPPLY_ONE_THIRD_PRESCALING = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::VBG,
            1 => REFSEL_A::EXTERNAL,
            2 => REFSEL_A::SUPPLY_ONE_HALF_PRESCALING,
            3 => REFSEL_A::SUPPLY_ONE_THIRD_PRESCALING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VBG`"]
    #[inline(always)]
    pub fn is_vbg(&self) -> bool {
        *self == REFSEL_A::VBG
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == REFSEL_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `SUPPLY_ONE_HALF_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_half_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_ONE_HALF_PRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLY_ONE_THIRD_PRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLY_ONE_THIRD_PRESCALING
    }
}
#[doc = "Field `REFSEL` writer - ADC reference selection."]
pub type REFSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, REFSEL_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
    #[inline(always)]
    pub fn vbg(self) -> &'a mut W {
        self.variant(REFSEL_A::VBG)
    }
    #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(REFSEL_A::EXTERNAL)
    }
    #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    #[inline(always)]
    pub fn supply_one_half_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_ONE_HALF_PRESCALING)
    }
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    #[inline(always)]
    pub fn supply_one_third_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLY_ONE_THIRD_PRESCALING)
    }
}
#[doc = "Field `PSEL` reader - ADC analog pin selection."]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "ADC analog pin selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: Analog input pins disabled."]
    DISABLED = 0,
    #[doc = "1: Use analog input 0 as analog input."]
    ANALOG_INPUT0 = 1,
    #[doc = "2: Use analog input 1 as analog input."]
    ANALOG_INPUT1 = 2,
    #[doc = "4: Use analog input 2 as analog input."]
    ANALOG_INPUT2 = 4,
    #[doc = "8: Use analog input 3 as analog input."]
    ANALOG_INPUT3 = 8,
    #[doc = "16: Use analog input 4 as analog input."]
    ANALOG_INPUT4 = 16,
    #[doc = "32: Use analog input 5 as analog input."]
    ANALOG_INPUT5 = 32,
    #[doc = "64: Use analog input 6 as analog input."]
    ANALOG_INPUT6 = 64,
    #[doc = "128: Use analog input 7 as analog input."]
    ANALOG_INPUT7 = 128,
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
            0 => Some(PSEL_A::DISABLED),
            1 => Some(PSEL_A::ANALOG_INPUT0),
            2 => Some(PSEL_A::ANALOG_INPUT1),
            4 => Some(PSEL_A::ANALOG_INPUT2),
            8 => Some(PSEL_A::ANALOG_INPUT3),
            16 => Some(PSEL_A::ANALOG_INPUT4),
            32 => Some(PSEL_A::ANALOG_INPUT5),
            64 => Some(PSEL_A::ANALOG_INPUT6),
            128 => Some(PSEL_A::ANALOG_INPUT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSEL_A::DISABLED
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
    #[doc = "Checks if the value of the field is `ANALOG_INPUT4`"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT5`"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT6`"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT7`"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT7
    }
}
#[doc = "Field `PSEL` writer - ADC analog pin selection."]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, PSEL_A, 8, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "Analog input pins disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSEL_A::DISABLED)
    }
    #[doc = "Use analog input 0 as analog input."]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT0)
    }
    #[doc = "Use analog input 1 as analog input."]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT1)
    }
    #[doc = "Use analog input 2 as analog input."]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT2)
    }
    #[doc = "Use analog input 3 as analog input."]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT3)
    }
    #[doc = "Use analog input 4 as analog input."]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT4)
    }
    #[doc = "Use analog input 5 as analog input."]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT5)
    }
    #[doc = "Use analog input 6 as analog input."]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT6)
    }
    #[doc = "Use analog input 7 as analog input."]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT7)
    }
}
#[doc = "Field `EXTREFSEL` reader - ADC external reference pin selection."]
pub type EXTREFSEL_R = crate::FieldReader<u8, EXTREFSEL_A>;
#[doc = "ADC external reference pin selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTREFSEL_A {
    #[doc = "0: Analog external reference inputs disabled."]
    NONE = 0,
    #[doc = "1: Use analog reference 0 as reference."]
    ANALOG_REFERENCE0 = 1,
    #[doc = "2: Use analog reference 1 as reference."]
    ANALOG_REFERENCE1 = 2,
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
            0 => Some(EXTREFSEL_A::NONE),
            1 => Some(EXTREFSEL_A::ANALOG_REFERENCE0),
            2 => Some(EXTREFSEL_A::ANALOG_REFERENCE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EXTREFSEL_A::NONE
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
}
#[doc = "Field `EXTREFSEL` writer - ADC external reference pin selection."]
pub type EXTREFSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, EXTREFSEL_A, 2, O>;
impl<'a, const O: u8> EXTREFSEL_W<'a, O> {
    #[doc = "Analog external reference inputs disabled."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::NONE)
    }
    #[doc = "Use analog reference 0 as reference."]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOG_REFERENCE0)
    }
    #[doc = "Use analog reference 1 as reference."]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOG_REFERENCE1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<0> {
        RES_W::new(self)
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<2> {
        INPSEL_W::new(self)
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W<5> {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W<8> {
        PSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> EXTREFSEL_W<16> {
        EXTREFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x18"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
