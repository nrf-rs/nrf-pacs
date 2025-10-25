#[doc = "Register `FRAMECONFIG` reader"]
pub type R = crate::R<FrameconfigSpec>;
#[doc = "Register `FRAMECONFIG` writer"]
pub type W = crate::W<FrameconfigSpec>;
#[doc = "Indicates if parity expected in RX frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    #[doc = "0: Parity is not expected in RX frames"]
    NoParity = 0,
    #[doc = "1: Parity is expected in RX frames"]
    Parity = 1,
}
impl From<Parity> for bool {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` reader - Indicates if parity expected in RX frame"]
pub type ParityR = crate::BitReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parity {
        match self.bits {
            false => Parity::NoParity,
            true => Parity::Parity,
        }
    }
    #[doc = "Parity is not expected in RX frames"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == Parity::NoParity
    }
    #[doc = "Parity is expected in RX frames"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        *self == Parity::Parity
    }
}
#[doc = "Field `PARITY` writer - Indicates if parity expected in RX frame"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity is not expected in RX frames"]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::NoParity)
    }
    #[doc = "Parity is expected in RX frames"]
    #[inline(always)]
    pub fn parity(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Parity)
    }
}
#[doc = "SoF expected or not in RX frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: SoF symbol is not expected in RX frames"]
    NoSoF = 0,
    #[doc = "1: SoF symbol is expected in RX frames"]
    SoF = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - SoF expected or not in RX frames"]
pub type SofR = crate::BitReader<Sof>;
impl SofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof {
        match self.bits {
            false => Sof::NoSoF,
            true => Sof::SoF,
        }
    }
    #[doc = "SoF symbol is not expected in RX frames"]
    #[inline(always)]
    pub fn is_no_so_f(&self) -> bool {
        *self == Sof::NoSoF
    }
    #[doc = "SoF symbol is expected in RX frames"]
    #[inline(always)]
    pub fn is_so_f(&self) -> bool {
        *self == Sof::SoF
    }
}
#[doc = "Field `SOF` writer - SoF expected or not in RX frames"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG, Sof>;
impl<'a, REG> SofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SoF symbol is not expected in RX frames"]
    #[inline(always)]
    pub fn no_so_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::NoSoF)
    }
    #[doc = "SoF symbol is expected in RX frames"]
    #[inline(always)]
    pub fn so_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::SoF)
    }
}
#[doc = "CRC mode for incoming frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcmoderx {
    #[doc = "0: CRC is not expected in RX frames"]
    NoCrcrx = 0,
    #[doc = "1: Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    Crc16rx = 1,
}
impl From<Crcmoderx> for bool {
    #[inline(always)]
    fn from(variant: Crcmoderx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCMODERX` reader - CRC mode for incoming frames"]
pub type CrcmoderxR = crate::BitReader<Crcmoderx>;
impl CrcmoderxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcmoderx {
        match self.bits {
            false => Crcmoderx::NoCrcrx,
            true => Crcmoderx::Crc16rx,
        }
    }
    #[doc = "CRC is not expected in RX frames"]
    #[inline(always)]
    pub fn is_no_crcrx(&self) -> bool {
        *self == Crcmoderx::NoCrcrx
    }
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    #[inline(always)]
    pub fn is_crc16rx(&self) -> bool {
        *self == Crcmoderx::Crc16rx
    }
}
#[doc = "Field `CRCMODERX` writer - CRC mode for incoming frames"]
pub type CrcmoderxW<'a, REG> = crate::BitWriter<'a, REG, Crcmoderx>;
impl<'a, REG> CrcmoderxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC is not expected in RX frames"]
    #[inline(always)]
    pub fn no_crcrx(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmoderx::NoCrcrx)
    }
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    #[inline(always)]
    pub fn crc16rx(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmoderx::Crc16rx)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if parity expected in RX frame"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline(always)]
    pub fn crcmoderx(&self) -> CrcmoderxR {
        CrcmoderxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if parity expected in RX frame"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, FrameconfigSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, FrameconfigSpec> {
        SofW::new(self, 2)
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline(always)]
    pub fn crcmoderx(&mut self) -> CrcmoderxW<'_, FrameconfigSpec> {
        CrcmoderxW::new(self, 4)
    }
}
#[doc = "Configuration of incoming frames\n\nYou can [`read`](crate::Reg::read) this register and get [`frameconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frameconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameconfigSpec;
impl crate::RegisterSpec for FrameconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frameconfig::R`](R) reader structure"]
impl crate::Readable for FrameconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`frameconfig::W`](W) writer structure"]
impl crate::Writable for FrameconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMECONFIG to value 0x15"]
impl crate::Resettable for FrameconfigSpec {
    const RESET_VALUE: u32 = 0x15;
}
