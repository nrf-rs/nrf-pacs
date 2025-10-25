#[doc = "Register `TRCPDSR` reader"]
pub type R = crate::R<TrcpdsrSpec>;
#[doc = "Register `TRCPDSR` writer"]
pub type W = crate::W<TrcpdsrSpec>;
#[doc = "Indicates ETM is powered up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Power {
    #[doc = "0: ETM is not powered up. All registers are not accessible."]
    NotPoweredUp = 0,
    #[doc = "1: ETM is powered up. All registers are accessible."]
    PoweredUp = 1,
}
impl From<Power> for bool {
    #[inline(always)]
    fn from(variant: Power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER` reader - Indicates ETM is powered up"]
pub type PowerR = crate::BitReader<Power>;
impl PowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Power {
        match self.bits {
            false => Power::NotPoweredUp,
            true => Power::PoweredUp,
        }
    }
    #[doc = "ETM is not powered up. All registers are not accessible."]
    #[inline(always)]
    pub fn is_not_powered_up(&self) -> bool {
        *self == Power::NotPoweredUp
    }
    #[doc = "ETM is powered up. All registers are accessible."]
    #[inline(always)]
    pub fn is_powered_up(&self) -> bool {
        *self == Power::PoweredUp
    }
}
#[doc = "Field `POWER` writer - Indicates ETM is powered up"]
pub type PowerW<'a, REG> = crate::BitWriter<'a, REG, Power>;
impl<'a, REG> PowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETM is not powered up. All registers are not accessible."]
    #[inline(always)]
    pub fn not_powered_up(self) -> &'a mut crate::W<REG> {
        self.variant(Power::NotPoweredUp)
    }
    #[doc = "ETM is powered up. All registers are accessible."]
    #[inline(always)]
    pub fn powered_up(self) -> &'a mut crate::W<REG> {
        self.variant(Power::PoweredUp)
    }
}
#[doc = "Sticky power down state. This bit is set to 1 when power to the ETM registers is removed, to indicate that programming state has been lost. It is cleared after a read of the TRCPDSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stickypd {
    #[doc = "0: Trace register power has not been removed since the TRCPDSR was last read."]
    NotPoweredDown = 0,
    #[doc = "1: Trace register power has been removed since the TRCPDSR was last read."]
    PoweredDown = 1,
}
impl From<Stickypd> for bool {
    #[inline(always)]
    fn from(variant: Stickypd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STICKYPD` reader - Sticky power down state. This bit is set to 1 when power to the ETM registers is removed, to indicate that programming state has been lost. It is cleared after a read of the TRCPDSR"]
pub type StickypdR = crate::BitReader<Stickypd>;
impl StickypdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stickypd {
        match self.bits {
            false => Stickypd::NotPoweredDown,
            true => Stickypd::PoweredDown,
        }
    }
    #[doc = "Trace register power has not been removed since the TRCPDSR was last read."]
    #[inline(always)]
    pub fn is_not_powered_down(&self) -> bool {
        *self == Stickypd::NotPoweredDown
    }
    #[doc = "Trace register power has been removed since the TRCPDSR was last read."]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == Stickypd::PoweredDown
    }
}
#[doc = "Field `STICKYPD` writer - Sticky power down state. This bit is set to 1 when power to the ETM registers is removed, to indicate that programming state has been lost. It is cleared after a read of the TRCPDSR"]
pub type StickypdW<'a, REG> = crate::BitWriter<'a, REG, Stickypd>;
impl<'a, REG> StickypdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trace register power has not been removed since the TRCPDSR was last read."]
    #[inline(always)]
    pub fn not_powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(Stickypd::NotPoweredDown)
    }
    #[doc = "Trace register power has been removed since the TRCPDSR was last read."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut crate::W<REG> {
        self.variant(Stickypd::PoweredDown)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates ETM is powered up"]
    #[inline(always)]
    pub fn power(&self) -> PowerR {
        PowerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sticky power down state. This bit is set to 1 when power to the ETM registers is removed, to indicate that programming state has been lost. It is cleared after a read of the TRCPDSR"]
    #[inline(always)]
    pub fn stickypd(&self) -> StickypdR {
        StickypdR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates ETM is powered up"]
    #[inline(always)]
    pub fn power(&mut self) -> PowerW<'_, TrcpdsrSpec> {
        PowerW::new(self, 0)
    }
    #[doc = "Bit 1 - Sticky power down state. This bit is set to 1 when power to the ETM registers is removed, to indicate that programming state has been lost. It is cleared after a read of the TRCPDSR"]
    #[inline(always)]
    pub fn stickypd(&mut self) -> StickypdW<'_, TrcpdsrSpec> {
        StickypdW::new(self, 1)
    }
}
#[doc = "Indicates the power down status of the ETM.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcpdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcpdsrSpec;
impl crate::RegisterSpec for TrcpdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpdsr::R`](R) reader structure"]
impl crate::Readable for TrcpdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcpdsr::W`](W) writer structure"]
impl crate::Writable for TrcpdsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCPDSR to value 0"]
impl crate::Resettable for TrcpdsrSpec {}
