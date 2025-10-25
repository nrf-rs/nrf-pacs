#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Constant carrier. Decision point: TXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Constcarrier {
    #[doc = "0: Constant carrier disabled."]
    Disabled = 0,
    #[doc = "1: Constant carrier enabled."]
    Enabled = 1,
}
impl From<Constcarrier> for bool {
    #[inline(always)]
    fn from(variant: Constcarrier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONSTCARRIER` reader - Constant carrier. Decision point: TXEN task."]
pub type ConstcarrierR = crate::BitReader<Constcarrier>;
impl ConstcarrierR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Constcarrier {
        match self.bits {
            false => Constcarrier::Disabled,
            true => Constcarrier::Enabled,
        }
    }
    #[doc = "Constant carrier disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Constcarrier::Disabled
    }
    #[doc = "Constant carrier enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Constcarrier::Enabled
    }
}
#[doc = "Field `CONSTCARRIER` writer - Constant carrier. Decision point: TXEN task."]
pub type ConstcarrierW<'a, REG> = crate::BitWriter<'a, REG, Constcarrier>;
impl<'a, REG> ConstcarrierW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Constant carrier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Constcarrier::Disabled)
    }
    #[doc = "Constant carrier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Constcarrier::Enabled)
    }
}
#[doc = "PLL lock. Decision point: TXEN or RXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plllock {
    #[doc = "0: PLL lock disabled."]
    Disabled = 0,
    #[doc = "1: PLL lock enabled."]
    Enabled = 1,
}
impl From<Plllock> for bool {
    #[inline(always)]
    fn from(variant: Plllock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLLOCK` reader - PLL lock. Decision point: TXEN or RXEN task."]
pub type PlllockR = crate::BitReader<Plllock>;
impl PlllockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plllock {
        match self.bits {
            false => Plllock::Disabled,
            true => Plllock::Enabled,
        }
    }
    #[doc = "PLL lock disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Plllock::Disabled
    }
    #[doc = "PLL lock enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Plllock::Enabled
    }
}
#[doc = "Field `PLLLOCK` writer - PLL lock. Decision point: TXEN or RXEN task."]
pub type PlllockW<'a, REG> = crate::BitWriter<'a, REG, Plllock>;
impl<'a, REG> PlllockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL lock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Plllock::Disabled)
    }
    #[doc = "PLL lock enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Plllock::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline(always)]
    pub fn constcarrier(&self) -> ConstcarrierR {
        ConstcarrierR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn plllock(&self) -> PlllockR {
        PlllockR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline(always)]
    pub fn constcarrier(&mut self) -> ConstcarrierW<'_, TestSpec> {
        ConstcarrierW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn plllock(&mut self) -> PlllockW<'_, TestSpec> {
        PlllockW::new(self, 1)
    }
}
#[doc = "Test features enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
