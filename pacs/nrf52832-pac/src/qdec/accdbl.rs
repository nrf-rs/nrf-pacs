#[doc = "Register `ACCDBL` reader"]
pub type R = crate::R<AccdblSpec>;
#[doc = "Field `ACCDBL` reader - Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
pub type AccdblR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
    #[inline(always)]
    pub fn accdbl(&self) -> AccdblR {
        AccdblR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Register accumulating the number of detected double transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`accdbl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccdblSpec;
impl crate::RegisterSpec for AccdblSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accdbl::R`](R) reader structure"]
impl crate::Readable for AccdblSpec {}
#[doc = "`reset()` method sets ACCDBL to value 0"]
impl crate::Resettable for AccdblSpec {}
