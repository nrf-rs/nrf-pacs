#[doc = "Register `ACCDBLREAD` reader"]
pub type R = crate::R<AccdblreadSpec>;
#[doc = "Field `ACCDBLREAD` reader - Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
pub type AccdblreadR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
    #[inline(always)]
    pub fn accdblread(&self) -> AccdblreadR {
        AccdblreadR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task\n\nYou can [`read`](crate::Reg::read) this register and get [`accdblread::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AccdblreadSpec;
impl crate::RegisterSpec for AccdblreadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`accdblread::R`](R) reader structure"]
impl crate::Readable for AccdblreadSpec {}
#[doc = "`reset()` method sets ACCDBLREAD to value 0"]
impl crate::Resettable for AccdblreadSpec {}
