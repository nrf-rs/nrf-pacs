#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Peripheral power control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Power {
    #[doc = "0: Module power disabled."]
    Disabled = 0,
    #[doc = "1: Module power enabled."]
    Enabled = 1,
}
impl From<Power> for bool {
    #[inline(always)]
    fn from(variant: Power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWER` reader - Peripheral power control."]
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
    #[doc = "Module power disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Power::Disabled
    }
    #[doc = "Module power enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Power::Enabled
    }
}
#[doc = "Field `POWER` writer - Peripheral power control."]
pub type PowerW<'a, REG> = crate::BitWriter<'a, REG, Power>;
impl<'a, REG> PowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Module power disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Power::Disabled)
    }
    #[doc = "Module power enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Power::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral power control."]
    #[inline(always)]
    pub fn power(&self) -> PowerR {
        PowerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral power control."]
    #[inline(always)]
    pub fn power(&mut self) -> PowerW<'_, PowerSpec> {
        PowerW::new(self, 0)
    }
}
#[doc = "Peripheral power control.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for PowerSpec {}
