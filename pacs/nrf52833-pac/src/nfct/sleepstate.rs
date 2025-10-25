#[doc = "Register `SLEEPSTATE` reader"]
pub type R = crate::R<SleepstateSpec>;
#[doc = "Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepstate {
    #[doc = "0: State is IDLE."]
    Idle = 0,
    #[doc = "1: State is SLEEP_A."]
    SleepA = 1,
}
impl From<Sleepstate> for bool {
    #[inline(always)]
    fn from(variant: Sleepstate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPSTATE` reader - Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
pub type SleepstateR = crate::BitReader<Sleepstate>;
impl SleepstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepstate {
        match self.bits {
            false => Sleepstate::Idle,
            true => Sleepstate::SleepA,
        }
    }
    #[doc = "State is IDLE."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Sleepstate::Idle
    }
    #[doc = "State is SLEEP_A."]
    #[inline(always)]
    pub fn is_sleep_a(&self) -> bool {
        *self == Sleepstate::SleepA
    }
}
impl R {
    #[doc = "Bit 0 - Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
    #[inline(always)]
    pub fn sleepstate(&self) -> SleepstateR {
        SleepstateR::new((self.bits & 1) != 0)
    }
}
#[doc = "Sleep state during automatic collision resolution\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepstateSpec;
impl crate::RegisterSpec for SleepstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepstate::R`](R) reader structure"]
impl crate::Readable for SleepstateSpec {}
#[doc = "`reset()` method sets SLEEPSTATE to value 0"]
impl crate::Resettable for SleepstateSpec {}
