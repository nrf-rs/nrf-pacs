#[doc = "Register `PERIPHID1` reader"]
pub type R = crate::R<Periphid1Spec>;
#[doc = "Field `PART_1` reader - Bits \\[11:8\\] of the component's part number. This is selected by the designer of the component."]
pub type Part1R = crate::FieldReader;
#[doc = "Field `DES_0` reader - Bits 3:0 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Bits \\[11:8\\] of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Bits 3:0 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid1Spec;
impl crate::RegisterSpec for Periphid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid1::R`](R) reader structure"]
impl crate::Readable for Periphid1Spec {}
#[doc = "`reset()` method sets PERIPHID1 to value 0xb9"]
impl crate::Resettable for Periphid1Spec {
    const RESET_VALUE: u32 = 0xb9;
}
