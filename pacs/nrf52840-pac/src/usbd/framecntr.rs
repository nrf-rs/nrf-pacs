#[doc = "Register `FRAMECNTR` reader"]
pub type R = crate::R<FramecntrSpec>;
#[doc = "Field `FRAMECNTR` reader - Returns the current value of the start of frame counter"]
pub type FramecntrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub fn framecntr(&self) -> FramecntrR {
        FramecntrR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Returns the current value of the start of frame counter\n\nYou can [`read`](crate::Reg::read) this register and get [`framecntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramecntrSpec;
impl crate::RegisterSpec for FramecntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framecntr::R`](R) reader structure"]
impl crate::Readable for FramecntrSpec {}
#[doc = "`reset()` method sets FRAMECNTR to value 0"]
impl crate::Resettable for FramecntrSpec {}
