#[doc = "Register `REQSTATUS` reader"]
pub type R = crate::R<ReqstatusSpec>;
#[doc = "Request status for RR\\[0\\] register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr0 {
    #[doc = "0: RR\\[0\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[0\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr0> for bool {
    #[inline(always)]
    fn from(variant: Rr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR0` reader - Request status for RR\\[0\\] register"]
pub type Rr0R = crate::BitReader<Rr0>;
impl Rr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr0 {
        match self.bits {
            false => Rr0::DisabledOrRequested,
            true => Rr0::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[0\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr0::DisabledOrRequested
    }
    #[doc = "RR\\[0\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr0::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[1\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr1 {
    #[doc = "0: RR\\[1\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[1\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr1> for bool {
    #[inline(always)]
    fn from(variant: Rr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR1` reader - Request status for RR\\[1\\] register"]
pub type Rr1R = crate::BitReader<Rr1>;
impl Rr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr1 {
        match self.bits {
            false => Rr1::DisabledOrRequested,
            true => Rr1::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[1\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr1::DisabledOrRequested
    }
    #[doc = "RR\\[1\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr1::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[2\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr2 {
    #[doc = "0: RR\\[2\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[2\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr2> for bool {
    #[inline(always)]
    fn from(variant: Rr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR2` reader - Request status for RR\\[2\\] register"]
pub type Rr2R = crate::BitReader<Rr2>;
impl Rr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr2 {
        match self.bits {
            false => Rr2::DisabledOrRequested,
            true => Rr2::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[2\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr2::DisabledOrRequested
    }
    #[doc = "RR\\[2\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr2::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[3\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr3 {
    #[doc = "0: RR\\[3\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[3\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr3> for bool {
    #[inline(always)]
    fn from(variant: Rr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR3` reader - Request status for RR\\[3\\] register"]
pub type Rr3R = crate::BitReader<Rr3>;
impl Rr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr3 {
        match self.bits {
            false => Rr3::DisabledOrRequested,
            true => Rr3::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[3\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr3::DisabledOrRequested
    }
    #[doc = "RR\\[3\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr3::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[4\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr4 {
    #[doc = "0: RR\\[4\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[4\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr4> for bool {
    #[inline(always)]
    fn from(variant: Rr4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR4` reader - Request status for RR\\[4\\] register"]
pub type Rr4R = crate::BitReader<Rr4>;
impl Rr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr4 {
        match self.bits {
            false => Rr4::DisabledOrRequested,
            true => Rr4::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[4\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr4::DisabledOrRequested
    }
    #[doc = "RR\\[4\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr4::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[5\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr5 {
    #[doc = "0: RR\\[5\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[5\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr5> for bool {
    #[inline(always)]
    fn from(variant: Rr5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR5` reader - Request status for RR\\[5\\] register"]
pub type Rr5R = crate::BitReader<Rr5>;
impl Rr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr5 {
        match self.bits {
            false => Rr5::DisabledOrRequested,
            true => Rr5::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[5\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr5::DisabledOrRequested
    }
    #[doc = "RR\\[5\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr5::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[6\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr6 {
    #[doc = "0: RR\\[6\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[6\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr6> for bool {
    #[inline(always)]
    fn from(variant: Rr6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR6` reader - Request status for RR\\[6\\] register"]
pub type Rr6R = crate::BitReader<Rr6>;
impl Rr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr6 {
        match self.bits {
            false => Rr6::DisabledOrRequested,
            true => Rr6::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[6\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr6::DisabledOrRequested
    }
    #[doc = "RR\\[6\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr6::EnabledAndUnrequested
    }
}
#[doc = "Request status for RR\\[7\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr7 {
    #[doc = "0: RR\\[7\\] register is not enabled, or are already requesting reload"]
    DisabledOrRequested = 0,
    #[doc = "1: RR\\[7\\] register is enabled, and are not yet requesting reload"]
    EnabledAndUnrequested = 1,
}
impl From<Rr7> for bool {
    #[inline(always)]
    fn from(variant: Rr7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR7` reader - Request status for RR\\[7\\] register"]
pub type Rr7R = crate::BitReader<Rr7>;
impl Rr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr7 {
        match self.bits {
            false => Rr7::DisabledOrRequested,
            true => Rr7::EnabledAndUnrequested,
        }
    }
    #[doc = "RR\\[7\\] register is not enabled, or are already requesting reload"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == Rr7::DisabledOrRequested
    }
    #[doc = "RR\\[7\\] register is enabled, and are not yet requesting reload"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == Rr7::EnabledAndUnrequested
    }
}
impl R {
    #[doc = "Bit 0 - Request status for RR\\[0\\] register"]
    #[inline(always)]
    pub fn rr0(&self) -> Rr0R {
        Rr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request status for RR\\[1\\] register"]
    #[inline(always)]
    pub fn rr1(&self) -> Rr1R {
        Rr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Request status for RR\\[2\\] register"]
    #[inline(always)]
    pub fn rr2(&self) -> Rr2R {
        Rr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request status for RR\\[3\\] register"]
    #[inline(always)]
    pub fn rr3(&self) -> Rr3R {
        Rr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Request status for RR\\[4\\] register"]
    #[inline(always)]
    pub fn rr4(&self) -> Rr4R {
        Rr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Request status for RR\\[5\\] register"]
    #[inline(always)]
    pub fn rr5(&self) -> Rr5R {
        Rr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request status for RR\\[6\\] register"]
    #[inline(always)]
    pub fn rr6(&self) -> Rr6R {
        Rr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Request status for RR\\[7\\] register"]
    #[inline(always)]
    pub fn rr7(&self) -> Rr7R {
        Rr7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Request status\n\nYou can [`read`](crate::Reg::read) this register and get [`reqstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqstatusSpec;
impl crate::RegisterSpec for ReqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqstatus::R`](R) reader structure"]
impl crate::Readable for ReqstatusSpec {}
#[doc = "`reset()` method sets REQSTATUS to value 0x01"]
impl crate::Resettable for ReqstatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
