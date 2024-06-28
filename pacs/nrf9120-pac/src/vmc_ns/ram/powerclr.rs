#[doc = "Register `POWERCLR` writer"]
pub type W = crate::W<PowerclrSpec>;
#[doc = "Keep RAM section S0 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0power {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S0power> for bool {
    #[inline(always)]
    fn from(variant: S0power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 of RAM n on or off in System ON mode"]
pub type S0powerW<'a, REG> = crate::BitWriter<'a, REG, S0power>;
impl<'a, REG> S0powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S0power::Off)
    }
}
#[doc = "Keep RAM section S1 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1power {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S1power> for bool {
    #[inline(always)]
    fn from(variant: S1power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 of RAM n on or off in System ON mode"]
pub type S1powerW<'a, REG> = crate::BitWriter<'a, REG, S1power>;
impl<'a, REG> S1powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S1power::Off)
    }
}
#[doc = "Keep RAM section S2 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2power {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S2power> for bool {
    #[inline(always)]
    fn from(variant: S2power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 of RAM n on or off in System ON mode"]
pub type S2powerW<'a, REG> = crate::BitWriter<'a, REG, S2power>;
impl<'a, REG> S2powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S2power::Off)
    }
}
#[doc = "Keep RAM section S3 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3power {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S3power> for bool {
    #[inline(always)]
    fn from(variant: S3power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 of RAM n on or off in System ON mode"]
pub type S3powerW<'a, REG> = crate::BitWriter<'a, REG, S3power>;
impl<'a, REG> S3powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S3power::Off)
    }
}
#[doc = "Keep retention on RAM section S0 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0retention {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S0retention> for bool {
    #[inline(always)]
    fn from(variant: S0retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
pub type S0retentionW<'a, REG> = crate::BitWriter<'a, REG, S0retention>;
impl<'a, REG> S0retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S0retention::Off)
    }
}
#[doc = "Keep retention on RAM section S1 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1retention {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S1retention> for bool {
    #[inline(always)]
    fn from(variant: S1retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
pub type S1retentionW<'a, REG> = crate::BitWriter<'a, REG, S1retention>;
impl<'a, REG> S1retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S1retention::Off)
    }
}
#[doc = "Keep retention on RAM section S2 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2retention {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S2retention> for bool {
    #[inline(always)]
    fn from(variant: S2retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
pub type S2retentionW<'a, REG> = crate::BitWriter<'a, REG, S2retention>;
impl<'a, REG> S2retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S2retention::Off)
    }
}
#[doc = "Keep retention on RAM section S3 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3retention {
    #[doc = "1: Off"]
    Off = 1,
}
impl From<S3retention> for bool {
    #[inline(always)]
    fn from(variant: S3retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
pub type S3retentionW<'a, REG> = crate::BitWriter<'a, REG, S3retention>;
impl<'a, REG> S3retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S3retention::Off)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s0power(&mut self) -> S0powerW<PowerclrSpec> {
        S0powerW::new(self, 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s1power(&mut self) -> S1powerW<PowerclrSpec> {
        S1powerW::new(self, 1)
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s2power(&mut self) -> S2powerW<PowerclrSpec> {
        S2powerW::new(self, 2)
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s3power(&mut self) -> S3powerW<PowerclrSpec> {
        S3powerW::new(self, 3)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s0retention(&mut self) -> S0retentionW<PowerclrSpec> {
        S0retentionW::new(self, 16)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s1retention(&mut self) -> S1retentionW<PowerclrSpec> {
        S1retentionW::new(self, 17)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s2retention(&mut self) -> S2retentionW<PowerclrSpec> {
        S2retentionW::new(self, 18)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s3retention(&mut self) -> S3retentionW<PowerclrSpec> {
        S3retentionW::new(self, 19)
    }
}
#[doc = "Description cluster: RAMn power control clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerclrSpec;
impl crate::RegisterSpec for PowerclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`powerclr::W`](W) writer structure"]
impl crate::Writable for PowerclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWERCLR to value 0xffff"]
impl crate::Resettable for PowerclrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
