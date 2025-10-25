#[doc = "Register `IFCONFIG0` reader"]
pub type R = crate::R<Ifconfig0Spec>;
#[doc = "Register `IFCONFIG0` writer"]
pub type W = crate::W<Ifconfig0Spec>;
#[doc = "Configure number of data lines and opcode used for reading.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Readoc {
    #[doc = "0: Single data line SPI. FAST_READ (opcode 0x0B)."]
    Fastread = 0,
    #[doc = "1: Dual data line SPI. READ2O (opcode 0x3B)."]
    Read2o = 1,
    #[doc = "2: Dual data line SPI. READ2IO (opcode 0xBB)."]
    Read2io = 2,
    #[doc = "3: Quad data line SPI. READ4O (opcode 0x6B)."]
    Read4o = 3,
    #[doc = "4: Quad data line SPI. READ4IO (opcode 0xEB)."]
    Read4io = 4,
}
impl From<Readoc> for u8 {
    #[inline(always)]
    fn from(variant: Readoc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Readoc {
    type Ux = u8;
}
impl crate::IsEnum for Readoc {}
#[doc = "Field `READOC` reader - Configure number of data lines and opcode used for reading."]
pub type ReadocR = crate::FieldReader<Readoc>;
impl ReadocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Readoc> {
        match self.bits {
            0 => Some(Readoc::Fastread),
            1 => Some(Readoc::Read2o),
            2 => Some(Readoc::Read2io),
            3 => Some(Readoc::Read4o),
            4 => Some(Readoc::Read4io),
            _ => None,
        }
    }
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    #[inline(always)]
    pub fn is_fastread(&self) -> bool {
        *self == Readoc::Fastread
    }
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    #[inline(always)]
    pub fn is_read2o(&self) -> bool {
        *self == Readoc::Read2o
    }
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    #[inline(always)]
    pub fn is_read2io(&self) -> bool {
        *self == Readoc::Read2io
    }
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    #[inline(always)]
    pub fn is_read4o(&self) -> bool {
        *self == Readoc::Read4o
    }
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    #[inline(always)]
    pub fn is_read4io(&self) -> bool {
        *self == Readoc::Read4io
    }
}
#[doc = "Field `READOC` writer - Configure number of data lines and opcode used for reading."]
pub type ReadocW<'a, REG> = crate::FieldWriter<'a, REG, 3, Readoc>;
impl<'a, REG> ReadocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    #[inline(always)]
    pub fn fastread(self) -> &'a mut crate::W<REG> {
        self.variant(Readoc::Fastread)
    }
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    #[inline(always)]
    pub fn read2o(self) -> &'a mut crate::W<REG> {
        self.variant(Readoc::Read2o)
    }
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    #[inline(always)]
    pub fn read2io(self) -> &'a mut crate::W<REG> {
        self.variant(Readoc::Read2io)
    }
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    #[inline(always)]
    pub fn read4o(self) -> &'a mut crate::W<REG> {
        self.variant(Readoc::Read4o)
    }
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    #[inline(always)]
    pub fn read4io(self) -> &'a mut crate::W<REG> {
        self.variant(Readoc::Read4io)
    }
}
#[doc = "Configure number of data lines and opcode used for writing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Writeoc {
    #[doc = "0: Single data line SPI. PP (opcode 0x02)."]
    Pp = 0,
    #[doc = "1: Dual data line SPI. PP2O (opcode 0xA2)."]
    Pp2o = 1,
    #[doc = "2: Quad data line SPI. PP4O (opcode 0x32)."]
    Pp4o = 2,
    #[doc = "3: Quad data line SPI. PP4IO (opcode 0x38)."]
    Pp4io = 3,
}
impl From<Writeoc> for u8 {
    #[inline(always)]
    fn from(variant: Writeoc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Writeoc {
    type Ux = u8;
}
impl crate::IsEnum for Writeoc {}
#[doc = "Field `WRITEOC` reader - Configure number of data lines and opcode used for writing."]
pub type WriteocR = crate::FieldReader<Writeoc>;
impl WriteocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Writeoc> {
        match self.bits {
            0 => Some(Writeoc::Pp),
            1 => Some(Writeoc::Pp2o),
            2 => Some(Writeoc::Pp4o),
            3 => Some(Writeoc::Pp4io),
            _ => None,
        }
    }
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == Writeoc::Pp
    }
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    #[inline(always)]
    pub fn is_pp2o(&self) -> bool {
        *self == Writeoc::Pp2o
    }
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    #[inline(always)]
    pub fn is_pp4o(&self) -> bool {
        *self == Writeoc::Pp4o
    }
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    #[inline(always)]
    pub fn is_pp4io(&self) -> bool {
        *self == Writeoc::Pp4io
    }
}
#[doc = "Field `WRITEOC` writer - Configure number of data lines and opcode used for writing."]
pub type WriteocW<'a, REG> = crate::FieldWriter<'a, REG, 3, Writeoc>;
impl<'a, REG> WriteocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    #[inline(always)]
    pub fn pp(self) -> &'a mut crate::W<REG> {
        self.variant(Writeoc::Pp)
    }
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    #[inline(always)]
    pub fn pp2o(self) -> &'a mut crate::W<REG> {
        self.variant(Writeoc::Pp2o)
    }
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    #[inline(always)]
    pub fn pp4o(self) -> &'a mut crate::W<REG> {
        self.variant(Writeoc::Pp4o)
    }
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    #[inline(always)]
    pub fn pp4io(self) -> &'a mut crate::W<REG> {
        self.variant(Writeoc::Pp4io)
    }
}
#[doc = "Addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrmode {
    #[doc = "0: 24-bit addressing."]
    _24bit = 0,
    #[doc = "1: 32-bit addressing."]
    _32bit = 1,
}
impl From<Addrmode> for bool {
    #[inline(always)]
    fn from(variant: Addrmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRMODE` reader - Addressing mode."]
pub type AddrmodeR = crate::BitReader<Addrmode>;
impl AddrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrmode {
        match self.bits {
            false => Addrmode::_24bit,
            true => Addrmode::_32bit,
        }
    }
    #[doc = "24-bit addressing."]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == Addrmode::_24bit
    }
    #[doc = "32-bit addressing."]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Addrmode::_32bit
    }
}
#[doc = "Field `ADDRMODE` writer - Addressing mode."]
pub type AddrmodeW<'a, REG> = crate::BitWriter<'a, REG, Addrmode>;
impl<'a, REG> AddrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24-bit addressing."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut crate::W<REG> {
        self.variant(Addrmode::_24bit)
    }
    #[doc = "32-bit addressing."]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Addrmode::_32bit)
    }
}
#[doc = "Enable deep power-down mode (DPM) feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpmenable {
    #[doc = "0: Disable DPM feature."]
    Disable = 0,
    #[doc = "1: Enable DPM feature."]
    Enable = 1,
}
impl From<Dpmenable> for bool {
    #[inline(always)]
    fn from(variant: Dpmenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPMENABLE` reader - Enable deep power-down mode (DPM) feature."]
pub type DpmenableR = crate::BitReader<Dpmenable>;
impl DpmenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpmenable {
        match self.bits {
            false => Dpmenable::Disable,
            true => Dpmenable::Enable,
        }
    }
    #[doc = "Disable DPM feature."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dpmenable::Disable
    }
    #[doc = "Enable DPM feature."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dpmenable::Enable
    }
}
#[doc = "Field `DPMENABLE` writer - Enable deep power-down mode (DPM) feature."]
pub type DpmenableW<'a, REG> = crate::BitWriter<'a, REG, Dpmenable>;
impl<'a, REG> DpmenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DPM feature."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dpmenable::Disable)
    }
    #[doc = "Enable DPM feature."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dpmenable::Enable)
    }
}
#[doc = "Page size for commands PP, PP2O, PP4O and PP4IO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ppsize {
    #[doc = "0: 256 bytes."]
    _256bytes = 0,
    #[doc = "1: 512 bytes."]
    _512bytes = 1,
}
impl From<Ppsize> for bool {
    #[inline(always)]
    fn from(variant: Ppsize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPSIZE` reader - Page size for commands PP, PP2O, PP4O and PP4IO."]
pub type PpsizeR = crate::BitReader<Ppsize>;
impl PpsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ppsize {
        match self.bits {
            false => Ppsize::_256bytes,
            true => Ppsize::_512bytes,
        }
    }
    #[doc = "256 bytes."]
    #[inline(always)]
    pub fn is_256bytes(&self) -> bool {
        *self == Ppsize::_256bytes
    }
    #[doc = "512 bytes."]
    #[inline(always)]
    pub fn is_512bytes(&self) -> bool {
        *self == Ppsize::_512bytes
    }
}
#[doc = "Field `PPSIZE` writer - Page size for commands PP, PP2O, PP4O and PP4IO."]
pub type PpsizeW<'a, REG> = crate::BitWriter<'a, REG, Ppsize>;
impl<'a, REG> PpsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "256 bytes."]
    #[inline(always)]
    pub fn _256bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Ppsize::_256bytes)
    }
    #[doc = "512 bytes."]
    #[inline(always)]
    pub fn _512bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Ppsize::_512bytes)
    }
}
impl R {
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub fn readoc(&self) -> ReadocR {
        ReadocR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub fn writeoc(&self) -> WriteocR {
        WriteocR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline(always)]
    pub fn addrmode(&self) -> AddrmodeR {
        AddrmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub fn dpmenable(&self) -> DpmenableR {
        DpmenableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub fn ppsize(&self) -> PpsizeR {
        PpsizeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub fn readoc(&mut self) -> ReadocW<'_, Ifconfig0Spec> {
        ReadocW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub fn writeoc(&mut self) -> WriteocW<'_, Ifconfig0Spec> {
        WriteocW::new(self, 3)
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline(always)]
    pub fn addrmode(&mut self) -> AddrmodeW<'_, Ifconfig0Spec> {
        AddrmodeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub fn dpmenable(&mut self) -> DpmenableW<'_, Ifconfig0Spec> {
        DpmenableW::new(self, 7)
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub fn ppsize(&mut self) -> PpsizeW<'_, Ifconfig0Spec> {
        PpsizeW::new(self, 12)
    }
}
#[doc = "Interface configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ifconfig0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifconfig0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifconfig0Spec;
impl crate::RegisterSpec for Ifconfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifconfig0::R`](R) reader structure"]
impl crate::Readable for Ifconfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`ifconfig0::W`](W) writer structure"]
impl crate::Writable for Ifconfig0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCONFIG0 to value 0"]
impl crate::Resettable for Ifconfig0Spec {}
