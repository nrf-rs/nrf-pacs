#[doc = "Register `COMPID3` reader"]
pub type R = crate::R<Compid3Spec>;
#[doc = "Field `PRMBL_3` reader - Contains bits \\[31:24\\] of the component identification"]
pub type Prmbl3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains bits \\[31:24\\] of the component identification"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compid3Spec;
impl crate::RegisterSpec for Compid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compid3::R`](R) reader structure"]
impl crate::Readable for Compid3Spec {}
#[doc = "`reset()` method sets COMPID3 to value 0xb1"]
impl crate::Resettable for Compid3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
