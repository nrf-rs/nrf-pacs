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
#[doc = "ETM Event Input 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin0 {
    #[doc = "1: Ctitrigout 4 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 4 is inactive."]
    Inactive = 0,
}
impl From<Etmevtin0> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN0` reader - ETM Event Input 0"]
pub type Etmevtin0R = crate::BitReader<Etmevtin0>;
impl Etmevtin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etmevtin0 {
        match self.bits {
            true => Etmevtin0::Active,
            false => Etmevtin0::Inactive,
        }
    }
    #[doc = "Ctitrigout 4 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Etmevtin0::Active
    }
    #[doc = "Ctitrigout 4 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Etmevtin0::Inactive
    }
}
#[doc = "ETM Event Input 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin1 {
    #[doc = "1: Ctitrigout 5 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 5 is inactive."]
    Inactive = 0,
}
impl From<Etmevtin1> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN1` reader - ETM Event Input 1"]
pub type Etmevtin1R = crate::BitReader<Etmevtin1>;
impl Etmevtin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etmevtin1 {
        match self.bits {
            true => Etmevtin1::Active,
            false => Etmevtin1::Inactive,
        }
    }
    #[doc = "Ctitrigout 5 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Etmevtin1::Active
    }
    #[doc = "Ctitrigout 5 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Etmevtin1::Inactive
    }
}
#[doc = "ETM Event Input 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin2 {
    #[doc = "1: Ctitrigout 6 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 6 is inactive."]
    Inactive = 0,
}
impl From<Etmevtin2> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN2` reader - ETM Event Input 2"]
pub type Etmevtin2R = crate::BitReader<Etmevtin2>;
impl Etmevtin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etmevtin2 {
        match self.bits {
            true => Etmevtin2::Active,
            false => Etmevtin2::Inactive,
        }
    }
    #[doc = "Ctitrigout 6 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Etmevtin2::Active
    }
    #[doc = "Ctitrigout 6 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Etmevtin2::Inactive
    }
}
#[doc = "ETM Event Input 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etmevtin3 {
    #[doc = "1: Ctitrigout 7 is active."]
    Active = 1,
    #[doc = "0: Ctitrigout 7 is inactive."]
    Inactive = 0,
}
impl From<Etmevtin3> for bool {
    #[inline(always)]
    fn from(variant: Etmevtin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMEVTIN3` reader - ETM Event Input 3"]
pub type Etmevtin3R = crate::BitReader<Etmevtin3>;
impl Etmevtin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etmevtin3 {
        match self.bits {
            true => Etmevtin3::Active,
            false => Etmevtin3::Inactive,
        }
    }
    #[doc = "Ctitrigout 7 is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Etmevtin3::Active
    }
    #[doc = "Ctitrigout 7 is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Etmevtin3::Inactive
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
    #[doc = "Bit 4 - ETM Event Input 0"]
    #[inline(always)]
    pub fn etmevtin0(&self) -> Etmevtin0R {
        Etmevtin0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ETM Event Input 1"]
    #[inline(always)]
    pub fn etmevtin1(&self) -> Etmevtin1R {
        Etmevtin1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ETM Event Input 2"]
    #[inline(always)]
    pub fn etmevtin2(&self) -> Etmevtin2R {
        Etmevtin2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ETM Event Input 3"]
    #[inline(always)]
    pub fn etmevtin3(&self) -> Etmevtin3R {
        Etmevtin3R::new(((self.bits >> 7) & 1) != 0)
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
