#[doc = "Register `CTICHINSTATUS` reader"]
pub type R = crate::R<CtichinstatusSpec>;
#[doc = "Shows the status of the ctitrigin 0 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctichinstatus0 {
    #[doc = "1: Ctichin 0 is active."]
    Active = 1,
    #[doc = "0: Ctichin 0 is inactive."]
    Inactive = 0,
}
impl From<Ctichinstatus0> for bool {
    #[inline(always)]
    fn from(variant: Ctichinstatus0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_0` reader - Shows the status of the ctitrigin 0 input."]
pub type Ctichinstatus0R = crate::BitReader<Ctichinstatus0>;
impl Ctichinstatus0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctichinstatus0 {
        match self.bits {
            true => Ctichinstatus0::Active,
            false => Ctichinstatus0::Inactive,
        }
    }
    #[doc = "Ctichin 0 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ctichinstatus0::Active
    }
    #[doc = "Ctichin 0 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ctichinstatus0::Inactive
    }
}
#[doc = "Shows the status of the ctitrigin 1 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctichinstatus1 {
    #[doc = "1: Ctichin 1 is active."]
    Active = 1,
    #[doc = "0: Ctichin 1 is inactive."]
    Inactive = 0,
}
impl From<Ctichinstatus1> for bool {
    #[inline(always)]
    fn from(variant: Ctichinstatus1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_1` reader - Shows the status of the ctitrigin 1 input."]
pub type Ctichinstatus1R = crate::BitReader<Ctichinstatus1>;
impl Ctichinstatus1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctichinstatus1 {
        match self.bits {
            true => Ctichinstatus1::Active,
            false => Ctichinstatus1::Inactive,
        }
    }
    #[doc = "Ctichin 1 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ctichinstatus1::Active
    }
    #[doc = "Ctichin 1 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ctichinstatus1::Inactive
    }
}
#[doc = "Shows the status of the ctitrigin 2 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctichinstatus2 {
    #[doc = "1: Ctichin 2 is active."]
    Active = 1,
    #[doc = "0: Ctichin 2 is inactive."]
    Inactive = 0,
}
impl From<Ctichinstatus2> for bool {
    #[inline(always)]
    fn from(variant: Ctichinstatus2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_2` reader - Shows the status of the ctitrigin 2 input."]
pub type Ctichinstatus2R = crate::BitReader<Ctichinstatus2>;
impl Ctichinstatus2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctichinstatus2 {
        match self.bits {
            true => Ctichinstatus2::Active,
            false => Ctichinstatus2::Inactive,
        }
    }
    #[doc = "Ctichin 2 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ctichinstatus2::Active
    }
    #[doc = "Ctichin 2 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ctichinstatus2::Inactive
    }
}
#[doc = "Shows the status of the ctitrigin 3 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctichinstatus3 {
    #[doc = "1: Ctichin 3 is active."]
    Active = 1,
    #[doc = "0: Ctichin 3 is inactive."]
    Inactive = 0,
}
impl From<Ctichinstatus3> for bool {
    #[inline(always)]
    fn from(variant: Ctichinstatus3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTICHINSTATUS_3` reader - Shows the status of the ctitrigin 3 input."]
pub type Ctichinstatus3R = crate::BitReader<Ctichinstatus3>;
impl Ctichinstatus3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctichinstatus3 {
        match self.bits {
            true => Ctichinstatus3::Active,
            false => Ctichinstatus3::Inactive,
        }
    }
    #[doc = "Ctichin 3 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ctichinstatus3::Active
    }
    #[doc = "Ctichin 3 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ctichinstatus3::Inactive
    }
}
impl R {
    #[doc = "Bit 0 - Shows the status of the ctitrigin 0 input."]
    #[inline(always)]
    pub fn ctichinstatus_0(&self) -> Ctichinstatus0R {
        Ctichinstatus0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shows the status of the ctitrigin 1 input."]
    #[inline(always)]
    pub fn ctichinstatus_1(&self) -> Ctichinstatus1R {
        Ctichinstatus1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shows the status of the ctitrigin 2 input."]
    #[inline(always)]
    pub fn ctichinstatus_2(&self) -> Ctichinstatus2R {
        Ctichinstatus2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shows the status of the ctitrigin 3 input."]
    #[inline(always)]
    pub fn ctichinstatus_3(&self) -> Ctichinstatus3R {
        Ctichinstatus3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "CTI Channel In Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctichinstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtichinstatusSpec;
impl crate::RegisterSpec for CtichinstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctichinstatus::R`](R) reader structure"]
impl crate::Readable for CtichinstatusSpec {}
#[doc = "`reset()` method sets CTICHINSTATUS to value 0"]
impl crate::Resettable for CtichinstatusSpec {}
