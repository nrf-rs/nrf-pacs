#[doc = "Register `RUNSTATUS` reader"]
pub type R = crate::R<RunstatusSpec>;
#[doc = "Indicates whether or not WDT is running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runstatuswdt {
    #[doc = "0: Watchdog is not running"]
    NotRunning = 0,
    #[doc = "1: Watchdog is running"]
    Running = 1,
}
impl From<Runstatuswdt> for bool {
    #[inline(always)]
    fn from(variant: Runstatuswdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTATUSWDT` reader - Indicates whether or not WDT is running"]
pub type RunstatuswdtR = crate::BitReader<Runstatuswdt>;
impl RunstatuswdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Runstatuswdt {
        match self.bits {
            false => Runstatuswdt::NotRunning,
            true => Runstatuswdt::Running,
        }
    }
    #[doc = "Watchdog is not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == Runstatuswdt::NotRunning
    }
    #[doc = "Watchdog is running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Runstatuswdt::Running
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not WDT is running"]
    #[inline(always)]
    pub fn runstatuswdt(&self) -> RunstatuswdtR {
        RunstatuswdtR::new((self.bits & 1) != 0)
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
