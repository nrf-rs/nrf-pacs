#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Power {
    #[doc = "0: Peripheral is powered off"]
    Disabled = 0,
    #[doc = "1: Peripheral is powered on"]
    Enabled = 1,
}
impl From<Power> for bool {
    #[inline(always)]
    fn from(variant: Power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER` reader - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
pub type PowerR = crate::BitReader<Power>;
impl PowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Power {
        match self.bits {
            false => Power::Disabled,
            true => Power::Enabled,
        }
    }
    #[doc = "Peripheral is powered off"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Power::Disabled
    }
    #[doc = "Peripheral is powered on"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Power::Enabled
    }
}
#[doc = "Field `POWER` writer - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
pub type PowerW<'a, REG> = crate::BitWriter<'a, REG, Power>;
impl<'a, REG> PowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral is powered off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Power::Disabled)
    }
    #[doc = "Peripheral is powered on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Power::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn power(&self) -> PowerR {
        PowerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn power(&mut self) -> PowerW<'_, PowerSpec> {
        PowerW::new(self, 0)
    }
}
#[doc = "Peripheral power control\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER to value 0x01"]
impl crate::Resettable for PowerSpec {
    const RESET_VALUE: u32 = 0x01;
}
