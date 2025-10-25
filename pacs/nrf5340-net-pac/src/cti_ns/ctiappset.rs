#[doc = "Register `CTIAPPSET` reader"]
pub type R = crate::R<CtiappsetSpec>;
#[doc = "Register `CTIAPPSET` writer"]
pub type W = crate::W<CtiappsetSpec>;
#[doc = "Application trigger event for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset0 {
    #[doc = "0: Application trigger 0 is inactive."]
    Inactive = 0,
    #[doc = "1: Application trigger 0 is active."]
    Active = 1,
}
impl From<Appset0> for bool {
    #[inline(always)]
    fn from(variant: Appset0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_0` reader - Application trigger event for channel 0."]
pub type Appset0R = crate::BitReader<Appset0>;
impl Appset0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Appset0 {
        match self.bits {
            false => Appset0::Inactive,
            true => Appset0::Active,
        }
    }
    #[doc = "Application trigger 0 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Appset0::Inactive
    }
    #[doc = "Application trigger 0 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Appset0::Active
    }
}
#[doc = "Application trigger event for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset0WO {
    #[doc = "1: Generate channel event for channel 0."]
    Activate = 1,
}
impl From<Appset0WO> for bool {
    #[inline(always)]
    fn from(variant: Appset0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_0` writer - Application trigger event for channel 0."]
pub type Appset0W<'a, REG> = crate::BitWriter<'a, REG, Appset0WO>;
impl<'a, REG> Appset0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate channel event for channel 0."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(Appset0WO::Activate)
    }
}
#[doc = "Application trigger event for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset1 {
    #[doc = "0: Application trigger 1 is inactive."]
    Inactive = 0,
    #[doc = "1: Application trigger 1 is active."]
    Active = 1,
}
impl From<Appset1> for bool {
    #[inline(always)]
    fn from(variant: Appset1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_1` reader - Application trigger event for channel 1."]
pub type Appset1R = crate::BitReader<Appset1>;
impl Appset1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Appset1 {
        match self.bits {
            false => Appset1::Inactive,
            true => Appset1::Active,
        }
    }
    #[doc = "Application trigger 1 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Appset1::Inactive
    }
    #[doc = "Application trigger 1 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Appset1::Active
    }
}
#[doc = "Application trigger event for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset1WO {
    #[doc = "1: Generate channel event for channel 1."]
    Activate = 1,
}
impl From<Appset1WO> for bool {
    #[inline(always)]
    fn from(variant: Appset1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_1` writer - Application trigger event for channel 1."]
pub type Appset1W<'a, REG> = crate::BitWriter<'a, REG, Appset1WO>;
impl<'a, REG> Appset1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate channel event for channel 1."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(Appset1WO::Activate)
    }
}
#[doc = "Application trigger event for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset2 {
    #[doc = "0: Application trigger 2 is inactive."]
    Inactive = 0,
    #[doc = "1: Application trigger 2 is active."]
    Active = 1,
}
impl From<Appset2> for bool {
    #[inline(always)]
    fn from(variant: Appset2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_2` reader - Application trigger event for channel 2."]
pub type Appset2R = crate::BitReader<Appset2>;
impl Appset2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Appset2 {
        match self.bits {
            false => Appset2::Inactive,
            true => Appset2::Active,
        }
    }
    #[doc = "Application trigger 2 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Appset2::Inactive
    }
    #[doc = "Application trigger 2 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Appset2::Active
    }
}
#[doc = "Application trigger event for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset2WO {
    #[doc = "1: Generate channel event for channel 2."]
    Activate = 1,
}
impl From<Appset2WO> for bool {
    #[inline(always)]
    fn from(variant: Appset2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_2` writer - Application trigger event for channel 2."]
pub type Appset2W<'a, REG> = crate::BitWriter<'a, REG, Appset2WO>;
impl<'a, REG> Appset2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate channel event for channel 2."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(Appset2WO::Activate)
    }
}
#[doc = "Application trigger event for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset3 {
    #[doc = "0: Application trigger 3 is inactive."]
    Inactive = 0,
    #[doc = "1: Application trigger 3 is active."]
    Active = 1,
}
impl From<Appset3> for bool {
    #[inline(always)]
    fn from(variant: Appset3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_3` reader - Application trigger event for channel 3."]
pub type Appset3R = crate::BitReader<Appset3>;
impl Appset3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Appset3 {
        match self.bits {
            false => Appset3::Inactive,
            true => Appset3::Active,
        }
    }
    #[doc = "Application trigger 3 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Appset3::Inactive
    }
    #[doc = "Application trigger 3 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Appset3::Active
    }
}
#[doc = "Application trigger event for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appset3WO {
    #[doc = "1: Generate channel event for channel 3."]
    Activate = 1,
}
impl From<Appset3WO> for bool {
    #[inline(always)]
    fn from(variant: Appset3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPSET_3` writer - Application trigger event for channel 3."]
pub type Appset3W<'a, REG> = crate::BitWriter<'a, REG, Appset3WO>;
impl<'a, REG> Appset3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate channel event for channel 3."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(Appset3WO::Activate)
    }
}
impl R {
    #[doc = "Bit 0 - Application trigger event for channel 0."]
    #[inline(always)]
    pub fn appset_0(&self) -> Appset0R {
        Appset0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Application trigger event for channel 1."]
    #[inline(always)]
    pub fn appset_1(&self) -> Appset1R {
        Appset1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Application trigger event for channel 2."]
    #[inline(always)]
    pub fn appset_2(&self) -> Appset2R {
        Appset2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Application trigger event for channel 3."]
    #[inline(always)]
    pub fn appset_3(&self) -> Appset3R {
        Appset3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application trigger event for channel 0."]
    #[inline(always)]
    pub fn appset_0(&mut self) -> Appset0W<'_, CtiappsetSpec> {
        Appset0W::new(self, 0)
    }
    #[doc = "Bit 1 - Application trigger event for channel 1."]
    #[inline(always)]
    pub fn appset_1(&mut self) -> Appset1W<'_, CtiappsetSpec> {
        Appset1W::new(self, 1)
    }
    #[doc = "Bit 2 - Application trigger event for channel 2."]
    #[inline(always)]
    pub fn appset_2(&mut self) -> Appset2W<'_, CtiappsetSpec> {
        Appset2W::new(self, 2)
    }
    #[doc = "Bit 3 - Application trigger event for channel 3."]
    #[inline(always)]
    pub fn appset_3(&mut self) -> Appset3W<'_, CtiappsetSpec> {
        Appset3W::new(self, 3)
    }
}
#[doc = "CTI Application Trigger Set register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiappset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiappsetSpec;
impl crate::RegisterSpec for CtiappsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiappset::R`](R) reader structure"]
impl crate::Readable for CtiappsetSpec {}
#[doc = "`write(|w| ..)` method takes [`ctiappset::W`](W) writer structure"]
impl crate::Writable for CtiappsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIAPPSET to value 0"]
impl crate::Resettable for CtiappsetSpec {}
