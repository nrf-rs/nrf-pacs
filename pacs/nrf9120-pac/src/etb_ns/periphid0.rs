#[doc = "Register `PERIPHID0` reader"]
pub type R = crate::R<Periphid0Spec>;
#[doc = "Field `PART_0` reader - Bits \\[7:0\\] of the component's part number. This is selected by the designer of the component."]
pub type Part0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits \\[7:0\\] of the component's part number. This is selected by the designer of the component."]
    #[inline(always)]
    pub fn part_0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid0Spec;
impl crate::RegisterSpec for Periphid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid0::R`](R) reader structure"]
impl crate::Readable for Periphid0Spec {}
#[doc = "`reset()` method sets PERIPHID0 to value 0x07"]
impl crate::Resettable for Periphid0Spec {
    const RESET_VALUE: u32 = 0x07;
}
