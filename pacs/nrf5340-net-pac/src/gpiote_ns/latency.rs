#[doc = "Register `LATENCY` reader"]
pub type R = crate::R<LatencySpec>;
#[doc = "Register `LATENCY` writer"]
pub type W = crate::W<LatencySpec>;
#[doc = "Latency setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Latency {
    #[doc = "0: Low power setting, for signals with minimum hold time tGPIOTE,HOLD,LP; refer to Electrical specification section"]
    LowPower = 0,
    #[doc = "1: Low latency setting, for signals with minimum hold time tGPIOTE,HOLD,LL; refer to Electrical specification section"]
    LowLatency = 1,
}
impl From<Latency> for bool {
    #[inline(always)]
    fn from(variant: Latency) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATENCY` reader - Latency setting"]
pub type LatencyR = crate::BitReader<Latency>;
impl LatencyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Latency {
        match self.bits {
            false => Latency::LowPower,
            true => Latency::LowLatency,
        }
    }
    #[doc = "Low power setting, for signals with minimum hold time tGPIOTE,HOLD,LP; refer to Electrical specification section"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Latency::LowPower
    }
    #[doc = "Low latency setting, for signals with minimum hold time tGPIOTE,HOLD,LL; refer to Electrical specification section"]
    #[inline(always)]
    pub fn is_low_latency(&self) -> bool {
        *self == Latency::LowLatency
    }
}
#[doc = "Field `LATENCY` writer - Latency setting"]
pub type LatencyW<'a, REG> = crate::BitWriter<'a, REG, Latency>;
impl<'a, REG> LatencyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low power setting, for signals with minimum hold time tGPIOTE,HOLD,LP; refer to Electrical specification section"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::LowPower)
    }
    #[doc = "Low latency setting, for signals with minimum hold time tGPIOTE,HOLD,LL; refer to Electrical specification section"]
    #[inline(always)]
    pub fn low_latency(self) -> &'a mut crate::W<REG> {
        self.variant(Latency::LowLatency)
    }
}
impl R {
    #[doc = "Bit 0 - Latency setting"]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latency setting"]
    #[inline(always)]
    pub fn latency(&mut self) -> LatencyW<'_, LatencySpec> {
        LatencyW::new(self, 0)
    }
}
#[doc = "Latency selection for Event mode (MODE=Event) with rising or falling edge detection on the pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`latency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`latency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LatencySpec;
impl crate::RegisterSpec for LatencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`latency::R`](R) reader structure"]
impl crate::Readable for LatencySpec {}
#[doc = "`write(|w| ..)` method takes [`latency::W`](W) writer structure"]
impl crate::Writable for LatencySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LATENCY to value 0x01"]
impl crate::Resettable for LatencySpec {
    const RESET_VALUE: u32 = 0x01;
}
