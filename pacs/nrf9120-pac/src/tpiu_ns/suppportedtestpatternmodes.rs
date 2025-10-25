#[doc = "Register `SUPPPORTEDTESTPATTERNMODES` reader"]
pub type R = crate::R<SuppportedtestpatternmodesSpec>;
#[doc = "Register `SUPPPORTEDTESTPATTERNMODES` writer"]
pub type W = crate::W<SuppportedtestpatternmodesSpec>;
#[doc = "Indicates whether the walking 1s pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patw1 {
    #[doc = "0: Test pattern is not supported."]
    NotSupported = 0,
    #[doc = "1: Test pattern is supported."]
    Supported = 1,
}
impl From<Patw1> for bool {
    #[inline(always)]
    fn from(variant: Patw1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATW1` reader - Indicates whether the walking 1s pattern is supported as output over the trace port."]
pub type Patw1R = crate::BitReader<Patw1>;
impl Patw1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Patw1 {
        match self.bits {
            false => Patw1::NotSupported,
            true => Patw1::Supported,
        }
    }
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Patw1::NotSupported
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Patw1::Supported
    }
}
#[doc = "Field `PATW1` writer - Indicates whether the walking 1s pattern is supported as output over the trace port."]
pub type Patw1W<'a, REG> = crate::BitWriter<'a, REG, Patw1>;
impl<'a, REG> Patw1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Patw1::NotSupported)
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Patw1::Supported)
    }
}
#[doc = "Indicates whether the walking 0s pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patw0 {
    #[doc = "0: Test pattern is not supported."]
    NotSupported = 0,
    #[doc = "1: Test pattern is supported."]
    Supported = 1,
}
impl From<Patw0> for bool {
    #[inline(always)]
    fn from(variant: Patw0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATW0` reader - Indicates whether the walking 0s pattern is supported as output over the trace port."]
pub type Patw0R = crate::BitReader<Patw0>;
impl Patw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Patw0 {
        match self.bits {
            false => Patw0::NotSupported,
            true => Patw0::Supported,
        }
    }
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Patw0::NotSupported
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Patw0::Supported
    }
}
#[doc = "Field `PATW0` writer - Indicates whether the walking 0s pattern is supported as output over the trace port."]
pub type Patw0W<'a, REG> = crate::BitWriter<'a, REG, Patw0>;
impl<'a, REG> Patw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Patw0::NotSupported)
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Patw0::Supported)
    }
}
#[doc = "Indicates whether the AA/55 pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pata5 {
    #[doc = "0: Test pattern is not supported."]
    NotSupported = 0,
    #[doc = "1: Test pattern is supported."]
    Supported = 1,
}
impl From<Pata5> for bool {
    #[inline(always)]
    fn from(variant: Pata5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATA5` reader - Indicates whether the AA/55 pattern is supported as output over the trace port."]
pub type Pata5R = crate::BitReader<Pata5>;
impl Pata5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pata5 {
        match self.bits {
            false => Pata5::NotSupported,
            true => Pata5::Supported,
        }
    }
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Pata5::NotSupported
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Pata5::Supported
    }
}
#[doc = "Field `PATA5` writer - Indicates whether the AA/55 pattern is supported as output over the trace port."]
pub type Pata5W<'a, REG> = crate::BitWriter<'a, REG, Pata5>;
impl<'a, REG> Pata5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Pata5::NotSupported)
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Pata5::Supported)
    }
}
#[doc = "Indicates whether the FF/00 pattern is supported as output over the trace port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Patf0 {
    #[doc = "0: Test pattern is not supported."]
    NotSupported = 0,
    #[doc = "1: Test pattern is supported."]
    Supported = 1,
}
impl From<Patf0> for bool {
    #[inline(always)]
    fn from(variant: Patf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PATF0` reader - Indicates whether the FF/00 pattern is supported as output over the trace port."]
pub type Patf0R = crate::BitReader<Patf0>;
impl Patf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Patf0 {
        match self.bits {
            false => Patf0::NotSupported,
            true => Patf0::Supported,
        }
    }
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Patf0::NotSupported
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Patf0::Supported
    }
}
#[doc = "Field `PATF0` writer - Indicates whether the FF/00 pattern is supported as output over the trace port."]
pub type Patf0W<'a, REG> = crate::BitWriter<'a, REG, Patf0>;
impl<'a, REG> Patf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test pattern is not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Patf0::NotSupported)
    }
    #[doc = "Test pattern is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Patf0::Supported)
    }
}
#[doc = "Indicates whether timed mode is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptimeen {
    #[doc = "0: Mode is not supported."]
    NotSupported = 0,
    #[doc = "1: Mode is supported."]
    Supported = 1,
}
impl From<Ptimeen> for bool {
    #[inline(always)]
    fn from(variant: Ptimeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTIMEEN` reader - Indicates whether timed mode is supported."]
pub type PtimeenR = crate::BitReader<Ptimeen>;
impl PtimeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptimeen {
        match self.bits {
            false => Ptimeen::NotSupported,
            true => Ptimeen::Supported,
        }
    }
    #[doc = "Mode is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Ptimeen::NotSupported
    }
    #[doc = "Mode is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Ptimeen::Supported
    }
}
#[doc = "Field `PTIMEEN` writer - Indicates whether timed mode is supported."]
pub type PtimeenW<'a, REG> = crate::BitWriter<'a, REG, Ptimeen>;
impl<'a, REG> PtimeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Ptimeen::NotSupported)
    }
    #[doc = "Mode is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Ptimeen::Supported)
    }
}
#[doc = "Indicates whether continuous mode is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pconten {
    #[doc = "0: Mode is not supported."]
    NotSupported = 0,
    #[doc = "1: Mode is supported."]
    Supported = 1,
}
impl From<Pconten> for bool {
    #[inline(always)]
    fn from(variant: Pconten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCONTEN` reader - Indicates whether continuous mode is supported."]
pub type PcontenR = crate::BitReader<Pconten>;
impl PcontenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pconten {
        match self.bits {
            false => Pconten::NotSupported,
            true => Pconten::Supported,
        }
    }
    #[doc = "Mode is not supported."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Pconten::NotSupported
    }
    #[doc = "Mode is supported."]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == Pconten::Supported
    }
}
#[doc = "Field `PCONTEN` writer - Indicates whether continuous mode is supported."]
pub type PcontenW<'a, REG> = crate::BitWriter<'a, REG, Pconten>;
impl<'a, REG> PcontenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is not supported."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Pconten::NotSupported)
    }
    #[doc = "Mode is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut crate::W<REG> {
        self.variant(Pconten::Supported)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether the walking 1s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw1(&self) -> Patw1R {
        Patw1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the walking 0s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw0(&self) -> Patw0R {
        Patw0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates whether the AA/55 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn pata5(&self) -> Pata5R {
        Pata5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates whether the FF/00 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patf0(&self) -> Patf0R {
        Patf0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates whether timed mode is supported."]
    #[inline(always)]
    pub fn ptimeen(&self) -> PtimeenR {
        PtimeenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates whether continuous mode is supported."]
    #[inline(always)]
    pub fn pconten(&self) -> PcontenR {
        PcontenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether the walking 1s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw1(&mut self) -> Patw1W<'_, SuppportedtestpatternmodesSpec> {
        Patw1W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates whether the walking 0s pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patw0(&mut self) -> Patw0W<'_, SuppportedtestpatternmodesSpec> {
        Patw0W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates whether the AA/55 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn pata5(&mut self) -> Pata5W<'_, SuppportedtestpatternmodesSpec> {
        Pata5W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates whether the FF/00 pattern is supported as output over the trace port."]
    #[inline(always)]
    pub fn patf0(&mut self) -> Patf0W<'_, SuppportedtestpatternmodesSpec> {
        Patf0W::new(self, 3)
    }
    #[doc = "Bit 16 - Indicates whether timed mode is supported."]
    #[inline(always)]
    pub fn ptimeen(&mut self) -> PtimeenW<'_, SuppportedtestpatternmodesSpec> {
        PtimeenW::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates whether continuous mode is supported."]
    #[inline(always)]
    pub fn pconten(&mut self) -> PcontenW<'_, SuppportedtestpatternmodesSpec> {
        PcontenW::new(self, 17)
    }
}
#[doc = "The Supported_test_pattern_modes register provides a set of known bit sequences or patterns that can be output over the trace port and can be detected by the TPA or other associated trace capture device.\n\nYou can [`read`](crate::Reg::read) this register and get [`suppportedtestpatternmodes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suppportedtestpatternmodes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SuppportedtestpatternmodesSpec;
impl crate::RegisterSpec for SuppportedtestpatternmodesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`suppportedtestpatternmodes::R`](R) reader structure"]
impl crate::Readable for SuppportedtestpatternmodesSpec {}
#[doc = "`write(|w| ..)` method takes [`suppportedtestpatternmodes::W`](W) writer structure"]
impl crate::Writable for SuppportedtestpatternmodesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUPPPORTEDTESTPATTERNMODES to value 0"]
impl crate::Resettable for SuppportedtestpatternmodesSpec {}
