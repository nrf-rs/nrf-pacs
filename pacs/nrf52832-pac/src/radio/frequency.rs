#[doc = "Register `FREQUENCY` reader"]
pub type R = crate::R<FrequencySpec>;
#[doc = "Register `FREQUENCY` writer"]
pub type W = crate::W<FrequencySpec>;
#[doc = "Field `FREQUENCY` reader - Radio channel frequency"]
pub type FrequencyR = crate::FieldReader;
#[doc = "Field `FREQUENCY` writer - Radio channel frequency"]
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Channel map selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Map {
    #[doc = "0: Channel map between 2400 MHZ .. 2500 MHz"]
    Default = 0,
    #[doc = "1: Channel map between 2360 MHZ .. 2460 MHz"]
    Low = 1,
}
impl From<Map> for bool {
    #[inline(always)]
    fn from(variant: Map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAP` reader - Channel map selection."]
pub type MapR = crate::BitReader<Map>;
impl MapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Map {
        match self.bits {
            false => Map::Default,
            true => Map::Low,
        }
    }
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Map::Default
    }
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Map::Low
    }
}
#[doc = "Field `MAP` writer - Channel map selection."]
pub type MapW<'a, REG> = crate::BitWriter<'a, REG, Map>;
impl<'a, REG> MapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Map::Default)
    }
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Map::Low)
    }
}
impl R {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&self) -> MapR {
        MapR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, FrequencySpec> {
        FrequencyW::new(self, 0)
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&mut self) -> MapW<'_, FrequencySpec> {
        MapW::new(self, 8)
    }
}
#[doc = "Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrequencySpec;
impl crate::RegisterSpec for FrequencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frequency::R`](R) reader structure"]
impl crate::Readable for FrequencySpec {}
#[doc = "`write(|w| ..)` method takes [`frequency::W`](W) writer structure"]
impl crate::Writable for FrequencySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x02"]
impl crate::Resettable for FrequencySpec {
    const RESET_VALUE: u32 = 0x02;
}
