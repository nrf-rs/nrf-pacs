#[doc = "Register `PKA_VERSION` reader"]
pub type R = crate::R<PkaVersionSpec>;
#[doc = "Field `PKA_VERSION` reader - "]
pub type PkaVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pka_version(&self) -> PkaVersionR {
        PkaVersionR::new(self.bits)
    }
}
#[doc = "PKA engine HW version. Reset value holds the version.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaVersionSpec;
impl crate::RegisterSpec for PkaVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_version::R`](R) reader structure"]
impl crate::Readable for PkaVersionSpec {}
#[doc = "`reset()` method sets PKA_VERSION to value 0x1611_0215"]
impl crate::Resettable for PkaVersionSpec {
    const RESET_VALUE: u32 = 0x1611_0215;
}
