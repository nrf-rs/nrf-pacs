#[doc = "Register `ER[%s]` reader"]
pub type R = crate::R<ErSpec>;
#[doc = "Field `ER` reader - Encryption Root, word n"]
pub type ErR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Encryption Root, word n"]
    #[inline(always)]
    pub fn er(&self) -> ErR {
        ErR::new(self.bits)
    }
}
#[doc = "Description collection\\[0\\]: Encryption Root, word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErSpec;
impl crate::RegisterSpec for ErSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`er::R`](R) reader structure"]
impl crate::Readable for ErSpec {}
#[doc = "`reset()` method sets ER[%s] to value 0xffff_ffff"]
impl crate::Resettable for ErSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
