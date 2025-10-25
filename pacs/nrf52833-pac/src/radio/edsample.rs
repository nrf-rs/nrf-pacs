#[doc = "Register `EDSAMPLE` reader"]
pub type R = crate::R<EdsampleSpec>;
#[doc = "Field `EDLVL` reader - IEEE 802.15.4 energy detect level"]
pub type EdlvlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EdlvlR {
        EdlvlR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IEEE 802.15.4 energy detect level\n\nYou can [`read`](crate::Reg::read) this register and get [`edsample::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdsampleSpec;
impl crate::RegisterSpec for EdsampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edsample::R`](R) reader structure"]
impl crate::Readable for EdsampleSpec {}
#[doc = "`reset()` method sets EDSAMPLE to value 0"]
impl crate::Resettable for EdsampleSpec {}
