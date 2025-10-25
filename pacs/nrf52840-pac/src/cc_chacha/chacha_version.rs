#[doc = "Register `CHACHA_VERSION` reader"]
pub type R = crate::R<ChachaVersionSpec>;
#[doc = "Field `CHACHA_VERSION` reader - "]
pub type ChachaVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chacha_version(&self) -> ChachaVersionR {
        ChachaVersionR::new(self.bits)
    }
}
#[doc = "CHACHA engine HW version\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaVersionSpec;
impl crate::RegisterSpec for ChachaVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_version::R`](R) reader structure"]
impl crate::Readable for ChachaVersionSpec {}
#[doc = "`reset()` method sets CHACHA_VERSION to value 0x01"]
impl crate::Resettable for ChachaVersionSpec {
    const RESET_VALUE: u32 = 0x01;
}
