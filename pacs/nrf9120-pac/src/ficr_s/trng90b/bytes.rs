#[doc = "Register `BYTES` reader"]
pub type R = crate::R<BytesSpec>;
#[doc = "Field `BYTES` reader - Amount of bytes for the required entropy bits"]
pub type BytesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Amount of bytes for the required entropy bits"]
    #[inline(always)]
    pub fn bytes(&self) -> BytesR {
        BytesR::new(self.bits)
    }
}
#[doc = "Amount of bytes for the required entropy bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytes::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BytesSpec;
impl crate::RegisterSpec for BytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bytes::R`](R) reader structure"]
impl crate::Readable for BytesSpec {}
#[doc = "`reset()` method sets BYTES to value 0xffff_ffff"]
impl crate::Resettable for BytesSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
