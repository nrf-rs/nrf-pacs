#[doc = "Register `LOWPOWER` reader"]
pub type R = crate::R<LowpowerSpec>;
#[doc = "Register `LOWPOWER` writer"]
pub type W = crate::W<LowpowerSpec>;
#[doc = "Controls USBD peripheral low-power mode during USB suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowpower {
    #[doc = "0: Software must write this value to exit low power mode and before performing a remote wake-up"]
    ForceNormal = 0,
    #[doc = "1: Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LowPower = 1,
}
impl From<Lowpower> for bool {
    #[inline(always)]
    fn from(variant: Lowpower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWPOWER` reader - Controls USBD peripheral low-power mode during USB suspend"]
pub type LowpowerR = crate::BitReader<Lowpower>;
impl LowpowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowpower {
        match self.bits {
            false => Lowpower::ForceNormal,
            true => Lowpower::LowPower,
        }
    }
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    #[inline(always)]
    pub fn is_force_normal(&self) -> bool {
        *self == Lowpower::ForceNormal
    }
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Lowpower::LowPower
    }
}
#[doc = "Field `LOWPOWER` writer - Controls USBD peripheral low-power mode during USB suspend"]
pub type LowpowerW<'a, REG> = crate::BitWriter<'a, REG, Lowpower>;
impl<'a, REG> LowpowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    #[inline(always)]
    pub fn force_normal(self) -> &'a mut crate::W<REG> {
        self.variant(Lowpower::ForceNormal)
    }
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lowpower::LowPower)
    }
}
impl R {
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&self) -> LowpowerR {
        LowpowerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LowpowerW<'_, LowpowerSpec> {
        LowpowerW::new(self, 0)
    }
}
#[doc = "Controls USBD peripheral low power mode during USB suspend\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowpowerSpec;
impl crate::RegisterSpec for LowpowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpower::R`](R) reader structure"]
impl crate::Readable for LowpowerSpec {}
#[doc = "`write(|w| ..)` method takes [`lowpower::W`](W) writer structure"]
impl crate::Writable for LowpowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOWPOWER to value 0"]
impl crate::Resettable for LowpowerSpec {}
