#[doc = "Register `CTITRIGOUTSTATUS` reader"]
pub type R = crate::R<CtitrigoutstatusSpec>;
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugreq {
    #[doc = "1: Ctitrigout 0 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 0 is inactive."]
    Inactive = 0,
}
impl From<Debugreq> for bool {
    #[inline(always)]
    fn from(variant: Debugreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGREQ` reader - Processor debug request"]
pub type DebugreqR = crate::BitReader<Debugreq>;
impl DebugreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debugreq {
        match self.bits {
            true => Debugreq::Active,
            false => Debugreq::Inactive,
        }
    }
    #[doc = "Ctitrigout 0 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Debugreq::Active
    }
    #[doc = "Ctitrigout 0 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Debugreq::Inactive
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpurestart {
    #[doc = "1: Ctitrigout 1 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 1 is inactive."]
    Inactive = 0,
}
impl From<Cpurestart> for bool {
    #[inline(always)]
    fn from(variant: Cpurestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURESTART` reader - Processor Restart"]
pub type CpurestartR = crate::BitReader<Cpurestart>;
impl CpurestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpurestart {
        match self.bits {
            true => Cpurestart::Active,
            false => Cpurestart::Inactive,
        }
    }
    #[doc = "Ctitrigout 1 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Cpurestart::Active
    }
    #[doc = "Ctitrigout 1 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Cpurestart::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused0 {
    #[doc = "1: Ctitrigout 2 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 2 is inactive."]
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
    #[doc = "Ctitrigout 2 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused0::Active
    }
    #[doc = "Ctitrigout 2 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused0::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused1 {
    #[doc = "1: Ctitrigout 3 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 3 is inactive."]
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
    #[doc = "Ctitrigout 3 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused1::Active
    }
    #[doc = "Ctitrigout 3 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused1::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused2 {
    #[doc = "1: Ctitrigout 4 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 4 is inactive."]
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
    #[doc = "Ctitrigout 4 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused2::Active
    }
    #[doc = "Ctitrigout 4 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused2::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused3 {
    #[doc = "1: Ctitrigout 5 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 5 is inactive."]
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
    #[doc = "Ctitrigout 5 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused3::Active
    }
    #[doc = "Ctitrigout 5 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused3::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused4 {
    #[doc = "1: Ctitrigout 6 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 6 is inactive."]
    Inactive = 0,
}
impl From<Unused4> for bool {
    #[inline(always)]
    fn from(variant: Unused4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED4` reader - N/A"]
pub type Unused4R = crate::BitReader<Unused4>;
impl Unused4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unused4 {
        match self.bits {
            true => Unused4::Active,
            false => Unused4::Inactive,
        }
    }
    #[doc = "Ctitrigout 6 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused4::Active
    }
    #[doc = "Ctitrigout 6 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused4::Inactive
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unused5 {
    #[doc = "1: Ctitrigout 7 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 7 is inactive."]
    Inactive = 0,
}
impl From<Unused5> for bool {
    #[inline(always)]
    fn from(variant: Unused5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNUSED5` reader - N/A"]
pub type Unused5R = crate::BitReader<Unused5>;
impl Unused5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Unused5 {
        match self.bits {
            true => Unused5::Active,
            false => Unused5::Inactive,
        }
    }
    #[doc = "Ctitrigout 7 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Unused5::Active
    }
    #[doc = "Ctitrigout 7 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Unused5::Inactive
    }
}
impl R {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&self) -> DebugreqR {
        DebugreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&self) -> CpurestartR {
        CpurestartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> Unused0R {
        Unused0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> Unused1R {
        Unused1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn unused2(&self) -> Unused2R {
        Unused2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn unused3(&self) -> Unused3R {
        Unused3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused4(&self) -> Unused4R {
        Unused4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused5(&self) -> Unused5R {
        Unused5R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "CTI Trigger Out Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctitrigoutstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtitrigoutstatusSpec;
impl crate::RegisterSpec for CtitrigoutstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctitrigoutstatus::R`](R) reader structure"]
impl crate::Readable for CtitrigoutstatusSpec {}
#[doc = "`reset()` method sets CTITRIGOUTSTATUS to value 0"]
impl crate::Resettable for CtitrigoutstatusSpec {}
