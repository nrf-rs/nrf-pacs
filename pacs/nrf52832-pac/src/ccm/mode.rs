#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "The mode of operation to be used\n\nValue on reset: 1"]
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
#[doc = "Field `MODE` reader - The mode of operation to be used"]
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
#[doc = "Field `MODE` writer - The mode of operation to be used"]
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
#[doc = "Data rate that the CCM shall run in synch with\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datarate {
    #[doc = "0: In synch with 1 Mbit data rate"]
    _1mbit = 0,
    #[doc = "1: In synch with 2 Mbit data rate"]
    _2mbit = 1,
}
impl From<Datarate> for bool {
    #[inline(always)]
    fn from(variant: Datarate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATARATE` reader - Data rate that the CCM shall run in synch with"]
pub type DatarateR = crate::BitReader<Datarate>;
impl DatarateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datarate {
        match self.bits {
            false => Datarate::_1mbit,
            true => Datarate::_2mbit,
        }
    }
    #[doc = "In synch with 1 Mbit data rate"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == Datarate::_1mbit
    }
    #[doc = "In synch with 2 Mbit data rate"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == Datarate::_2mbit
    }
}
#[doc = "Field `DATARATE` writer - Data rate that the CCM shall run in synch with"]
pub type DatarateW<'a, REG> = crate::BitWriter<'a, REG, Datarate>;
impl<'a, REG> DatarateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In synch with 1 Mbit data rate"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_1mbit)
    }
    #[doc = "In synch with 2 Mbit data rate"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_2mbit)
    }
}
#[doc = "Packet length configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Length {
    #[doc = "0: Default length. Effective length of LENGTH field is 5-bit"]
    Default = 0,
    #[doc = "1: Extended length. Effective length of LENGTH field is 8-bit"]
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
    #[doc = "Default length. Effective length of LENGTH field is 5-bit"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Length::Default
    }
    #[doc = "Extended length. Effective length of LENGTH field is 8-bit"]
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
    #[doc = "Default length. Effective length of LENGTH field is 5-bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Length::Default)
    }
    #[doc = "Extended length. Effective length of LENGTH field is 8-bit"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Length::Extended)
    }
}
impl R {
    #[doc = "Bit 0 - The mode of operation to be used"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Data rate that the CCM shall run in synch with"]
    #[inline(always)]
    pub fn datarate(&self) -> DatarateR {
        DatarateR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The mode of operation to be used"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModeSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 16 - Data rate that the CCM shall run in synch with"]
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
