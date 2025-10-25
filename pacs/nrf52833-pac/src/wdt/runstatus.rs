#[doc = "Register `RUNSTATUS` reader"]
pub type R = crate::R<RunstatusSpec>;
#[doc = "Indicates whether or not the watchdog is running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runstatus {
    #[doc = "0: Watchdog not running"]
    NotRunning = 0,
    #[doc = "1: Watchdog is running"]
    Running = 1,
}
impl From<Runstatus> for bool {
    #[inline(always)]
    fn from(variant: Runstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTATUS` reader - Indicates whether or not the watchdog is running"]
pub type RunstatusR = crate::BitReader<Runstatus>;
impl RunstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Runstatus {
        match self.bits {
            false => Runstatus::NotRunning,
            true => Runstatus::Running,
        }
    }
    #[doc = "Watchdog not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == Runstatus::NotRunning
    }
    #[doc = "Watchdog is running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Runstatus::Running
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub fn runstatus(&self) -> RunstatusR {
        RunstatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Run status\n\nYou can [`read`](crate::Reg::read) this register and get [`runstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RunstatusSpec;
impl crate::RegisterSpec for RunstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`runstatus::R`](R) reader structure"]
impl crate::Readable for RunstatusSpec {}
#[doc = "`reset()` method sets RUNSTATUS to value 0"]
impl crate::Resettable for RunstatusSpec {}
