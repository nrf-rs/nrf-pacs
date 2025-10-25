#[doc = "Register `RAMSTATUS` reader"]
pub type R = crate::R<RamstatusSpec>;
#[doc = "RAM block 0 status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramblock0 {
    #[doc = "0: RAM block 0 is off or powering up."]
    Off = 0,
    #[doc = "1: RAM block 0 is on."]
    On = 1,
}
impl From<Ramblock0> for bool {
    #[inline(always)]
    fn from(variant: Ramblock0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBLOCK0` reader - RAM block 0 status."]
pub type Ramblock0R = crate::BitReader<Ramblock0>;
impl Ramblock0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramblock0 {
        match self.bits {
            false => Ramblock0::Off,
            true => Ramblock0::On,
        }
    }
    #[doc = "RAM block 0 is off or powering up."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramblock0::Off
    }
    #[doc = "RAM block 0 is on."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ramblock0::On
    }
}
#[doc = "RAM block 1 status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramblock1 {
    #[doc = "0: RAM block 1 is off or powering up."]
    Off = 0,
    #[doc = "1: RAM block 1 is on."]
    On = 1,
}
impl From<Ramblock1> for bool {
    #[inline(always)]
    fn from(variant: Ramblock1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBLOCK1` reader - RAM block 1 status."]
pub type Ramblock1R = crate::BitReader<Ramblock1>;
impl Ramblock1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramblock1 {
        match self.bits {
            false => Ramblock1::Off,
            true => Ramblock1::On,
        }
    }
    #[doc = "RAM block 1 is off or powering up."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramblock1::Off
    }
    #[doc = "RAM block 1 is on."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ramblock1::On
    }
}
#[doc = "RAM block 2 status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramblock2 {
    #[doc = "0: RAM block 2 is off or powering up."]
    Off = 0,
    #[doc = "1: RAM block 2 is on."]
    On = 1,
}
impl From<Ramblock2> for bool {
    #[inline(always)]
    fn from(variant: Ramblock2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBLOCK2` reader - RAM block 2 status."]
pub type Ramblock2R = crate::BitReader<Ramblock2>;
impl Ramblock2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramblock2 {
        match self.bits {
            false => Ramblock2::Off,
            true => Ramblock2::On,
        }
    }
    #[doc = "RAM block 2 is off or powering up."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramblock2::Off
    }
    #[doc = "RAM block 2 is on."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ramblock2::On
    }
}
#[doc = "RAM block 3 status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramblock3 {
    #[doc = "0: RAM block 3 is off or powering up."]
    Off = 0,
    #[doc = "1: RAM block 3 is on."]
    On = 1,
}
impl From<Ramblock3> for bool {
    #[inline(always)]
    fn from(variant: Ramblock3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBLOCK3` reader - RAM block 3 status."]
pub type Ramblock3R = crate::BitReader<Ramblock3>;
impl Ramblock3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramblock3 {
        match self.bits {
            false => Ramblock3::Off,
            true => Ramblock3::On,
        }
    }
    #[doc = "RAM block 3 is off or powering up."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramblock3::Off
    }
    #[doc = "RAM block 3 is on."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ramblock3::On
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 0 status."]
    #[inline(always)]
    pub fn ramblock0(&self) -> Ramblock0R {
        Ramblock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM block 1 status."]
    #[inline(always)]
    pub fn ramblock1(&self) -> Ramblock1R {
        Ramblock1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM block 2 status."]
    #[inline(always)]
    pub fn ramblock2(&self) -> Ramblock2R {
        Ramblock2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM block 3 status."]
    #[inline(always)]
    pub fn ramblock3(&self) -> Ramblock3R {
        Ramblock3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Ram status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamstatusSpec;
impl crate::RegisterSpec for RamstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramstatus::R`](R) reader structure"]
impl crate::Readable for RamstatusSpec {}
#[doc = "`reset()` method sets RAMSTATUS to value 0"]
impl crate::Resettable for RamstatusSpec {}
