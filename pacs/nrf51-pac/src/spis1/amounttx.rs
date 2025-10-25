#[doc = "Register `AMOUNTTX` reader"]
pub type R = crate::R<AmounttxSpec>;
#[doc = "Field `AMOUNTTX` reader - Number of bytes transmitted in last granted transaction."]
pub type AmounttxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of bytes transmitted in last granted transaction."]
    #[inline(always)]
    pub fn amounttx(&self) -> AmounttxR {
        AmounttxR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes transmitted in last granted transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`amounttx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmounttxSpec;
impl crate::RegisterSpec for AmounttxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amounttx::R`](R) reader structure"]
impl crate::Readable for AmounttxSpec {}
#[doc = "`reset()` method sets AMOUNTTX to value 0"]
impl crate::Resettable for AmounttxSpec {}
