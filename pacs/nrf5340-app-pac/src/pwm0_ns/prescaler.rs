#[doc = "Register `PRESCALER` reader"]
pub type R = crate::R<PrescalerSpec>;
#[doc = "Register `PRESCALER` writer"]
pub type W = crate::W<PrescalerSpec>;
#[doc = "Prescaler of PWM_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler {
    #[doc = "0: Divide by 1 (16 MHz)"]
    Div1 = 0,
    #[doc = "1: Divide by 2 (8 MHz)"]
    Div2 = 1,
    #[doc = "2: Divide by 4 (4 MHz)"]
    Div4 = 2,
    #[doc = "3: Divide by 8 (2 MHz)"]
    Div8 = 3,
    #[doc = "4: Divide by 16 (1 MHz)"]
    Div16 = 4,
    #[doc = "5: Divide by 32 (500 kHz)"]
    Div32 = 5,
    #[doc = "6: Divide by 64 (250 kHz)"]
    Div64 = 6,
    #[doc = "7: Divide by 128 (125 kHz)"]
    Div128 = 7,
}
impl From<Prescaler> for u8 {
    #[inline(always)]
    fn from(variant: Prescaler) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescaler {
    type Ux = u8;
}
impl crate::IsEnum for Prescaler {}
#[doc = "Field `PRESCALER` reader - Prescaler of PWM_CLK"]
pub type PrescalerR = crate::FieldReader<Prescaler>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescaler {
        match self.bits {
            0 => Prescaler::Div1,
            1 => Prescaler::Div2,
            2 => Prescaler::Div4,
            3 => Prescaler::Div8,
            4 => Prescaler::Div16,
            5 => Prescaler::Div32,
            6 => Prescaler::Div64,
            7 => Prescaler::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1 (16 MHz)"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == Prescaler::Div1
    }
    #[doc = "Divide by 2 (8 MHz)"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == Prescaler::Div2
    }
    #[doc = "Divide by 4 (4 MHz)"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == Prescaler::Div4
    }
    #[doc = "Divide by 8 (2 MHz)"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == Prescaler::Div8
    }
    #[doc = "Divide by 16 (1 MHz)"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == Prescaler::Div16
    }
    #[doc = "Divide by 32 (500 kHz)"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == Prescaler::Div32
    }
    #[doc = "Divide by 64 (250 kHz)"]
    #[inline(always)]
    pub fn is_div_64(&self) -> bool {
        *self == Prescaler::Div64
    }
    #[doc = "Divide by 128 (125 kHz)"]
    #[inline(always)]
    pub fn is_div_128(&self) -> bool {
        *self == Prescaler::Div128
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler of PWM_CLK"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescaler, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1 (16 MHz)"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div1)
    }
    #[doc = "Divide by 2 (8 MHz)"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div2)
    }
    #[doc = "Divide by 4 (4 MHz)"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div4)
    }
    #[doc = "Divide by 8 (2 MHz)"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div8)
    }
    #[doc = "Divide by 16 (1 MHz)"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div16)
    }
    #[doc = "Divide by 32 (500 kHz)"]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div32)
    }
    #[doc = "Divide by 64 (250 kHz)"]
    #[inline(always)]
    pub fn div_64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div64)
    }
    #[doc = "Divide by 128 (125 kHz)"]
    #[inline(always)]
    pub fn div_128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescaler::Div128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler of PWM_CLK"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler of PWM_CLK"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<'_, PrescalerSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "Configuration for PWM_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`prescaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescalerSpec;
impl crate::RegisterSpec for PrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prescaler::R`](R) reader structure"]
impl crate::Readable for PrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`prescaler::W`](W) writer structure"]
impl crate::Writable for PrescalerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESCALER to value 0"]
impl crate::Resettable for PrescalerSpec {}
