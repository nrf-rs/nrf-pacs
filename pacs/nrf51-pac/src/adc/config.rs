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
#[doc = "Field `RES` reader - ADC resolution."]
pub struct RES_R(crate::FieldReader<u8, RES_A>);
impl RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_9BIT`"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        **self == RES_A::_9BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        **self == RES_A::_10BIT
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u8, RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES` writer - ADC resolution."]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "ADC input selection.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPSEL_A {
    #[doc = "0: Analog input specified by PSEL with no prescaling used as input for the conversion."]
    ANALOGINPUTNOPRESCALING = 0,
    #[doc = "1: Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    ANALOGINPUTTWOTHIRDSPRESCALING = 1,
    #[doc = "2: Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    ANALOGINPUTONETHIRDPRESCALING = 2,
    #[doc = "5: Supply voltage with 2/3 prescaling used as input for the conversion."]
    SUPPLYTWOTHIRDSPRESCALING = 5,
    #[doc = "6: Supply voltage with 1/3 prescaling used as input for the conversion."]
    SUPPLYONETHIRDPRESCALING = 6,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPSEL` reader - ADC input selection."]
pub struct INPSEL_R(crate::FieldReader<u8, INPSEL_A>);
impl INPSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPSEL_A> {
        match self.bits {
            0 => Some(INPSEL_A::ANALOGINPUTNOPRESCALING),
            1 => Some(INPSEL_A::ANALOGINPUTTWOTHIRDSPRESCALING),
            2 => Some(INPSEL_A::ANALOGINPUTONETHIRDPRESCALING),
            5 => Some(INPSEL_A::SUPPLYTWOTHIRDSPRESCALING),
            6 => Some(INPSEL_A::SUPPLYONETHIRDPRESCALING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUTNOPRESCALING`"]
    #[inline(always)]
    pub fn is_analog_input_no_prescaling(&self) -> bool {
        **self == INPSEL_A::ANALOGINPUTNOPRESCALING
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUTTWOTHIRDSPRESCALING`"]
    #[inline(always)]
    pub fn is_analog_input_two_thirds_prescaling(&self) -> bool {
        **self == INPSEL_A::ANALOGINPUTTWOTHIRDSPRESCALING
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUTONETHIRDPRESCALING`"]
    #[inline(always)]
    pub fn is_analog_input_one_third_prescaling(&self) -> bool {
        **self == INPSEL_A::ANALOGINPUTONETHIRDPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTWOTHIRDSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_two_thirds_prescaling(&self) -> bool {
        **self == INPSEL_A::SUPPLYTWOTHIRDSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYONETHIRDPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        **self == INPSEL_A::SUPPLYONETHIRDPRESCALING
    }
}
impl core::ops::Deref for INPSEL_R {
    type Target = crate::FieldReader<u8, INPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPSEL` writer - ADC input selection."]
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_no_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::ANALOGINPUTNOPRESCALING)
    }
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_two_thirds_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::ANALOGINPUTTWOTHIRDSPRESCALING)
    }
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_one_third_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::ANALOGINPUTONETHIRDPRESCALING)
    }
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn supply_two_thirds_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::SUPPLYTWOTHIRDSPRESCALING)
    }
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn supply_one_third_prescaling(self) -> &'a mut W {
        self.variant(INPSEL_A::SUPPLYONETHIRDPRESCALING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "ADC reference selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Use internal 1.2V bandgap voltage as reference for conversion."]
    VBG = 0,
    #[doc = "1: Use external source configured by EXTREFSEL as reference for conversion."]
    EXTERNAL = 1,
    #[doc = "2: Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    SUPPLYONEHALFPRESCALING = 2,
    #[doc = "3: Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    SUPPLYONETHIRDPRESCALING = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - ADC reference selection."]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::VBG,
            1 => REFSEL_A::EXTERNAL,
            2 => REFSEL_A::SUPPLYONEHALFPRESCALING,
            3 => REFSEL_A::SUPPLYONETHIRDPRESCALING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VBG`"]
    #[inline(always)]
    pub fn is_vbg(&self) -> bool {
        **self == REFSEL_A::VBG
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == REFSEL_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `SUPPLYONEHALFPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_half_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYONEHALFPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYONETHIRDPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        **self == REFSEL_A::SUPPLYONETHIRDPRESCALING
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - ADC reference selection."]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
        self.variant(REFSEL_A::SUPPLYONEHALFPRESCALING)
    }
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    #[inline(always)]
    pub fn supply_one_third_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYONETHIRDPRESCALING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "ADC analog pin selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: Analog input pins disabled."]
    DISABLED = 0,
    #[doc = "1: Use analog input 0 as analog input."]
    ANALOGINPUT0 = 1,
    #[doc = "2: Use analog input 1 as analog input."]
    ANALOGINPUT1 = 2,
    #[doc = "4: Use analog input 2 as analog input."]
    ANALOGINPUT2 = 4,
    #[doc = "8: Use analog input 3 as analog input."]
    ANALOGINPUT3 = 8,
    #[doc = "16: Use analog input 4 as analog input."]
    ANALOGINPUT4 = 16,
    #[doc = "32: Use analog input 5 as analog input."]
    ANALOGINPUT5 = 32,
    #[doc = "64: Use analog input 6 as analog input."]
    ANALOGINPUT6 = 64,
    #[doc = "128: Use analog input 7 as analog input."]
    ANALOGINPUT7 = 128,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSEL` reader - ADC analog pin selection."]
pub struct PSEL_R(crate::FieldReader<u8, PSEL_A>);
impl PSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSEL_A> {
        match self.bits {
            0 => Some(PSEL_A::DISABLED),
            1 => Some(PSEL_A::ANALOGINPUT0),
            2 => Some(PSEL_A::ANALOGINPUT1),
            4 => Some(PSEL_A::ANALOGINPUT2),
            8 => Some(PSEL_A::ANALOGINPUT3),
            16 => Some(PSEL_A::ANALOGINPUT4),
            32 => Some(PSEL_A::ANALOGINPUT5),
            64 => Some(PSEL_A::ANALOGINPUT6),
            128 => Some(PSEL_A::ANALOGINPUT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PSEL_A::DISABLED
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
#[doc = "Field `PSEL` writer - ADC analog pin selection."]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Analog input pins disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSEL_A::DISABLED)
    }
    #[doc = "Use analog input 0 as analog input."]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT0)
    }
    #[doc = "Use analog input 1 as analog input."]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT1)
    }
    #[doc = "Use analog input 2 as analog input."]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT2)
    }
    #[doc = "Use analog input 3 as analog input."]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT3)
    }
    #[doc = "Use analog input 4 as analog input."]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT4)
    }
    #[doc = "Use analog input 5 as analog input."]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT5)
    }
    #[doc = "Use analog input 6 as analog input."]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT6)
    }
    #[doc = "Use analog input 7 as analog input."]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "ADC external reference pin selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTREFSEL_A {
    #[doc = "0: Analog external reference inputs disabled."]
    NONE = 0,
    #[doc = "1: Use analog reference 0 as reference."]
    ANALOGREFERENCE0 = 1,
    #[doc = "2: Use analog reference 1 as reference."]
    ANALOGREFERENCE1 = 2,
}
impl From<EXTREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTREFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTREFSEL` reader - ADC external reference pin selection."]
pub struct EXTREFSEL_R(crate::FieldReader<u8, EXTREFSEL_A>);
impl EXTREFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTREFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTREFSEL_A> {
        match self.bits {
            0 => Some(EXTREFSEL_A::NONE),
            1 => Some(EXTREFSEL_A::ANALOGREFERENCE0),
            2 => Some(EXTREFSEL_A::ANALOGREFERENCE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == EXTREFSEL_A::NONE
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
    type Target = crate::FieldReader<u8, EXTREFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTREFSEL` writer - ADC external reference pin selection."]
pub struct EXTREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTREFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Analog external reference inputs disabled."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::NONE)
    }
    #[doc = "Use analog reference 0 as reference."]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE0)
    }
    #[doc = "Use analog reference 1 as reference."]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
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
