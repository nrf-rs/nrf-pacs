#[doc = "Register `COMPID2` reader"]
pub type R = crate::R<Compid2Spec>;
#[doc = "Field `PRMBL_2` reader - Contains bits \\[23:16\\] of the component identification"]
pub type Prmbl2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains bits \\[23:16\\] of the component identification"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> Prmbl2R {
        Prmbl2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compid2Spec;
impl crate::RegisterSpec for Compid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compid2::R`](R) reader structure"]
impl crate::Readable for Compid2Spec {}
#[doc = "`reset()` method sets COMPID2 to value 0x05"]
impl crate::Resettable for Compid2Spec {
    const RESET_VALUE: u32 = 0x05;
}
