#[doc = "Register `FFSR` reader"]
pub type R = crate::R<FfsrSpec>;
#[doc = "Field `FLINPROG` reader - Flush In Progress. This is an indication of the current state of afvalids."]
pub type FlinprogR = crate::BitReader;
#[doc = "Field `FTSTOPPED` reader - Formatter stopped. The formatter has received a stop request signal and all trace data and post-amble has been output. Any more trace data on the ATB interface is ignored and atreadys goes HIGH."]
pub type FtstoppedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flush In Progress. This is an indication of the current state of afvalids."]
    #[inline(always)]
    pub fn flinprog(&self) -> FlinprogR {
        FlinprogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Formatter stopped. The formatter has received a stop request signal and all trace data and post-amble has been output. Any more trace data on the ATB interface is ignored and atreadys goes HIGH."]
    #[inline(always)]
    pub fn ftstopped(&self) -> FtstoppedR {
        FtstoppedR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ETB Formatter and Flush Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfsrSpec;
impl crate::RegisterSpec for FfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffsr::R`](R) reader structure"]
impl crate::Readable for FfsrSpec {}
#[doc = "`reset()` method sets FFSR to value 0x02"]
impl crate::Resettable for FfsrSpec {
    const RESET_VALUE: u32 = 0x02;
}
