#[doc = "Register `RXMATCH` reader"]
pub type R = crate::R<RxmatchSpec>;
#[doc = "Field `RXMATCH` reader - Logical address in which previous packet was received."]
pub type RxmatchR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Logical address in which previous packet was received."]
    #[inline(always)]
    pub fn rxmatch(&self) -> RxmatchR {
        RxmatchR::new((self.bits & 7) as u8)
    }
}
#[doc = "Received address.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmatch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmatchSpec;
impl crate::RegisterSpec for RxmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmatch::R`](R) reader structure"]
impl crate::Readable for RxmatchSpec {}
#[doc = "`reset()` method sets RXMATCH to value 0"]
impl crate::Resettable for RxmatchSpec {}
