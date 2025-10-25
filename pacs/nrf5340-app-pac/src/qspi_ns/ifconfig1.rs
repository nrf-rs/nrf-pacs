#[doc = "Register `IFCONFIG1` reader"]
pub type R = crate::R<Ifconfig1Spec>;
#[doc = "Register `IFCONFIG1` writer"]
pub type W = crate::W<Ifconfig1Spec>;
#[doc = "Field `SCKDELAY` reader - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
pub type SckdelayR = crate::FieldReader;
#[doc = "Field `SCKDELAY` writer - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
pub type SckdelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Enter/exit deep power-down mode (DPM) for external flash memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpmen {
    #[doc = "0: Exit DPM."]
    Exit = 0,
    #[doc = "1: Enter DPM."]
    Enter = 1,
}
impl From<Dpmen> for bool {
    #[inline(always)]
    fn from(variant: Dpmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPMEN` reader - Enter/exit deep power-down mode (DPM) for external flash memory."]
pub type DpmenR = crate::BitReader<Dpmen>;
impl DpmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpmen {
        match self.bits {
            false => Dpmen::Exit,
            true => Dpmen::Enter,
        }
    }
    #[doc = "Exit DPM."]
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == Dpmen::Exit
    }
    #[doc = "Enter DPM."]
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        *self == Dpmen::Enter
    }
}
#[doc = "Field `DPMEN` writer - Enter/exit deep power-down mode (DPM) for external flash memory."]
pub type DpmenW<'a, REG> = crate::BitWriter<'a, REG, Dpmen>;
impl<'a, REG> DpmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exit DPM."]
    #[inline(always)]
    pub fn exit(self) -> &'a mut crate::W<REG> {
        self.variant(Dpmen::Exit)
    }
    #[doc = "Enter DPM."]
    #[inline(always)]
    pub fn enter(self) -> &'a mut crate::W<REG> {
        self.variant(Dpmen::Enter)
    }
}
#[doc = "Select SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spimode {
    #[doc = "0: Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    Mode0 = 0,
    #[doc = "1: Mode 3: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    Mode3 = 1,
}
impl From<Spimode> for bool {
    #[inline(always)]
    fn from(variant: Spimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIMODE` reader - Select SPI mode."]
pub type SpimodeR = crate::BitReader<Spimode>;
impl SpimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spimode {
        match self.bits {
            false => Spimode::Mode0,
            true => Spimode::Mode3,
        }
    }
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Spimode::Mode0
    }
    #[doc = "Mode 3: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Spimode::Mode3
    }
}
#[doc = "Field `SPIMODE` writer - Select SPI mode."]
pub type SpimodeW<'a, REG> = crate::BitWriter<'a, REG, Spimode>;
impl<'a, REG> SpimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::Mode0)
    }
    #[doc = "Mode 3: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::Mode3)
    }
}
#[doc = "Field `SCKFREQ` reader - SCK frequency is derived from PCLK192M with SCK frequency = PCLK192M / (2*(SCKFREQ + 1))."]
pub type SckfreqR = crate::FieldReader;
#[doc = "Field `SCKFREQ` writer - SCK frequency is derived from PCLK192M with SCK frequency = PCLK192M / (2*(SCKFREQ + 1))."]
pub type SckfreqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn sckdelay(&self) -> SckdelayR {
        SckdelayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn dpmen(&self) -> DpmenR {
        DpmenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - SCK frequency is derived from PCLK192M with SCK frequency = PCLK192M / (2*(SCKFREQ + 1))."]
    #[inline(always)]
    pub fn sckfreq(&self) -> SckfreqR {
        SckfreqR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn sckdelay(&mut self) -> SckdelayW<'_, Ifconfig1Spec> {
        SckdelayW::new(self, 0)
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn dpmen(&mut self) -> DpmenW<'_, Ifconfig1Spec> {
        DpmenW::new(self, 24)
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline(always)]
    pub fn spimode(&mut self) -> SpimodeW<'_, Ifconfig1Spec> {
        SpimodeW::new(self, 25)
    }
    #[doc = "Bits 28:31 - SCK frequency is derived from PCLK192M with SCK frequency = PCLK192M / (2*(SCKFREQ + 1))."]
    #[inline(always)]
    pub fn sckfreq(&mut self) -> SckfreqW<'_, Ifconfig1Spec> {
        SckfreqW::new(self, 28)
    }
}
#[doc = "Interface configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ifconfig1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifconfig1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifconfig1Spec;
impl crate::RegisterSpec for Ifconfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifconfig1::R`](R) reader structure"]
impl crate::Readable for Ifconfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`ifconfig1::W`](W) writer structure"]
impl crate::Writable for Ifconfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCONFIG1 to value 0x0004_0480"]
impl crate::Resettable for Ifconfig1Spec {
    const RESET_VALUE: u32 = 0x0004_0480;
}
