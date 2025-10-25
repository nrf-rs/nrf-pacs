#[doc = "Register `CTITRIGINSTATUS` reader"]
pub type R = crate::R<CtitriginstatusSpec>;
#[doc = "Processor Halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpuhalted {
    #[doc = "1: Ctitrigin 0 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 0 is inactive."]
    Inactive = 0,
}
impl From<Cpuhalted> for bool {
    #[inline(always)]
    fn from(variant: Cpuhalted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUHALTED` reader - Processor Halted"]
pub type CpuhaltedR = crate::BitReader<Cpuhalted>;
impl CpuhaltedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpuhalted {
        match self.bits {
            true => Cpuhalted::Active,
            false => Cpuhalted::Inactive,
        }
    }
    #[doc = "Ctitrigin 0 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Cpuhalted::Active
    }
    #[doc = "Ctitrigin 0 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Cpuhalted::Inactive
    }
}
#[doc = "DWT Comparator Output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dwtcompout0 {
    #[doc = "1: Ctitrigin 1 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 1 is inactive."]
    Inactive = 0,
}
impl From<Dwtcompout0> for bool {
    #[inline(always)]
    fn from(variant: Dwtcompout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWTCOMPOUT0` reader - DWT Comparator Output 0"]
pub type Dwtcompout0R = crate::BitReader<Dwtcompout0>;
impl Dwtcompout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dwtcompout0 {
        match self.bits {
            true => Dwtcompout0::Active,
            false => Dwtcompout0::Inactive,
        }
    }
    #[doc = "Ctitrigin 1 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dwtcompout0::Active
    }
    #[doc = "Ctitrigin 1 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dwtcompout0::Inactive
    }
}
#[doc = "DWT Comparator Output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dwtcompout1 {
    #[doc = "1: Ctitrigin 2 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 2 is inactive."]
    Inactive = 0,
}
impl From<Dwtcompout1> for bool {
    #[inline(always)]
    fn from(variant: Dwtcompout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWTCOMPOUT1` reader - DWT Comparator Output 1"]
pub type Dwtcompout1R = crate::BitReader<Dwtcompout1>;
impl Dwtcompout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dwtcompout1 {
        match self.bits {
            true => Dwtcompout1::Active,
            false => Dwtcompout1::Inactive,
        }
    }
    #[doc = "Ctitrigin 2 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dwtcompout1::Active
    }
    #[doc = "Ctitrigin 2 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dwtcompout1::Inactive
    }
}
#[doc = "DWT Comparator Output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dwtcompout2 {
    #[doc = "1: Ctitrigin 3 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 3 is inactive."]
    Inactive = 0,
}
impl From<Dwtcompout2> for bool {
    #[inline(always)]
    fn from(variant: Dwtcompout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWTCOMPOUT2` reader - DWT Comparator Output 2"]
pub type Dwtcompout2R = crate::BitReader<Dwtcompout2>;
impl Dwtcompout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dwtcompout2 {
        match self.bits {
            true => Dwtcompout2::Active,
            false => Dwtcompout2::Inactive,
        }
    }
    #[doc = "Ctitrigin 3 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dwtcompout2::Active
    }
    #[doc = "Ctitrigin 3 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dwtcompout2::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused0 {
    #[doc = "1: Ctitrigin 4 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 4 is inactive."]
    Inactive = 0,
}
impl From<Unused0> for bool {
    #[inline(always)]
    fn from(variant: Unused0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED0` reader - N/A"]
pub type Unused0R = crate::BitReader<Unused0>;
impl Unused0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unused0 {
        match self.bits {
            true => Unused0::Active,
            false => Unused0::Inactive,
        }
    }
    #[doc = "Ctitrigin 4 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused0::Active
    }
    #[doc = "Ctitrigin 4 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused0::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused1 {
    #[doc = "1: Ctitrigin 5 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 5 is inactive."]
    Inactive = 0,
}
impl From<Unused1> for bool {
    #[inline(always)]
    fn from(variant: Unused1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED1` reader - N/A"]
pub type Unused1R = crate::BitReader<Unused1>;
impl Unused1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unused1 {
        match self.bits {
            true => Unused1::Active,
            false => Unused1::Inactive,
        }
    }
    #[doc = "Ctitrigin 5 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused1::Active
    }
    #[doc = "Ctitrigin 5 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused1::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused2 {
    #[doc = "1: Ctitrigin 6 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 6 is inactive."]
    Inactive = 0,
}
impl From<Unused2> for bool {
    #[inline(always)]
    fn from(variant: Unused2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED2` reader - N/A"]
pub type Unused2R = crate::BitReader<Unused2>;
impl Unused2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unused2 {
        match self.bits {
            true => Unused2::Active,
            false => Unused2::Inactive,
        }
    }
    #[doc = "Ctitrigin 6 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused2::Active
    }
    #[doc = "Ctitrigin 6 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused2::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused3 {
    #[doc = "1: Ctitrigin 7 is active."]
    Active = 1,
    #[doc = "0: Ctitrigin 7 is inactive."]
    Inactive = 0,
}
impl From<Unused3> for bool {
    #[inline(always)]
    fn from(variant: Unused3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED3` reader - N/A"]
pub type Unused3R = crate::BitReader<Unused3>;
impl Unused3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unused3 {
        match self.bits {
            true => Unused3::Active,
            false => Unused3::Inactive,
        }
    }
    #[doc = "Ctitrigin 7 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused3::Active
    }
    #[doc = "Ctitrigin 7 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused3::Inactive
    }
}
impl R {
    #[doc = "Bit 0 - Processor Halted"]
    #[inline(always)]
    pub fn cpuhalted(&self) -> CpuhaltedR {
        CpuhaltedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DWT Comparator Output 0"]
    #[inline(always)]
    pub fn dwtcompout0(&self) -> Dwtcompout0R {
        Dwtcompout0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DWT Comparator Output 1"]
    #[inline(always)]
    pub fn dwtcompout1(&self) -> Dwtcompout1R {
        Dwtcompout1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DWT Comparator Output 2"]
    #[inline(always)]
    pub fn dwtcompout2(&self) -> Dwtcompout2R {
        Dwtcompout2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> Unused0R {
        Unused0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> Unused1R {
        Unused1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused2(&self) -> Unused2R {
        Unused2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused3(&self) -> Unused3R {
        Unused3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "CTI Trigger In Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctitriginstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtitriginstatusSpec;
impl crate::RegisterSpec for CtitriginstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctitriginstatus::R`](R) reader structure"]
impl crate::Readable for CtitriginstatusSpec {}
#[doc = "`reset()` method sets CTITRIGINSTATUS to value 0"]
impl crate::Resettable for CtitriginstatusSpec {}
