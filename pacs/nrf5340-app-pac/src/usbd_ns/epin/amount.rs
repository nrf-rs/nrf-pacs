#[doc = "Register `AMOUNT` reader"]
pub type R = crate::R<AmountSpec>;
#[doc = "Field `AMOUNT` reader - Number of bytes transferred in the last transaction"]
pub type AmountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(&self) -> AmountR {
        AmountR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Description cluster: Number of bytes transferred in the last transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmountSpec;
impl crate::RegisterSpec for AmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amount::R`](R) reader structure"]
impl crate::Readable for AmountSpec {}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AmountSpec {}
