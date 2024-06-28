#[doc = "Register `APCUTOFF` reader"]
pub type R = crate::R<ApcutoffSpec>;
#[doc = "Field `APCUTOFF` reader - Adaptive proportion cutoff"]
pub type ApcutoffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Adaptive proportion cutoff"]
    #[inline(always)]
    pub fn apcutoff(&self) -> ApcutoffR {
        ApcutoffR::new(self.bits)
    }
}
#[doc = "Adaptive proportion cutoff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apcutoff::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApcutoffSpec;
impl crate::RegisterSpec for ApcutoffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apcutoff::R`](R) reader structure"]
impl crate::Readable for ApcutoffSpec {}
#[doc = "`reset()` method sets APCUTOFF to value 0xffff_ffff"]
impl crate::Resettable for ApcutoffSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
