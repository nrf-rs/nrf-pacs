#[doc = "Register `PSEL` reader"]
pub type R = crate::R<PselSpec>;
#[doc = "Register `PSEL` writer"]
pub type W = crate::W<PselSpec>;
#[doc = "Analog pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: AIN0 selected as analog input"]
    AnalogInput0 = 0,
    #[doc = "1: AIN1 selected as analog input"]
    AnalogInput1 = 1,
    #[doc = "2: AIN2 selected as analog input"]
    AnalogInput2 = 2,
    #[doc = "3: AIN3 selected as analog input"]
    AnalogInput3 = 3,
    #[doc = "7: VDDH/5 selected as analog input"]
    VddhDiv5 = 7,
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
#[doc = "Field `PSEL` reader - Analog pin select"]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Psel> {
        match self.bits {
            0 => Some(Psel::AnalogInput0),
            1 => Some(Psel::AnalogInput1),
            2 => Some(Psel::AnalogInput2),
            3 => Some(Psel::AnalogInput3),
            7 => Some(Psel::VddhDiv5),
            _ => None,
        }
    }
    #[doc = "AIN0 selected as analog input"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == Psel::AnalogInput0
    }
    #[doc = "AIN1 selected as analog input"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == Psel::AnalogInput1
    }
    #[doc = "AIN2 selected as analog input"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == Psel::AnalogInput2
    }
    #[doc = "AIN3 selected as analog input"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == Psel::AnalogInput3
    }
    #[doc = "VDDH/5 selected as analog input"]
    #[inline(always)]
    pub fn is_vddh_div5(&self) -> bool {
        *self == Psel::VddhDiv5
    }
}
#[doc = "Field `PSEL` writer - Analog pin select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Psel>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AIN0 selected as analog input"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput0)
    }
    #[doc = "AIN1 selected as analog input"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput1)
    }
    #[doc = "AIN2 selected as analog input"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput2)
    }
    #[doc = "AIN3 selected as analog input"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::AnalogInput3)
    }
    #[doc = "VDDH/5 selected as analog input"]
    #[inline(always)]
    pub fn vddh_div5(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::VddhDiv5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<'_, PselSpec> {
        PselW::new(self, 0)
    }
}
#[doc = "Pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`psel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselSpec;
impl crate::RegisterSpec for PselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psel::R`](R) reader structure"]
impl crate::Readable for PselSpec {}
#[doc = "`write(|w| ..)` method takes [`psel::W`](W) writer structure"]
impl crate::Writable for PselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSEL to value 0"]
impl crate::Resettable for PselSpec {}
