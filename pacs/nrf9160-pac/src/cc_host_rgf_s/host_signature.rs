#[doc = "Register `HOST_SIGNATURE` reader"]
pub type R = crate::R<HostSignatureSpec>;
#[doc = "Field `VALUE` reader - Fixed-value identification signature used by host driver to verify CRYPTOCELL presence at this address."]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Fixed-value identification signature used by host driver to verify CRYPTOCELL presence at this address."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "This register holds the CRYPTOCELL subsystem signature. See reset value.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_signature::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostSignatureSpec;
impl crate::RegisterSpec for HostSignatureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_signature::R`](R) reader structure"]
impl crate::Readable for HostSignatureSpec {}
#[doc = "`reset()` method sets HOST_SIGNATURE to value 0x20e0_0000"]
impl crate::Resettable for HostSignatureSpec {
    const RESET_VALUE: u32 = 0x20e0_0000;
}
