#[doc = "Register `AMOUNTRX` reader"]
pub type R = crate::R<AmountrxSpec>;
#[doc = "Field `AMOUNTRX` reader - Number of bytes received in last granted transaction."]
pub type AmountrxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of bytes received in last granted transaction."]
    #[inline(always)]
    pub fn amountrx(&self) -> AmountrxR {
        AmountrxR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes received in last granted transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`amountrx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmountrxSpec;
impl crate::RegisterSpec for AmountrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amountrx::R`](R) reader structure"]
impl crate::Readable for AmountrxSpec {}
#[doc = "`reset()` method sets AMOUNTRX to value 0"]
impl crate::Resettable for AmountrxSpec {}
