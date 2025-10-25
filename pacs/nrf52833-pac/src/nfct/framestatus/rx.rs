#[doc = "Register `RX` reader"]
pub type R = crate::R<RxSpec>;
#[doc = "Register `RX` writer"]
pub type W = crate::W<RxSpec>;
#[doc = "No valid end of frame (EoF) detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerror {
    #[doc = "0: Valid CRC detected"]
    Crccorrect = 0,
    #[doc = "1: CRC received does not match local check"]
    Crcerror = 1,
}
impl From<Crcerror> for bool {
    #[inline(always)]
    fn from(variant: Crcerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERROR` reader - No valid end of frame (EoF) detected"]
pub type CrcerrorR = crate::BitReader<Crcerror>;
impl CrcerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcerror {
        match self.bits {
            false => Crcerror::Crccorrect,
            true => Crcerror::Crcerror,
        }
    }
    #[doc = "Valid CRC detected"]
    #[inline(always)]
    pub fn is_crccorrect(&self) -> bool {
        *self == Crcerror::Crccorrect
    }
    #[doc = "CRC received does not match local check"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        *self == Crcerror::Crcerror
    }
}
#[doc = "Field `CRCERROR` writer - No valid end of frame (EoF) detected"]
pub type CrcerrorW<'a, REG> = crate::BitWriter1C<'a, REG, Crcerror>;
impl<'a, REG> CrcerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Valid CRC detected"]
    #[inline(always)]
    pub fn crccorrect(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerror::Crccorrect)
    }
    #[doc = "CRC received does not match local check"]
    #[inline(always)]
    pub fn crcerror(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerror::Crcerror)
    }
}
#[doc = "Parity status of received frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paritystatus {
    #[doc = "0: Frame received with parity OK"]
    ParityOk = 0,
    #[doc = "1: Frame received with parity error"]
    ParityError = 1,
}
impl From<Paritystatus> for bool {
    #[inline(always)]
    fn from(variant: Paritystatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYSTATUS` reader - Parity status of received frame"]
pub type ParitystatusR = crate::BitReader<Paritystatus>;
impl ParitystatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Paritystatus {
        match self.bits {
            false => Paritystatus::ParityOk,
            true => Paritystatus::ParityError,
        }
    }
    #[doc = "Frame received with parity OK"]
    #[inline(always)]
    pub fn is_parity_ok(&self) -> bool {
        *self == Paritystatus::ParityOk
    }
    #[doc = "Frame received with parity error"]
    #[inline(always)]
    pub fn is_parity_error(&self) -> bool {
        *self == Paritystatus::ParityError
    }
}
#[doc = "Field `PARITYSTATUS` writer - Parity status of received frame"]
pub type ParitystatusW<'a, REG> = crate::BitWriter1C<'a, REG, Paritystatus>;
impl<'a, REG> ParitystatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frame received with parity OK"]
    #[inline(always)]
    pub fn parity_ok(self) -> &'a mut crate::W<REG> {
        self.variant(Paritystatus::ParityOk)
    }
    #[doc = "Frame received with parity error"]
    #[inline(always)]
    pub fn parity_error(self) -> &'a mut crate::W<REG> {
        self.variant(Paritystatus::ParityError)
    }
}
#[doc = "Overrun detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overrun {
    #[doc = "0: No overrun detected"]
    NoOverrun = 0,
    #[doc = "1: Overrun error"]
    Overrun = 1,
}
impl From<Overrun> for bool {
    #[inline(always)]
    fn from(variant: Overrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` reader - Overrun detected"]
pub type OverrunR = crate::BitReader<Overrun>;
impl OverrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overrun {
        match self.bits {
            false => Overrun::NoOverrun,
            true => Overrun::Overrun,
        }
    }
    #[doc = "No overrun detected"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Overrun::NoOverrun
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Overrun::Overrun
    }
}
#[doc = "Field `OVERRUN` writer - Overrun detected"]
pub type OverrunW<'a, REG> = crate::BitWriter1C<'a, REG, Overrun>;
impl<'a, REG> OverrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun detected"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Overrun::NoOverrun)
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Overrun::Overrun)
    }
}
impl R {
    #[doc = "Bit 0 - No valid end of frame (EoF) detected"]
    #[inline(always)]
    pub fn crcerror(&self) -> CrcerrorR {
        CrcerrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline(always)]
    pub fn paritystatus(&self) -> ParitystatusR {
        ParitystatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No valid end of frame (EoF) detected"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CrcerrorW<'_, RxSpec> {
        CrcerrorW::new(self, 0)
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline(always)]
    pub fn paritystatus(&mut self) -> ParitystatusW<'_, RxSpec> {
        ParitystatusW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, RxSpec> {
        OverrunW::new(self, 3)
    }
}
#[doc = "Result of last incoming frame\n\nYou can [`read`](crate::Reg::read) this register and get [`rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxSpec;
impl crate::RegisterSpec for RxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx::R`](R) reader structure"]
impl crate::Readable for RxSpec {}
#[doc = "`write(|w| ..)` method takes [`rx::W`](W) writer structure"]
impl crate::Writable for RxSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0d;
}
#[doc = "`reset()` method sets RX to value 0"]
impl crate::Resettable for RxSpec {}
