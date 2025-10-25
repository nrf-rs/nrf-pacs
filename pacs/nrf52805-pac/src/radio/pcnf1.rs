#[doc = "Register `PCNF1` reader"]
pub type R = crate::R<Pcnf1Spec>;
#[doc = "Register `PCNF1` writer"]
pub type W = crate::W<Pcnf1Spec>;
#[doc = "Field `MAXLEN` reader - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
pub type MaxlenR = crate::FieldReader;
#[doc = "Field `MAXLEN` writer - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
pub type MaxlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STATLEN` reader - Static length in number of bytes"]
pub type StatlenR = crate::FieldReader;
#[doc = "Field `STATLEN` writer - Static length in number of bytes"]
pub type StatlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BALEN` reader - Base address length in number of bytes"]
pub type BalenR = crate::FieldReader;
#[doc = "Field `BALEN` writer - Base address length in number of bytes"]
pub type BalenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Least significant bit on air first"]
    Little = 0,
    #[doc = "1: Most significant bit on air first"]
    Big = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIAN` reader - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
pub type EndianR = crate::BitReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::Little,
            true => Endian::Big,
        }
    }
    #[doc = "Least significant bit on air first"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == Endian::Little
    }
    #[doc = "Most significant bit on air first"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == Endian::Big
    }
}
#[doc = "Field `ENDIAN` writer - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Least significant bit on air first"]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Little)
    }
    #[doc = "Most significant bit on air first"]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Big)
    }
}
#[doc = "Enable or disable packet whitening\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Whiteen {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Whiteen> for bool {
    #[inline(always)]
    fn from(variant: Whiteen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WHITEEN` reader - Enable or disable packet whitening"]
pub type WhiteenR = crate::BitReader<Whiteen>;
impl WhiteenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Whiteen {
        match self.bits {
            false => Whiteen::Disabled,
            true => Whiteen::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Whiteen::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Whiteen::Enabled
    }
}
#[doc = "Field `WHITEEN` writer - Enable or disable packet whitening"]
pub type WhiteenW<'a, REG> = crate::BitWriter<'a, REG, Whiteen>;
impl<'a, REG> WhiteenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Whiteen::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Whiteen::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&self) -> MaxlenR {
        MaxlenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&self) -> StatlenR {
        StatlenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&self) -> BalenR {
        BalenR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&self) -> WhiteenR {
        WhiteenR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MaxlenW<'_, Pcnf1Spec> {
        MaxlenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&mut self) -> StatlenW<'_, Pcnf1Spec> {
        StatlenW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&mut self) -> BalenW<'_, Pcnf1Spec> {
        BalenW::new(self, 16)
    }
    #[doc = "Bit 24 - On-air endianness of packet, this applies to the S0, LENGTH, S1, and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&mut self) -> EndianW<'_, Pcnf1Spec> {
        EndianW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&mut self) -> WhiteenW<'_, Pcnf1Spec> {
        WhiteenW::new(self, 25)
    }
}
#[doc = "Packet configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcnf1Spec;
impl crate::RegisterSpec for Pcnf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnf1::R`](R) reader structure"]
impl crate::Readable for Pcnf1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcnf1::W`](W) writer structure"]
impl crate::Writable for Pcnf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNF1 to value 0"]
impl crate::Resettable for Pcnf1Spec {}
