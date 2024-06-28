#[doc = "Register `PSELP` reader"]
pub type R = crate::R<PselpSpec>;
#[doc = "Register `PSELP` writer"]
pub type W = crate::W<PselpSpec>;
#[doc = "Analog positive input channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pselp {
    #[doc = "0: Not connected"]
    Nc = 0,
    #[doc = "1: AIN0"]
    AnalogInput0 = 1,
    #[doc = "2: AIN1"]
    AnalogInput1 = 2,
    #[doc = "3: AIN2"]
    AnalogInput2 = 3,
    #[doc = "4: AIN3"]
    AnalogInput3 = 4,
    #[doc = "5: AIN4"]
    AnalogInput4 = 5,
    #[doc = "6: AIN5"]
    AnalogInput5 = 6,
    #[doc = "7: AIN6"]
    AnalogInput6 = 7,
    #[doc = "8: AIN7"]
    AnalogInput7 = 8,
    #[doc = "9: VDD_GPIO"]
    Vddgpio = 9,
}
impl From<Pselp> for u8 {
    #[inline(always)]
    fn from(variant: Pselp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselp {
    type Ux = u8;
}
impl crate::IsEnum for Pselp {}
#[doc = "Field `PSELP` reader - Analog positive input channel"]
pub type PselpR = crate::FieldReader<Pselp>;
impl PselpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselp> {
        match self.bits {
            0 => Some(Pselp::Nc),
            1 => Some(Pselp::AnalogInput0),
            2 => Some(Pselp::AnalogInput1),
            3 => Some(Pselp::AnalogInput2),
            4 => Some(Pselp::AnalogInput3),
            5 => Some(Pselp::AnalogInput4),
            6 => Some(Pselp::AnalogInput5),
            7 => Some(Pselp::AnalogInput6),
            8 => Some(Pselp::AnalogInput7),
            9 => Some(Pselp::Vddgpio),
            _ => None,
        }
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == Pselp::Nc
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == Pselp::AnalogInput0
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == Pselp::AnalogInput1
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == Pselp::AnalogInput2
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == Pselp::AnalogInput3
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == Pselp::AnalogInput4
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == Pselp::AnalogInput5
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == Pselp::AnalogInput6
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == Pselp::AnalogInput7
    }
    #[doc = "VDD_GPIO"]
    #[inline(always)]
    pub fn is_vddgpio(&self) -> bool {
        *self == Pselp::Vddgpio
    }
}
#[doc = "Field `PSELP` writer - Analog positive input channel"]
pub type PselpW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pselp>;
impl<'a, REG> PselpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::Nc)
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput0)
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput1)
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput2)
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput3)
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput4)
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput5)
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput6)
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::AnalogInput7)
    }
    #[doc = "VDD_GPIO"]
    #[inline(always)]
    pub fn vddgpio(self) -> &'a mut crate::W<REG> {
        self.variant(Pselp::Vddgpio)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog positive input channel"]
    #[inline(always)]
    pub fn pselp(&self) -> PselpR {
        PselpR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog positive input channel"]
    #[inline(always)]
    #[must_use]
    pub fn pselp(&mut self) -> PselpW<PselpSpec> {
        PselpW::new(self, 0)
    }
}
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pselp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pselp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselpSpec;
impl crate::RegisterSpec for PselpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselp::R`](R) reader structure"]
impl crate::Readable for PselpSpec {}
#[doc = "`write(|w| ..)` method takes [`pselp::W`](W) writer structure"]
impl crate::Writable for PselpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSELP to value 0"]
impl crate::Resettable for PselpSpec {
    const RESET_VALUE: u32 = 0;
}
