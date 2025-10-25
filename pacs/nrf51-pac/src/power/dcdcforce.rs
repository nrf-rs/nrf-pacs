#[doc = "Register `DCDCFORCE` reader"]
pub type R = crate::R<DcdcforceSpec>;
#[doc = "Register `DCDCFORCE` writer"]
pub type W = crate::W<DcdcforceSpec>;
#[doc = "DCDC power-up force off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceoff {
    #[doc = "0: No force."]
    NoForce = 0,
    #[doc = "1: Force."]
    Force = 1,
}
impl From<Forceoff> for bool {
    #[inline(always)]
    fn from(variant: Forceoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEOFF` reader - DCDC power-up force off."]
pub type ForceoffR = crate::BitReader<Forceoff>;
impl ForceoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceoff {
        match self.bits {
            false => Forceoff::NoForce,
            true => Forceoff::Force,
        }
    }
    #[doc = "No force."]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == Forceoff::NoForce
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == Forceoff::Force
    }
}
#[doc = "Field `FORCEOFF` writer - DCDC power-up force off."]
pub type ForceoffW<'a, REG> = crate::BitWriter<'a, REG, Forceoff>;
impl<'a, REG> ForceoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut crate::W<REG> {
        self.variant(Forceoff::NoForce)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(Forceoff::Force)
    }
}
#[doc = "DCDC power-up force on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceon {
    #[doc = "0: No force."]
    NoForce = 0,
    #[doc = "1: Force."]
    Force = 1,
}
impl From<Forceon> for bool {
    #[inline(always)]
    fn from(variant: Forceon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEON` reader - DCDC power-up force on."]
pub type ForceonR = crate::BitReader<Forceon>;
impl ForceonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceon {
        match self.bits {
            false => Forceon::NoForce,
            true => Forceon::Force,
        }
    }
    #[doc = "No force."]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == Forceon::NoForce
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == Forceon::Force
    }
}
#[doc = "Field `FORCEON` writer - DCDC power-up force on."]
pub type ForceonW<'a, REG> = crate::BitWriter<'a, REG, Forceon>;
impl<'a, REG> ForceonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut crate::W<REG> {
        self.variant(Forceon::NoForce)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(Forceon::Force)
    }
}
impl R {
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&self) -> ForceoffR {
        ForceoffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&self) -> ForceonR {
        ForceonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&mut self) -> ForceoffW<'_, DcdcforceSpec> {
        ForceoffW::new(self, 0)
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&mut self) -> ForceonW<'_, DcdcforceSpec> {
        ForceonW::new(self, 1)
    }
}
#[doc = "DCDC power-up force register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcforce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcforce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcforceSpec;
impl crate::RegisterSpec for DcdcforceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcforce::R`](R) reader structure"]
impl crate::Readable for DcdcforceSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdcforce::W`](W) writer structure"]
impl crate::Writable for DcdcforceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCFORCE to value 0"]
impl crate::Resettable for DcdcforceSpec {}
