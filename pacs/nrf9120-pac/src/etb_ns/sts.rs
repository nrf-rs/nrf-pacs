#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `FULL` reader - RAM Full. The flag indicates when the RAM write pointer has wrapped around."]
pub type FullR = crate::BitReader;
#[doc = "Field `TRIGGERED` reader - The Triggered bit is set when a trigger has been observed. This does not indicate that a trigger has been embedded in the trace data by the formatter, but is determined by the programming of the Formatter and Flush Control Register."]
pub type TriggeredR = crate::BitReader;
#[doc = "Field `ACQCOMP` reader - The acquisition complete flag indicates that capture has been completed when the formatter stops because of any of the methods defined in the Formatter and Flush Control Register, or TraceCaptEn = 0. This also results in FtStopped in the Formatter and Flush Status Register going HIGH."]
pub type AcqcompR = crate::BitReader;
#[doc = "Field `FTEMPTY` reader - Formatter pipeline empty. All data stored to RAM."]
pub type FtemptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RAM Full. The flag indicates when the RAM write pointer has wrapped around."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The Triggered bit is set when a trigger has been observed. This does not indicate that a trigger has been embedded in the trace data by the formatter, but is determined by the programming of the Formatter and Flush Control Register."]
    #[inline(always)]
    pub fn triggered(&self) -> TriggeredR {
        TriggeredR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The acquisition complete flag indicates that capture has been completed when the formatter stops because of any of the methods defined in the Formatter and Flush Control Register, or TraceCaptEn = 0. This also results in FtStopped in the Formatter and Flush Status Register going HIGH."]
    #[inline(always)]
    pub fn acqcomp(&self) -> AcqcompR {
        AcqcompR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Formatter pipeline empty. All data stored to RAM."]
    #[inline(always)]
    pub fn ftempty(&self) -> FtemptyR {
        FtemptyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "ETB Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`reset()` method sets STS to value 0x08"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0x08;
}
