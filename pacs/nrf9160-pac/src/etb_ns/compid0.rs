#[doc = "Register `COMPID0` reader"]
pub type R = crate::R<Compid0Spec>;
#[doc = "Field `PRMBL_0` reader - Contains bits \\[7:0\\] of the component identification"]
pub type Prmbl0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains bits \\[7:0\\] of the component identification"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> Prmbl0R {
        Prmbl0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compid0Spec;
impl crate::RegisterSpec for Compid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compid0::R`](R) reader structure"]
impl crate::Readable for Compid0Spec {}
#[doc = "`reset()` method sets COMPID0 to value 0x0d"]
impl crate::Resettable for Compid0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
