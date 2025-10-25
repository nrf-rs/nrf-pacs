#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "ADC resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Res {
    #[doc = "0: 8bit ADC resolution."]
    _8bit = 0,
    #[doc = "1: 9bit ADC resolution."]
    _9bit = 1,
    #[doc = "2: 10bit ADC resolution."]
    _10bit = 2,
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(variant: Res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Res {
    type Ux = u8;
}
impl crate::IsEnum for Res {}
#[doc = "Field `RES` reader - ADC resolution."]
pub type ResR = crate::FieldReader<Res>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Res> {
        match self.bits {
            0 => Some(Res::_8bit),
            1 => Some(Res::_9bit),
            2 => Some(Res::_10bit),
            _ => None,
        }
    }
    #[doc = "8bit ADC resolution."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Res::_8bit
    }
    #[doc = "9bit ADC resolution."]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Res::_9bit
    }
    #[doc = "10bit ADC resolution."]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Res::_10bit
    }
}
#[doc = "Field `RES` writer - ADC resolution."]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, Res>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8bit ADC resolution."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Res::_8bit)
    }
    #[doc = "9bit ADC resolution."]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Res::_9bit)
    }
    #[doc = "10bit ADC resolution."]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Res::_10bit)
    }
}
#[doc = "ADC input selection.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inpsel {
    #[doc = "0: Analog input specified by PSEL with no prescaling used as input for the conversion."]
    AnalogInputNoPrescaling = 0,
    #[doc = "1: Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    AnalogInputTwoThirdsPrescaling = 1,
    #[doc = "2: Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    AnalogInputOneThirdPrescaling = 2,
    #[doc = "5: Supply voltage with 2/3 prescaling used as input for the conversion."]
    SupplyTwoThirdsPrescaling = 5,
    #[doc = "6: Supply voltage with 1/3 prescaling used as input for the conversion."]
    SupplyOneThirdPrescaling = 6,
}
impl From<Inpsel> for u8 {
    #[inline(always)]
    fn from(variant: Inpsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inpsel {
    type Ux = u8;
}
impl crate::IsEnum for Inpsel {}
#[doc = "Field `INPSEL` reader - ADC input selection."]
pub type InpselR = crate::FieldReader<Inpsel>;
impl InpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inpsel> {
        match self.bits {
            0 => Some(Inpsel::AnalogInputNoPrescaling),
            1 => Some(Inpsel::AnalogInputTwoThirdsPrescaling),
            2 => Some(Inpsel::AnalogInputOneThirdPrescaling),
            5 => Some(Inpsel::SupplyTwoThirdsPrescaling),
            6 => Some(Inpsel::SupplyOneThirdPrescaling),
            _ => None,
        }
    }
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn is_analog_input_no_prescaling(&self) -> bool {
        *self == Inpsel::AnalogInputNoPrescaling
    }
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn is_analog_input_two_thirds_prescaling(&self) -> bool {
        *self == Inpsel::AnalogInputTwoThirdsPrescaling
    }
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn is_analog_input_one_third_prescaling(&self) -> bool {
        *self == Inpsel::AnalogInputOneThirdPrescaling
    }
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn is_supply_two_thirds_prescaling(&self) -> bool {
        *self == Inpsel::SupplyTwoThirdsPrescaling
    }
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        *self == Inpsel::SupplyOneThirdPrescaling
    }
}
#[doc = "Field `INPSEL` writer - ADC input selection."]
pub type InpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Inpsel>;
impl<'a, REG> InpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog input specified by PSEL with no prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_no_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::AnalogInputNoPrescaling)
    }
    #[doc = "Analog input specified by PSEL with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_two_thirds_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::AnalogInputTwoThirdsPrescaling)
    }
    #[doc = "Analog input specified by PSEL with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn analog_input_one_third_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::AnalogInputOneThirdPrescaling)
    }
    #[doc = "Supply voltage with 2/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn supply_two_thirds_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::SupplyTwoThirdsPrescaling)
    }
    #[doc = "Supply voltage with 1/3 prescaling used as input for the conversion."]
    #[inline(always)]
    pub fn supply_one_third_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Inpsel::SupplyOneThirdPrescaling)
    }
}
#[doc = "ADC reference selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: Use internal 1.2V bandgap voltage as reference for conversion."]
    Vbg = 0,
    #[doc = "1: Use external source configured by EXTREFSEL as reference for conversion."]
    External = 1,
    #[doc = "2: Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    SupplyOneHalfPrescaling = 2,
    #[doc = "3: Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    SupplyOneThirdPrescaling = 3,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - ADC reference selection."]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            0 => Refsel::Vbg,
            1 => Refsel::External,
            2 => Refsel::SupplyOneHalfPrescaling,
            3 => Refsel::SupplyOneThirdPrescaling,
            _ => unreachable!(),
        }
    }
    #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
    #[inline(always)]
    pub fn is_vbg(&self) -> bool {
        *self == Refsel::Vbg
    }
    #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Refsel::External
    }
    #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    #[inline(always)]
    pub fn is_supply_one_half_prescaling(&self) -> bool {
        *self == Refsel::SupplyOneHalfPrescaling
    }
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    #[inline(always)]
    pub fn is_supply_one_third_prescaling(&self) -> bool {
        *self == Refsel::SupplyOneThirdPrescaling
    }
}
#[doc = "Field `REFSEL` writer - ADC reference selection."]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refsel, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use internal 1.2V bandgap voltage as reference for conversion."]
    #[inline(always)]
    pub fn vbg(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vbg)
    }
    #[doc = "Use external source configured by EXTREFSEL as reference for conversion."]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::External)
    }
    #[doc = "Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V."]
    #[inline(always)]
    pub fn supply_one_half_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyOneHalfPrescaling)
    }
    #[doc = "Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V."]
    #[inline(always)]
    pub fn supply_one_third_prescaling(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::SupplyOneThirdPrescaling)
    }
}
#[doc = "ADC analog pin selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: Analog input pins disabled."]
    Disabled = 0,
    #[doc = "1: Use analog input 0 as analog input."]
    AnalogInput0 = 1,
    #[doc = "2: Use analog input 1 as analog input."]
    AnalogInput1 = 2,
    #[doc = "4: Use analog input 2 as analog input."]
    AnalogInput2 = 4,
    #[doc = "8: Use analog input 3 as analog input."]
    AnalogInput3 = 8,
    #[doc = "16: Use analog input 4 as analog input."]
    AnalogInput4 = 16,
    #[doc = "32: Use analog input 5 as analog input."]
    AnalogInput5 = 32,
    #[doc = "64: Use analog input 6 as analog input."]
    AnalogInput6 = 64,
    #[doc = "128: Use analog input 7 as analog input."]
    AnalogInput7 = 128,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
impl crate::IsEnum for Psel {}
#[doc = "Field `PSEL` reader - ADC analog pin selection."]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Psel> {
        match self.bits {
            0 => Some(Psel::Disabled),
            1 => Some(Psel::AnalogInput0),
            2 => Some(Psel::AnalogInput1),
            4 => Some(Psel::AnalogInput2),
            8 => Some(Psel::AnalogInput3),
            16 => Some(Psel::AnalogInput4),
            32 => Some(Psel::AnalogInput5),
            64 => Some(Psel::AnalogInput6),
            128 => Some(Psel::AnalogInput7),
            _ => None,
        }
    }
    #[doc = "Analog input pins disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Psel::Disabled
    }
    #[doc = "Use analog input 0 as analog input."]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == Psel::AnalogInput0
    }
    #[doc = "Use analog input 1 as analog input."]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == Psel::AnalogInput1
    }
    #[doc = "Use analog input 2 as analog input."]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == Psel::AnalogInput2
    }
    #[doc = "Use analog input 3 as analog input."]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == Psel::AnalogInput3
    }
    #[doc = "Use analog input 4 as analog input."]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == Psel::AnalogInput4
    }
    #[doc = "Use analog input 5 as analog input."]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == Psel::AnalogInput5
    }
    #[doc = "Use analog input 6 as analog input."]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == Psel::AnalogInput6
    }
    #[doc = "Use analog input 7 as analog input."]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == Psel::AnalogInput7
    }
}
#[doc = "Field `PSEL` writer - ADC analog pin selection."]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Psel>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog input pins disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Disabled)
    }
    #[doc = "Use analog input 0 as analog input."]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput0)
    }
    #[doc = "Use analog input 1 as analog input."]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput1)
    }
    #[doc = "Use analog input 2 as analog input."]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput2)
    }
    #[doc = "Use analog input 3 as analog input."]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput3)
    }
    #[doc = "Use analog input 4 as analog input."]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput4)
    }
    #[doc = "Use analog input 5 as analog input."]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput5)
    }
    #[doc = "Use analog input 6 as analog input."]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput6)
    }
    #[doc = "Use analog input 7 as analog input."]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput7)
    }
}
#[doc = "ADC external reference pin selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extrefsel {
    #[doc = "0: Analog external reference inputs disabled."]
    None = 0,
    #[doc = "1: Use analog reference 0 as reference."]
    AnalogReference0 = 1,
    #[doc = "2: Use analog reference 1 as reference."]
    AnalogReference1 = 2,
}
impl From<Extrefsel> for u8 {
    #[inline(always)]
    fn from(variant: Extrefsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extrefsel {
    type Ux = u8;
}
impl crate::IsEnum for Extrefsel {}
#[doc = "Field `EXTREFSEL` reader - ADC external reference pin selection."]
pub type ExtrefselR = crate::FieldReader<Extrefsel>;
impl ExtrefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Extrefsel> {
        match self.bits {
            0 => Some(Extrefsel::None),
            1 => Some(Extrefsel::AnalogReference0),
            2 => Some(Extrefsel::AnalogReference1),
            _ => None,
        }
    }
    #[doc = "Analog external reference inputs disabled."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Extrefsel::None
    }
    #[doc = "Use analog reference 0 as reference."]
    #[inline(always)]
    pub fn is_analog_reference0(&self) -> bool {
        *self == Extrefsel::AnalogReference0
    }
    #[doc = "Use analog reference 1 as reference."]
    #[inline(always)]
    pub fn is_analog_reference1(&self) -> bool {
        *self == Extrefsel::AnalogReference1
    }
}
#[doc = "Field `EXTREFSEL` writer - ADC external reference pin selection."]
pub type ExtrefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Extrefsel>;
impl<'a, REG> ExtrefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog external reference inputs disabled."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefsel::None)
    }
    #[doc = "Use analog reference 0 as reference."]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefsel::AnalogReference0)
    }
    #[doc = "Use analog reference 1 as reference."]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut crate::W<REG> {
        self.variant(Extrefsel::AnalogReference1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline(always)]
    pub fn extrefsel(&self) -> ExtrefselR {
        ExtrefselR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC resolution."]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<'_, ConfigSpec> {
        ResW::new(self, 0)
    }
    #[doc = "Bits 2:4 - ADC input selection."]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<'_, ConfigSpec> {
        InpselW::new(self, 2)
    }
    #[doc = "Bits 5:6 - ADC reference selection."]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, ConfigSpec> {
        RefselW::new(self, 5)
    }
    #[doc = "Bits 8:15 - ADC analog pin selection."]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<'_, ConfigSpec> {
        PselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - ADC external reference pin selection."]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> ExtrefselW<'_, ConfigSpec> {
        ExtrefselW::new(self, 16)
    }
}
#[doc = "ADC configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x18"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x18;
}
