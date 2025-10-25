#[doc = "Register `PCNF0` reader"]
pub type R = crate::R<Pcnf0Spec>;
#[doc = "Register `PCNF0` writer"]
pub type W = crate::W<Pcnf0Spec>;
#[doc = "Field `LFLEN` reader - Length on air of LENGTH field in number of bits"]
pub type LflenR = crate::FieldReader;
#[doc = "Field `LFLEN` writer - Length on air of LENGTH field in number of bits"]
pub type LflenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `S0LEN` reader - Length on air of S0 field in number of bytes"]
pub type S0lenR = crate::BitReader;
#[doc = "Field `S0LEN` writer - Length on air of S0 field in number of bytes"]
pub type S0lenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1LEN` reader - Length on air of S1 field in number of bits"]
pub type S1lenR = crate::FieldReader;
#[doc = "Field `S1LEN` writer - Length on air of S1 field in number of bits"]
pub type S1lenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Include or exclude S1 field in RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1incl {
    #[doc = "0: Include S1 field in RAM only if S1LEN &gt; 0"]
    Automatic = 0,
    #[doc = "1: Always include S1 field in RAM independent of S1LEN"]
    Include = 1,
}
impl From<S1incl> for bool {
    #[inline(always)]
    fn from(variant: S1incl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1INCL` reader - Include or exclude S1 field in RAM"]
pub type S1inclR = crate::BitReader<S1incl>;
impl S1inclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1incl {
        match self.bits {
            false => S1incl::Automatic,
            true => S1incl::Include,
        }
    }
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == S1incl::Automatic
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == S1incl::Include
    }
}
#[doc = "Field `S1INCL` writer - Include or exclude S1 field in RAM"]
pub type S1inclW<'a, REG> = crate::BitWriter<'a, REG, S1incl>;
impl<'a, REG> S1inclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(S1incl::Automatic)
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(S1incl::Include)
    }
}
#[doc = "Field `CILEN` reader - Length of code indicator - long range"]
pub type CilenR = crate::FieldReader;
#[doc = "Field `CILEN` writer - Length of code indicator - long range"]
pub type CilenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Length of preamble on air. Decision point: TASKS_START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plen {
    #[doc = "0: 8-bit preamble"]
    _8bit = 0,
    #[doc = "1: 16-bit preamble"]
    _16bit = 1,
    #[doc = "2: 32-bit zero preamble - used for IEEE 802.15.4"]
    _32bitZero = 2,
    #[doc = "3: Preamble - used for BLE long range"]
    LongRange = 3,
}
impl From<Plen> for u8 {
    #[inline(always)]
    fn from(variant: Plen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plen {
    type Ux = u8;
}
impl crate::IsEnum for Plen {}
#[doc = "Field `PLEN` reader - Length of preamble on air. Decision point: TASKS_START task"]
pub type PlenR = crate::FieldReader<Plen>;
impl PlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plen {
        match self.bits {
            0 => Plen::_8bit,
            1 => Plen::_16bit,
            2 => Plen::_32bitZero,
            3 => Plen::LongRange,
            _ => unreachable!(),
        }
    }
    #[doc = "8-bit preamble"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Plen::_8bit
    }
    #[doc = "16-bit preamble"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Plen::_16bit
    }
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    #[inline(always)]
    pub fn is_32bit_zero(&self) -> bool {
        *self == Plen::_32bitZero
    }
    #[doc = "Preamble - used for BLE long range"]
    #[inline(always)]
    pub fn is_long_range(&self) -> bool {
        *self == Plen::LongRange
    }
}
#[doc = "Field `PLEN` writer - Length of preamble on air. Decision point: TASKS_START task"]
pub type PlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Plen, crate::Safe>;
impl<'a, REG> PlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit preamble"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Plen::_8bit)
    }
    #[doc = "16-bit preamble"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Plen::_16bit)
    }
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    #[inline(always)]
    pub fn _32bit_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Plen::_32bitZero)
    }
    #[doc = "Preamble - used for BLE long range"]
    #[inline(always)]
    pub fn long_range(self) -> &'a mut crate::W<REG> {
        self.variant(Plen::LongRange)
    }
}
#[doc = "Indicates if LENGTH field contains CRC or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcinc {
    #[doc = "0: LENGTH does not contain CRC"]
    Exclude = 0,
    #[doc = "1: LENGTH includes CRC"]
    Include = 1,
}
impl From<Crcinc> for bool {
    #[inline(always)]
    fn from(variant: Crcinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCINC` reader - Indicates if LENGTH field contains CRC or not"]
pub type CrcincR = crate::BitReader<Crcinc>;
impl CrcincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcinc {
        match self.bits {
            false => Crcinc::Exclude,
            true => Crcinc::Include,
        }
    }
    #[doc = "LENGTH does not contain CRC"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Crcinc::Exclude
    }
    #[doc = "LENGTH includes CRC"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Crcinc::Include
    }
}
#[doc = "Field `CRCINC` writer - Indicates if LENGTH field contains CRC or not"]
pub type CrcincW<'a, REG> = crate::BitWriter<'a, REG, Crcinc>;
impl<'a, REG> CrcincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LENGTH does not contain CRC"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Crcinc::Exclude)
    }
    #[doc = "LENGTH includes CRC"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Crcinc::Include)
    }
}
#[doc = "Field `TERMLEN` reader - Length of TERM field in Long Range operation"]
pub type TermlenR = crate::FieldReader;
#[doc = "Field `TERMLEN` writer - Length of TERM field in Long Range operation"]
pub type TermlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits"]
    #[inline(always)]
    pub fn lflen(&self) -> LflenR {
        LflenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes"]
    #[inline(always)]
    pub fn s0len(&self) -> S0lenR {
        S0lenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits"]
    #[inline(always)]
    pub fn s1len(&self) -> S1lenR {
        S1lenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&self) -> S1inclR {
        S1inclR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Length of code indicator - long range"]
    #[inline(always)]
    pub fn cilen(&self) -> CilenR {
        CilenR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&self) -> PlenR {
        PlenR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub fn crcinc(&self) -> CrcincR {
        CrcincR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub fn termlen(&self) -> TermlenR {
        TermlenR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits"]
    #[inline(always)]
    pub fn lflen(&mut self) -> LflenW<'_, Pcnf0Spec> {
        LflenW::new(self, 0)
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes"]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0lenW<'_, Pcnf0Spec> {
        S0lenW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits"]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1lenW<'_, Pcnf0Spec> {
        S1lenW::new(self, 16)
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&mut self) -> S1inclW<'_, Pcnf0Spec> {
        S1inclW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Length of code indicator - long range"]
    #[inline(always)]
    pub fn cilen(&mut self) -> CilenW<'_, Pcnf0Spec> {
        CilenW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&mut self) -> PlenW<'_, Pcnf0Spec> {
        PlenW::new(self, 24)
    }
    #[doc = "Bit 26 - Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub fn crcinc(&mut self) -> CrcincW<'_, Pcnf0Spec> {
        CrcincW::new(self, 26)
    }
    #[doc = "Bits 29:30 - Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub fn termlen(&mut self) -> TermlenW<'_, Pcnf0Spec> {
        TermlenW::new(self, 29)
    }
}
#[doc = "Packet configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcnf0Spec;
impl crate::RegisterSpec for Pcnf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnf0::R`](R) reader structure"]
impl crate::Readable for Pcnf0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcnf0::W`](W) writer structure"]
impl crate::Writable for Pcnf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNF0 to value 0"]
impl crate::Resettable for Pcnf0Spec {}
