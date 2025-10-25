#[doc = "Register `DFECTRL1` reader"]
pub type R = crate::R<Dfectrl1Spec>;
#[doc = "Register `DFECTRL1` writer"]
pub type W = crate::W<Dfectrl1Spec>;
#[doc = "Field `NUMBEROF8US` reader - Length of the AoA/AoD procedure in number of 8 us units"]
pub type Numberof8usR = crate::FieldReader;
#[doc = "Field `NUMBEROF8US` writer - Length of the AoA/AoD procedure in number of 8 us units"]
pub type Numberof8usW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Add CTE extension and do antenna switching/sampling in this extension\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfeinextension {
    #[doc = "1: AoA/AoD procedure triggered at end of CRC"]
    Crc = 1,
    #[doc = "0: Antenna switching/sampling is done in the packet payload"]
    Payload = 0,
}
impl From<Dfeinextension> for bool {
    #[inline(always)]
    fn from(variant: Dfeinextension) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEINEXTENSION` reader - Add CTE extension and do antenna switching/sampling in this extension"]
pub type DfeinextensionR = crate::BitReader<Dfeinextension>;
impl DfeinextensionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfeinextension {
        match self.bits {
            true => Dfeinextension::Crc,
            false => Dfeinextension::Payload,
        }
    }
    #[doc = "AoA/AoD procedure triggered at end of CRC"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == Dfeinextension::Crc
    }
    #[doc = "Antenna switching/sampling is done in the packet payload"]
    #[inline(always)]
    pub fn is_payload(&self) -> bool {
        *self == Dfeinextension::Payload
    }
}
#[doc = "Field `DFEINEXTENSION` writer - Add CTE extension and do antenna switching/sampling in this extension"]
pub type DfeinextensionW<'a, REG> = crate::BitWriter<'a, REG, Dfeinextension>;
impl<'a, REG> DfeinextensionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AoA/AoD procedure triggered at end of CRC"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(Dfeinextension::Crc)
    }
    #[doc = "Antenna switching/sampling is done in the packet payload"]
    #[inline(always)]
    pub fn payload(self) -> &'a mut crate::W<REG> {
        self.variant(Dfeinextension::Payload)
    }
}
#[doc = "Interval between every time the antenna is changed in the SWITCHING state\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tswitchspacing {
    #[doc = "1: 4 us"]
    _4us = 1,
    #[doc = "2: 2 us"]
    _2us = 2,
    #[doc = "3: 1 us"]
    _1us = 3,
}
impl From<Tswitchspacing> for u8 {
    #[inline(always)]
    fn from(variant: Tswitchspacing) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tswitchspacing {
    type Ux = u8;
}
impl crate::IsEnum for Tswitchspacing {}
#[doc = "Field `TSWITCHSPACING` reader - Interval between every time the antenna is changed in the SWITCHING state"]
pub type TswitchspacingR = crate::FieldReader<Tswitchspacing>;
impl TswitchspacingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tswitchspacing> {
        match self.bits {
            1 => Some(Tswitchspacing::_4us),
            2 => Some(Tswitchspacing::_2us),
            3 => Some(Tswitchspacing::_1us),
            _ => None,
        }
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == Tswitchspacing::_4us
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == Tswitchspacing::_2us
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == Tswitchspacing::_1us
    }
}
#[doc = "Field `TSWITCHSPACING` writer - Interval between every time the antenna is changed in the SWITCHING state"]
pub type TswitchspacingW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tswitchspacing>;
impl<'a, REG> TswitchspacingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut crate::W<REG> {
        self.variant(Tswitchspacing::_4us)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut crate::W<REG> {
        self.variant(Tswitchspacing::_2us)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut crate::W<REG> {
        self.variant(Tswitchspacing::_1us)
    }
}
#[doc = "Interval between samples in the REFERENCE period\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsamplespacingref {
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
impl From<Tsamplespacingref> for u8 {
    #[inline(always)]
    fn from(variant: Tsamplespacingref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsamplespacingref {
    type Ux = u8;
}
impl crate::IsEnum for Tsamplespacingref {}
#[doc = "Field `TSAMPLESPACINGREF` reader - Interval between samples in the REFERENCE period"]
pub type TsamplespacingrefR = crate::FieldReader<Tsamplespacingref>;
impl TsamplespacingrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsamplespacingref> {
        match self.bits {
            1 => Some(Tsamplespacingref::_4us),
            2 => Some(Tsamplespacingref::_2us),
            3 => Some(Tsamplespacingref::_1us),
            4 => Some(Tsamplespacingref::_500ns),
            5 => Some(Tsamplespacingref::_250ns),
            6 => Some(Tsamplespacingref::_125ns),
            _ => None,
        }
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == Tsamplespacingref::_4us
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == Tsamplespacingref::_2us
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == Tsamplespacingref::_1us
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == Tsamplespacingref::_500ns
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == Tsamplespacingref::_250ns
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == Tsamplespacingref::_125ns
    }
}
#[doc = "Field `TSAMPLESPACINGREF` writer - Interval between samples in the REFERENCE period"]
pub type TsamplespacingrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsamplespacingref>;
impl<'a, REG> TsamplespacingrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacingref::_4us)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacingref::_2us)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacingref::_1us)
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacingref::_500ns)
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacingref::_250ns)
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacingref::_125ns)
    }
}
#[doc = "Whether to sample I/Q or magnitude/phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampletype {
    #[doc = "0: Complex samples in I and Q"]
    Iq = 0,
    #[doc = "1: Complex samples as magnitude and phase"]
    MagPhase = 1,
}
impl From<Sampletype> for bool {
    #[inline(always)]
    fn from(variant: Sampletype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLETYPE` reader - Whether to sample I/Q or magnitude/phase"]
pub type SampletypeR = crate::BitReader<Sampletype>;
impl SampletypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampletype {
        match self.bits {
            false => Sampletype::Iq,
            true => Sampletype::MagPhase,
        }
    }
    #[doc = "Complex samples in I and Q"]
    #[inline(always)]
    pub fn is_iq(&self) -> bool {
        *self == Sampletype::Iq
    }
    #[doc = "Complex samples as magnitude and phase"]
    #[inline(always)]
    pub fn is_mag_phase(&self) -> bool {
        *self == Sampletype::MagPhase
    }
}
#[doc = "Field `SAMPLETYPE` writer - Whether to sample I/Q or magnitude/phase"]
pub type SampletypeW<'a, REG> = crate::BitWriter<'a, REG, Sampletype>;
impl<'a, REG> SampletypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Complex samples in I and Q"]
    #[inline(always)]
    pub fn iq(self) -> &'a mut crate::W<REG> {
        self.variant(Sampletype::Iq)
    }
    #[doc = "Complex samples as magnitude and phase"]
    #[inline(always)]
    pub fn mag_phase(self) -> &'a mut crate::W<REG> {
        self.variant(Sampletype::MagPhase)
    }
}
#[doc = "Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsamplespacing {
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
impl From<Tsamplespacing> for u8 {
    #[inline(always)]
    fn from(variant: Tsamplespacing) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsamplespacing {
    type Ux = u8;
}
impl crate::IsEnum for Tsamplespacing {}
#[doc = "Field `TSAMPLESPACING` reader - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
pub type TsamplespacingR = crate::FieldReader<Tsamplespacing>;
impl TsamplespacingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsamplespacing> {
        match self.bits {
            1 => Some(Tsamplespacing::_4us),
            2 => Some(Tsamplespacing::_2us),
            3 => Some(Tsamplespacing::_1us),
            4 => Some(Tsamplespacing::_500ns),
            5 => Some(Tsamplespacing::_250ns),
            6 => Some(Tsamplespacing::_125ns),
            _ => None,
        }
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == Tsamplespacing::_4us
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == Tsamplespacing::_2us
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == Tsamplespacing::_1us
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == Tsamplespacing::_500ns
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == Tsamplespacing::_250ns
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == Tsamplespacing::_125ns
    }
}
#[doc = "Field `TSAMPLESPACING` writer - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
pub type TsamplespacingW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsamplespacing>;
impl<'a, REG> TsamplespacingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacing::_4us)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacing::_2us)
    }
    #[doc = "1 us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacing::_1us)
    }
    #[doc = "0.5 us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacing::_500ns)
    }
    #[doc = "0.25 us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacing::_250ns)
    }
    #[doc = "0.125 us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut crate::W<REG> {
        self.variant(Tsamplespacing::_125ns)
    }
}
#[doc = "Field `AGCBACKOFFGAIN` reader - Gain will be lowered by the specified number of gain steps at the start of CTE"]
pub type AgcbackoffgainR = crate::FieldReader;
#[doc = "Field `AGCBACKOFFGAIN` writer - Gain will be lowered by the specified number of gain steps at the start of CTE"]
pub type AgcbackoffgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&self) -> Numberof8usR {
        Numberof8usR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&self) -> DfeinextensionR {
        DfeinextensionR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&self) -> TswitchspacingR {
        TswitchspacingR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&self) -> TsamplespacingrefR {
        TsamplespacingrefR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&self) -> SampletypeR {
        SampletypeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&self) -> TsamplespacingR {
        TsamplespacingR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&self) -> AgcbackoffgainR {
        AgcbackoffgainR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Length of the AoA/AoD procedure in number of 8 us units"]
    #[inline(always)]
    pub fn numberof8us(&mut self) -> Numberof8usW<'_, Dfectrl1Spec> {
        Numberof8usW::new(self, 0)
    }
    #[doc = "Bit 7 - Add CTE extension and do antenna switching/sampling in this extension"]
    #[inline(always)]
    pub fn dfeinextension(&mut self) -> DfeinextensionW<'_, Dfectrl1Spec> {
        DfeinextensionW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Interval between every time the antenna is changed in the SWITCHING state"]
    #[inline(always)]
    pub fn tswitchspacing(&mut self) -> TswitchspacingW<'_, Dfectrl1Spec> {
        TswitchspacingW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Interval between samples in the REFERENCE period"]
    #[inline(always)]
    pub fn tsamplespacingref(&mut self) -> TsamplespacingrefW<'_, Dfectrl1Spec> {
        TsamplespacingrefW::new(self, 12)
    }
    #[doc = "Bit 15 - Whether to sample I/Q or magnitude/phase"]
    #[inline(always)]
    pub fn sampletype(&mut self) -> SampletypeW<'_, Dfectrl1Spec> {
        SampletypeW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Interval between samples in the SWITCHING period when CTEINLINECTRLEN is 0"]
    #[inline(always)]
    pub fn tsamplespacing(&mut self) -> TsamplespacingW<'_, Dfectrl1Spec> {
        TsamplespacingW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Gain will be lowered by the specified number of gain steps at the start of CTE"]
    #[inline(always)]
    pub fn agcbackoffgain(&mut self) -> AgcbackoffgainW<'_, Dfectrl1Spec> {
        AgcbackoffgainW::new(self, 24)
    }
}
#[doc = "Various configuration for Direction finding\n\nYou can [`read`](crate::Reg::read) this register and get [`dfectrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfectrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfectrl1Spec;
impl crate::RegisterSpec for Dfectrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfectrl1::R`](R) reader structure"]
impl crate::Readable for Dfectrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dfectrl1::W`](W) writer structure"]
impl crate::Writable for Dfectrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFECTRL1 to value 0x0002_3282"]
impl crate::Resettable for Dfectrl1Spec {
    const RESET_VALUE: u32 = 0x0002_3282;
}
