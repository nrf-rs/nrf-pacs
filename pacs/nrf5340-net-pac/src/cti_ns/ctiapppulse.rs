#[doc = "Register `CTIAPPPULSE` writer"]
pub type W = crate::W<CtiapppulseSpec>;
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appulse0 {
    #[doc = "1: Generates an event pulse on channel 0."]
    Generate = 1,
}
impl From<Appulse0> for bool {
    #[inline(always)]
    fn from(variant: Appulse0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_0` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub type Appulse0W<'a, REG> = crate::BitWriter<'a, REG, Appulse0>;
impl<'a, REG> Appulse0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an event pulse on channel 0."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Appulse0::Generate)
    }
}
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appulse1 {
    #[doc = "1: Generates an event pulse on channel 1."]
    Generate = 1,
}
impl From<Appulse1> for bool {
    #[inline(always)]
    fn from(variant: Appulse1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_1` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub type Appulse1W<'a, REG> = crate::BitWriter<'a, REG, Appulse1>;
impl<'a, REG> Appulse1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an event pulse on channel 1."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Appulse1::Generate)
    }
}
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appulse2 {
    #[doc = "1: Generates an event pulse on channel 2."]
    Generate = 1,
}
impl From<Appulse2> for bool {
    #[inline(always)]
    fn from(variant: Appulse2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_2` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub type Appulse2W<'a, REG> = crate::BitWriter<'a, REG, Appulse2>;
impl<'a, REG> Appulse2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an event pulse on channel 2."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Appulse2::Generate)
    }
}
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appulse3 {
    #[doc = "1: Generates an event pulse on channel 3."]
    Generate = 1,
}
impl From<Appulse3> for bool {
    #[inline(always)]
    fn from(variant: Appulse3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_3` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub type Appulse3W<'a, REG> = crate::BitWriter<'a, REG, Appulse3>;
impl<'a, REG> Appulse3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an event pulse on channel 3."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut crate::W<REG> {
        self.variant(Appulse3::Generate)
    }
}
impl W {
    #[doc = "Bit 0 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_0(&mut self) -> Appulse0W<'_, CtiapppulseSpec> {
        Appulse0W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_1(&mut self) -> Appulse1W<'_, CtiapppulseSpec> {
        Appulse1W::new(self, 1)
    }
    #[doc = "Bit 2 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_2(&mut self) -> Appulse2W<'_, CtiapppulseSpec> {
        Appulse2W::new(self, 2)
    }
    #[doc = "Bit 3 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_3(&mut self) -> Appulse3W<'_, CtiapppulseSpec> {
        Appulse3W::new(self, 3)
    }
}
#[doc = "CTI Application Pulse register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiapppulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiapppulseSpec;
impl crate::RegisterSpec for CtiapppulseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctiapppulse::W`](W) writer structure"]
impl crate::Writable for CtiapppulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIAPPPULSE to value 0"]
impl crate::Resettable for CtiapppulseSpec {}
