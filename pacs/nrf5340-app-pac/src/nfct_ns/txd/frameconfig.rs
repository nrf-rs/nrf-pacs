#[doc = "Register `FRAMECONFIG` reader"]
pub type R = crate::R<FrameconfigSpec>;
#[doc = "Register `FRAMECONFIG` writer"]
pub type W = crate::W<FrameconfigSpec>;
#[doc = "Indicates if parity is added to the frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parity {
    #[doc = "0: Parity is not added to TX frames"]
    NoParity = 0,
    #[doc = "1: Parity is added to TX frames"]
    Parity = 1,
}
impl From<Parity> for bool {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITY` reader - Indicates if parity is added to the frame"]
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
    #[doc = "Parity is not added to TX frames"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == Parity::NoParity
    }
    #[doc = "Parity is added to TX frames"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        *self == Parity::Parity
    }
}
#[doc = "Field `PARITY` writer - Indicates if parity is added to the frame"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity is not added to TX frames"]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::NoParity)
    }
    #[doc = "Parity is added to TX frames"]
    #[inline(always)]
    pub fn parity(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Parity)
    }
}
#[doc = "Discarding unused bits at start or end of a frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Discardmode {
    #[doc = "0: Unused bits are discarded at end of frame (EoF)"]
    DiscardEnd = 0,
    #[doc = "1: Unused bits are discarded at start of frame (SoF)"]
    DiscardStart = 1,
}
impl From<Discardmode> for bool {
    #[inline(always)]
    fn from(variant: Discardmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCARDMODE` reader - Discarding unused bits at start or end of a frame"]
pub type DiscardmodeR = crate::BitReader<Discardmode>;
impl DiscardmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Discardmode {
        match self.bits {
            false => Discardmode::DiscardEnd,
            true => Discardmode::DiscardStart,
        }
    }
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    #[inline(always)]
    pub fn is_discard_end(&self) -> bool {
        *self == Discardmode::DiscardEnd
    }
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    #[inline(always)]
    pub fn is_discard_start(&self) -> bool {
        *self == Discardmode::DiscardStart
    }
}
#[doc = "Field `DISCARDMODE` writer - Discarding unused bits at start or end of a frame"]
pub type DiscardmodeW<'a, REG> = crate::BitWriter<'a, REG, Discardmode>;
impl<'a, REG> DiscardmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    #[inline(always)]
    pub fn discard_end(self) -> &'a mut crate::W<REG> {
        self.variant(Discardmode::DiscardEnd)
    }
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    #[inline(always)]
    pub fn discard_start(self) -> &'a mut crate::W<REG> {
        self.variant(Discardmode::DiscardStart)
    }
}
#[doc = "Adding SoF or not in TX frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: SoF symbol not added"]
    NoSoF = 0,
    #[doc = "1: SoF symbol added"]
    SoF = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Adding SoF or not in TX frames"]
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
    #[doc = "SoF symbol not added"]
    #[inline(always)]
    pub fn is_no_so_f(&self) -> bool {
        *self == Sof::NoSoF
    }
    #[doc = "SoF symbol added"]
    #[inline(always)]
    pub fn is_so_f(&self) -> bool {
        *self == Sof::SoF
    }
}
#[doc = "Field `SOF` writer - Adding SoF or not in TX frames"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG, Sof>;
impl<'a, REG> SofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SoF symbol not added"]
    #[inline(always)]
    pub fn no_so_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::NoSoF)
    }
    #[doc = "SoF symbol added"]
    #[inline(always)]
    pub fn so_f(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::SoF)
    }
}
#[doc = "CRC mode for outgoing frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcmodetx {
    #[doc = "0: CRC is not added to the frame"]
    NoCrctx = 0,
    #[doc = "1: 16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    Crc16tx = 1,
}
impl From<Crcmodetx> for bool {
    #[inline(always)]
    fn from(variant: Crcmodetx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCMODETX` reader - CRC mode for outgoing frames"]
pub type CrcmodetxR = crate::BitReader<Crcmodetx>;
impl CrcmodetxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcmodetx {
        match self.bits {
            false => Crcmodetx::NoCrctx,
            true => Crcmodetx::Crc16tx,
        }
    }
    #[doc = "CRC is not added to the frame"]
    #[inline(always)]
    pub fn is_no_crctx(&self) -> bool {
        *self == Crcmodetx::NoCrctx
    }
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    #[inline(always)]
    pub fn is_crc16tx(&self) -> bool {
        *self == Crcmodetx::Crc16tx
    }
}
#[doc = "Field `CRCMODETX` writer - CRC mode for outgoing frames"]
pub type CrcmodetxW<'a, REG> = crate::BitWriter<'a, REG, Crcmodetx>;
impl<'a, REG> CrcmodetxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC is not added to the frame"]
    #[inline(always)]
    pub fn no_crctx(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmodetx::NoCrctx)
    }
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    #[inline(always)]
    pub fn crc16tx(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmodetx::Crc16tx)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if parity is added to the frame"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Discarding unused bits at start or end of a frame"]
    #[inline(always)]
    pub fn discardmode(&self) -> DiscardmodeR {
        DiscardmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline(always)]
    pub fn crcmodetx(&self) -> CrcmodetxR {
        CrcmodetxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if parity is added to the frame"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, FrameconfigSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - Discarding unused bits at start or end of a frame"]
    #[inline(always)]
    pub fn discardmode(&mut self) -> DiscardmodeW<'_, FrameconfigSpec> {
        DiscardmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, FrameconfigSpec> {
        SofW::new(self, 2)
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline(always)]
    pub fn crcmodetx(&mut self) -> CrcmodetxW<'_, FrameconfigSpec> {
        CrcmodetxW::new(self, 4)
    }
}
#[doc = "Configuration of outgoing frames\n\nYou can [`read`](crate::Reg::read) this register and get [`frameconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frameconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets FRAMECONFIG to value 0x17"]
impl crate::Resettable for FrameconfigSpec {
    const RESET_VALUE: u32 = 0x17;
}
