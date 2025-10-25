#[doc = "Register `DOUT_BUFFER` reader"]
pub type R = crate::R<DoutBufferSpec>;
#[doc = "Field `DATA` reader - This address can be used by the CPU to read data directly from the DOUT buffer."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This address can be used by the CPU to read data directly from the DOUT buffer."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Cryptographic results directly accessible by the CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_buffer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutBufferSpec;
impl crate::RegisterSpec for DoutBufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_buffer::R`](R) reader structure"]
impl crate::Readable for DoutBufferSpec {}
#[doc = "`reset()` method sets DOUT_BUFFER to value 0"]
impl crate::Resettable for DoutBufferSpec {}
