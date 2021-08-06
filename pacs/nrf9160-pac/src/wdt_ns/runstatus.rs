#[doc = "Register `RUNSTATUS` reader"]
pub struct R(crate::R<RUNSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUNSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUNSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUNSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicates whether or not the watchdog is running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTATUSWDT_A {
    #[doc = "0: Watchdog not running"]
    NOTRUNNING = 0,
    #[doc = "1: Watchdog is running"]
    RUNNING = 1,
}
impl From<RUNSTATUSWDT_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTATUSWDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUNSTATUSWDT` reader - Indicates whether or not the watchdog is running"]
pub struct RUNSTATUSWDT_R(crate::FieldReader<bool, RUNSTATUSWDT_A>);
impl RUNSTATUSWDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTATUSWDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTATUSWDT_A {
        match self.bits {
            false => RUNSTATUSWDT_A::NOTRUNNING,
            true => RUNSTATUSWDT_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        **self == RUNSTATUSWDT_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == RUNSTATUSWDT_A::RUNNING
    }
}
impl core::ops::Deref for RUNSTATUSWDT_R {
    type Target = crate::FieldReader<bool, RUNSTATUSWDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub fn runstatuswdt(&self) -> RUNSTATUSWDT_R {
        RUNSTATUSWDT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Run status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [runstatus](index.html) module"]
pub struct RUNSTATUS_SPEC;
impl crate::RegisterSpec for RUNSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [runstatus::R](R) reader structure"]
impl crate::Readable for RUNSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RUNSTATUS to value 0"]
impl crate::Resettable for RUNSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
