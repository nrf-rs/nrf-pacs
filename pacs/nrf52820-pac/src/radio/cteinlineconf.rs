#[doc = "Register `CTEINLINECONF` reader"]
pub type R = crate::R<CteinlineconfSpec>;
#[doc = "Register `CTEINLINECONF` writer"]
pub type W = crate::W<CteinlineconfSpec>;
#[doc = "Enable parsing of CTEInfo from received packet in BLE modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cteinlinectrlen {
    #[doc = "1: Parsing of CTEInfo is enabled"]
    Enabled = 1,
    #[doc = "0: Parsing of CTEInfo is disabled"]
    Disabled = 0,
}
impl From<Cteinlinectrlen> for bool {
    #[inline(always)]
    fn from(variant: Cteinlinectrlen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEINLINECTRLEN` reader - Enable parsing of CTEInfo from received packet in BLE modes"]
pub type CteinlinectrlenR = crate::BitReader<Cteinlinectrlen>;
impl CteinlinectrlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cteinlinectrlen {
        match self.bits {
            true => Cteinlinectrlen::Enabled,
            false => Cteinlinectrlen::Disabled,
        }
    }
    #[doc = "Parsing of CTEInfo is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cteinlinectrlen::Enabled
    }
    #[doc = "Parsing of CTEInfo is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cteinlinectrlen::Disabled
    }
}
#[doc = "Field `CTEINLINECTRLEN` writer - Enable parsing of CTEInfo from received packet in BLE modes"]
pub type CteinlinectrlenW<'a, REG> = crate::BitWriter<'a, REG, Cteinlinectrlen>;
impl<'a, REG> CteinlinectrlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parsing of CTEInfo is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinectrlen::Enabled)
    }
    #[doc = "Parsing of CTEInfo is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinectrlen::Disabled)
    }
}
#[doc = "CTEInfo is S1 byte or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cteinfoins1 {
    #[doc = "1: CTEInfo is in S1 byte (data PDU)"]
    InS1 = 1,
    #[doc = "0: CTEInfo is NOT in S1 byte (advertising PDU)"]
    NotInS1 = 0,
}
impl From<Cteinfoins1> for bool {
    #[inline(always)]
    fn from(variant: Cteinfoins1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEINFOINS1` reader - CTEInfo is S1 byte or not"]
pub type Cteinfoins1R = crate::BitReader<Cteinfoins1>;
impl Cteinfoins1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cteinfoins1 {
        match self.bits {
            true => Cteinfoins1::InS1,
            false => Cteinfoins1::NotInS1,
        }
    }
    #[doc = "CTEInfo is in S1 byte (data PDU)"]
    #[inline(always)]
    pub fn is_in_s1(&self) -> bool {
        *self == Cteinfoins1::InS1
    }
    #[doc = "CTEInfo is NOT in S1 byte (advertising PDU)"]
    #[inline(always)]
    pub fn is_not_in_s1(&self) -> bool {
        *self == Cteinfoins1::NotInS1
    }
}
#[doc = "Field `CTEINFOINS1` writer - CTEInfo is S1 byte or not"]
pub type Cteinfoins1W<'a, REG> = crate::BitWriter<'a, REG, Cteinfoins1>;
impl<'a, REG> Cteinfoins1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTEInfo is in S1 byte (data PDU)"]
    #[inline(always)]
    pub fn in_s1(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinfoins1::InS1)
    }
    #[doc = "CTEInfo is NOT in S1 byte (advertising PDU)"]
    #[inline(always)]
    pub fn not_in_s1(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinfoins1::NotInS1)
    }
}
#[doc = "Sampling/switching if CRC is not OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cteerrorhandling {
    #[doc = "1: Sampling and antenna switching also when CRC is not OK"]
    Yes = 1,
    #[doc = "0: No sampling and antenna switching when CRC is not OK"]
    No = 0,
}
impl From<Cteerrorhandling> for bool {
    #[inline(always)]
    fn from(variant: Cteerrorhandling) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEERRORHANDLING` reader - Sampling/switching if CRC is not OK"]
pub type CteerrorhandlingR = crate::BitReader<Cteerrorhandling>;
impl CteerrorhandlingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cteerrorhandling {
        match self.bits {
            true => Cteerrorhandling::Yes,
            false => Cteerrorhandling::No,
        }
    }
    #[doc = "Sampling and antenna switching also when CRC is not OK"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Cteerrorhandling::Yes
    }
    #[doc = "No sampling and antenna switching when CRC is not OK"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Cteerrorhandling::No
    }
}
#[doc = "Field `CTEERRORHANDLING` writer - Sampling/switching if CRC is not OK"]
pub type CteerrorhandlingW<'a, REG> = crate::BitWriter<'a, REG, Cteerrorhandling>;
impl<'a, REG> CteerrorhandlingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling and antenna switching also when CRC is not OK"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut crate::W<REG> {
        self.variant(Cteerrorhandling::Yes)
    }
    #[doc = "No sampling and antenna switching when CRC is not OK"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Cteerrorhandling::No)
    }
}
#[doc = "Max range of CTETime\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctetimevalidrange {
    #[doc = "0: 20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
    _20 = 0,
    #[doc = "1: 31 in 8 us unit"]
    _31 = 1,
    #[doc = "2: 63 in 8 us unit"]
    _63 = 2,
}
impl From<Ctetimevalidrange> for u8 {
    #[inline(always)]
    fn from(variant: Ctetimevalidrange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctetimevalidrange {
    type Ux = u8;
}
impl crate::IsEnum for Ctetimevalidrange {}
#[doc = "Field `CTETIMEVALIDRANGE` reader - Max range of CTETime"]
pub type CtetimevalidrangeR = crate::FieldReader<Ctetimevalidrange>;
impl CtetimevalidrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctetimevalidrange> {
        match self.bits {
            0 => Some(Ctetimevalidrange::_20),
            1 => Some(Ctetimevalidrange::_31),
            2 => Some(Ctetimevalidrange::_63),
            _ => None,
        }
    }
    #[doc = "20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == Ctetimevalidrange::_20
    }
    #[doc = "31 in 8 us unit"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == Ctetimevalidrange::_31
    }
    #[doc = "63 in 8 us unit"]
    #[inline(always)]
    pub fn is_63(&self) -> bool {
        *self == Ctetimevalidrange::_63
    }
}
#[doc = "Field `CTETIMEVALIDRANGE` writer - Max range of CTETime"]
pub type CtetimevalidrangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctetimevalidrange>;
impl<'a, REG> CtetimevalidrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 in 8 us unit (default) Set to 20 if parsed CTETime is larger than 20"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut crate::W<REG> {
        self.variant(Ctetimevalidrange::_20)
    }
    #[doc = "31 in 8 us unit"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut crate::W<REG> {
        self.variant(Ctetimevalidrange::_31)
    }
    #[doc = "63 in 8 us unit"]
    #[inline(always)]
    pub fn _63(self) -> &'a mut crate::W<REG> {
        self.variant(Ctetimevalidrange::_63)
    }
}
#[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cteinlinerxmode1us {
    #[doc = "1: 4 us"]
    _4us = 1,
    #[doc = "2: 2 us"]
    _2us = 2,
    #[doc = "3: 1 us"]
    _1us = 3,
    #[doc = "4: 0.5 us"]
    _500ns = 4,
    #[doc = "5: 0.25 us"]
    _250ns = 5,
    #[doc = "6: 0.125 us"]
    _125ns = 6,
}
impl From<Cteinlinerxmode1us> for u8 {
    #[inline(always)]
    fn from(variant: Cteinlinerxmode1us) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cteinlinerxmode1us {
    type Ux = u8;
}
impl crate::IsEnum for Cteinlinerxmode1us {}
#[doc = "Field `CTEINLINERXMODE1US` reader - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type Cteinlinerxmode1usR = crate::FieldReader<Cteinlinerxmode1us>;
impl Cteinlinerxmode1usR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cteinlinerxmode1us> {
        match self.bits {
            1 => Some(Cteinlinerxmode1us::_4us),
            2 => Some(Cteinlinerxmode1us::_2us),
            3 => Some(Cteinlinerxmode1us::_1us),
            4 => Some(Cteinlinerxmode1us::_500ns),
            5 => Some(Cteinlinerxmode1us::_250ns),
            6 => Some(Cteinlinerxmode1us::_125ns),
            _ => None,
        }
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == Cteinlinerxmode1us::_4us
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == Cteinlinerxmode1us::_2us
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == Cteinlinerxmode1us::_1us
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == Cteinlinerxmode1us::_500ns
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == Cteinlinerxmode1us::_250ns
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == Cteinlinerxmode1us::_125ns
    }
}
#[doc = "Field `CTEINLINERXMODE1US` writer - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type Cteinlinerxmode1usW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cteinlinerxmode1us>;
impl<'a, REG> Cteinlinerxmode1usW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode1us::_4us)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode1us::_2us)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode1us::_1us)
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode1us::_500ns)
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode1us::_250ns)
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode1us::_125ns)
    }
}
#[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cteinlinerxmode2us {
    #[doc = "1: 4 us"]
    _4us = 1,
    #[doc = "2: 2 us"]
    _2us = 2,
    #[doc = "3: 1 us"]
    _1us = 3,
    #[doc = "4: 0.5 us"]
    _500ns = 4,
    #[doc = "5: 0.25 us"]
    _250ns = 5,
    #[doc = "6: 0.125 us"]
    _125ns = 6,
}
impl From<Cteinlinerxmode2us> for u8 {
    #[inline(always)]
    fn from(variant: Cteinlinerxmode2us) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cteinlinerxmode2us {
    type Ux = u8;
}
impl crate::IsEnum for Cteinlinerxmode2us {}
#[doc = "Field `CTEINLINERXMODE2US` reader - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type Cteinlinerxmode2usR = crate::FieldReader<Cteinlinerxmode2us>;
impl Cteinlinerxmode2usR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cteinlinerxmode2us> {
        match self.bits {
            1 => Some(Cteinlinerxmode2us::_4us),
            2 => Some(Cteinlinerxmode2us::_2us),
            3 => Some(Cteinlinerxmode2us::_1us),
            4 => Some(Cteinlinerxmode2us::_500ns),
            5 => Some(Cteinlinerxmode2us::_250ns),
            6 => Some(Cteinlinerxmode2us::_125ns),
            _ => None,
        }
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == Cteinlinerxmode2us::_4us
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == Cteinlinerxmode2us::_2us
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == Cteinlinerxmode2us::_1us
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == Cteinlinerxmode2us::_500ns
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == Cteinlinerxmode2us::_250ns
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == Cteinlinerxmode2us::_125ns
    }
}
#[doc = "Field `CTEINLINERXMODE2US` writer - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
pub type Cteinlinerxmode2usW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cteinlinerxmode2us>;
impl<'a, REG> Cteinlinerxmode2usW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode2us::_4us)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode2us::_2us)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode2us::_1us)
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode2us::_500ns)
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode2us::_250ns)
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut crate::W<REG> {
        self.variant(Cteinlinerxmode2us::_125ns)
    }
}
#[doc = "Field `S0CONF` reader - S0 bit pattern to match"]
pub type S0confR = crate::FieldReader;
#[doc = "Field `S0CONF` writer - S0 bit pattern to match"]
pub type S0confW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `S0MASK` reader - S0 bit mask to set which bit to match"]
pub type S0maskR = crate::FieldReader;
#[doc = "Field `S0MASK` writer - S0 bit mask to set which bit to match"]
pub type S0maskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn cteinlinectrlen(&self) -> CteinlinectrlenR {
        CteinlinectrlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn cteinfoins1(&self) -> Cteinfoins1R {
        Cteinfoins1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn cteerrorhandling(&self) -> CteerrorhandlingR {
        CteerrorhandlingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Max range of CTETime"]
    #[inline(always)]
    pub fn ctetimevalidrange(&self) -> CtetimevalidrangeR {
        CtetimevalidrangeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode1us(&self) -> Cteinlinerxmode1usR {
        Cteinlinerxmode1usR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode2us(&self) -> Cteinlinerxmode2usR {
        Cteinlinerxmode2usR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - S0 bit pattern to match"]
    #[inline(always)]
    pub fn s0conf(&self) -> S0confR {
        S0confR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn s0mask(&self) -> S0maskR {
        S0maskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn cteinlinectrlen(&mut self) -> CteinlinectrlenW<'_, CteinlineconfSpec> {
        CteinlinectrlenW::new(self, 0)
    }
    #[doc = "Bit 3 - CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn cteinfoins1(&mut self) -> Cteinfoins1W<'_, CteinlineconfSpec> {
        Cteinfoins1W::new(self, 3)
    }
    #[doc = "Bit 4 - Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn cteerrorhandling(&mut self) -> CteerrorhandlingW<'_, CteinlineconfSpec> {
        CteerrorhandlingW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Max range of CTETime"]
    #[inline(always)]
    pub fn ctetimevalidrange(&mut self) -> CtetimevalidrangeW<'_, CteinlineconfSpec> {
        CtetimevalidrangeW::new(self, 6)
    }
    #[doc = "Bits 10:12 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode1us(&mut self) -> Cteinlinerxmode1usW<'_, CteinlineconfSpec> {
        Cteinlinerxmode1usW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set."]
    #[inline(always)]
    pub fn cteinlinerxmode2us(&mut self) -> Cteinlinerxmode2usW<'_, CteinlineconfSpec> {
        Cteinlinerxmode2usW::new(self, 13)
    }
    #[doc = "Bits 16:23 - S0 bit pattern to match"]
    #[inline(always)]
    pub fn s0conf(&mut self) -> S0confW<'_, CteinlineconfSpec> {
        S0confW::new(self, 16)
    }
    #[doc = "Bits 24:31 - S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn s0mask(&mut self) -> S0maskW<'_, CteinlineconfSpec> {
        S0maskW::new(self, 24)
    }
}
#[doc = "Configuration for CTE inline mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cteinlineconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cteinlineconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CteinlineconfSpec;
impl crate::RegisterSpec for CteinlineconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cteinlineconf::R`](R) reader structure"]
impl crate::Readable for CteinlineconfSpec {}
#[doc = "`write(|w| ..)` method takes [`cteinlineconf::W`](W) writer structure"]
impl crate::Writable for CteinlineconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTEINLINECONF to value 0x2800"]
impl crate::Resettable for CteinlineconfSpec {
    const RESET_VALUE: u32 = 0x2800;
}
