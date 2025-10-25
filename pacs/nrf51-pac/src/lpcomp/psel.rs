#[doc = "Register `PSEL` reader"]
pub type R = crate::R<PselSpec>;
#[doc = "Register `PSEL` writer"]
pub type W = crate::W<PselSpec>;
#[doc = "Analog input pin select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: Use analog input 0 as analog input."]
    AnalogInput0 = 0,
    #[doc = "1: Use analog input 1 as analog input."]
    AnalogInput1 = 1,
    #[doc = "2: Use analog input 2 as analog input."]
    AnalogInput2 = 2,
    #[doc = "3: Use analog input 3 as analog input."]
    AnalogInput3 = 3,
    #[doc = "4: Use analog input 4 as analog input."]
    AnalogInput4 = 4,
    #[doc = "5: Use analog input 5 as analog input."]
    AnalogInput5 = 5,
    #[doc = "6: Use analog input 6 as analog input."]
    AnalogInput6 = 6,
    #[doc = "7: Use analog input 7 as analog input."]
    AnalogInput7 = 7,
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
#[doc = "Field `PSEL` reader - Analog input pin select."]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psel {
        match self.bits {
            0 => Psel::AnalogInput0,
            1 => Psel::AnalogInput1,
            2 => Psel::AnalogInput2,
            3 => Psel::AnalogInput3,
            4 => Psel::AnalogInput4,
            5 => Psel::AnalogInput5,
            6 => Psel::AnalogInput6,
            7 => Psel::AnalogInput7,
            _ => unreachable!(),
        }
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
#[doc = "Field `PSEL` writer - Analog input pin select."]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Psel, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
impl R {
    #[doc = "Bits 0:2 - Analog input pin select."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog input pin select."]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<'_, PselSpec> {
        PselW::new(self, 0)
    }
}
#[doc = "Input pin select.\n\nYou can [`read`](crate::Reg::read) this register and get [`psel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
