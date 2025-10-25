#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: AES CCM packet encryption mode"]
    Encryption = 0,
    #[doc = "1: AES CCM packet decryption mode"]
    Decryption = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Encryption,
            true => Mode::Decryption,
        }
    }
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == Mode::Encryption
    }
    #[doc = "AES CCM packet decryption mode"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == Mode::Decryption
    }
}
#[doc = "Field `MODE` writer - The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Encryption)
    }
    #[doc = "AES CCM packet decryption mode"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Decryption)
    }
}
#[doc = "Radio data rate that the CCM shall run synchronous with\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datarate {
    #[doc = "0: 1 Mbps"]
    _1mbit = 0,
    #[doc = "1: 2 Mbps"]
    _2mbit = 1,
    #[doc = "2: 125 Kbps"]
    _125kbps = 2,
    #[doc = "3: 500 Kbps"]
    _500kbps = 3,
}
impl From<Datarate> for u8 {
    #[inline(always)]
    fn from(variant: Datarate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datarate {
    type Ux = u8;
}
impl crate::IsEnum for Datarate {}
#[doc = "Field `DATARATE` reader - Radio data rate that the CCM shall run synchronous with"]
pub type DatarateR = crate::FieldReader<Datarate>;
impl DatarateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datarate {
        match self.bits {
            0 => Datarate::_1mbit,
            1 => Datarate::_2mbit,
            2 => Datarate::_125kbps,
            3 => Datarate::_500kbps,
            _ => unreachable!(),
        }
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == Datarate::_1mbit
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == Datarate::_2mbit
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn is_125kbps(&self) -> bool {
        *self == Datarate::_125kbps
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn is_500kbps(&self) -> bool {
        *self == Datarate::_500kbps
    }
}
#[doc = "Field `DATARATE` writer - Radio data rate that the CCM shall run synchronous with"]
pub type DatarateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datarate, crate::Safe>;
impl<'a, REG> DatarateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_1mbit)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_2mbit)
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn _125kbps(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_125kbps)
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn _500kbps(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_500kbps)
    }
}
#[doc = "Packet length configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Length {
    #[doc = "0: Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    Default = 0,
    #[doc = "1: Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    Extended = 1,
}
impl From<Length> for bool {
    #[inline(always)]
    fn from(variant: Length) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LENGTH` reader - Packet length configuration"]
pub type LengthR = crate::BitReader<Length>;
impl LengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Length {
        match self.bits {
            false => Length::Default,
            true => Length::Extended,
        }
    }
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Length::Default
    }
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Length::Extended
    }
}
#[doc = "Field `LENGTH` writer - Packet length configuration"]
pub type LengthW<'a, REG> = crate::BitWriter<'a, REG, Length>;
impl<'a, REG> LengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Length::Default)
    }
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Length::Extended)
    }
}
impl R {
    #[doc = "Bit 0 - The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub fn datarate(&self) -> DatarateR {
        DatarateR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModeSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub fn datarate(&mut self) -> DatarateW<'_, ModeSpec> {
        DatarateW::new(self, 16)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&mut self) -> LengthW<'_, ModeSpec> {
        LengthW::new(self, 24)
    }
}
#[doc = "Operation mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x01;
}
