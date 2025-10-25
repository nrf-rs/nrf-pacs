#[doc = "Register `COMPID1` reader"]
pub type R = crate::R<Compid1Spec>;
#[doc = "Field `PRMBL_1` reader - Contains bits \\[11:8\\] of the component identification"]
pub type Prmbl1R = crate::FieldReader;
#[doc = "Field `CLASS` reader - Class of the component. E. g. ROM table, CoreSight component etc. Constitutes bits \\[15:12\\] of the component identification."]
pub type ClassR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Contains bits \\[11:8\\] of the component identification"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Class of the component. E. g. ROM table, CoreSight component etc. Constitutes bits \\[15:12\\] of the component identification."]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compid1Spec;
impl crate::RegisterSpec for Compid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compid1::R`](R) reader structure"]
impl crate::Readable for Compid1Spec {}
#[doc = "`reset()` method sets COMPID1 to value 0x90"]
impl crate::Resettable for Compid1Spec {
    const RESET_VALUE: u32 = 0x90;
}
