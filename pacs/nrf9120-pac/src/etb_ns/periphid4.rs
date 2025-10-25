#[doc = "Register `PERIPHID4` reader"]
pub type R = crate::R<Periphid4Spec>;
#[doc = "Field `DES_2` reader - JEDEC continuation code indicating the designer of the component (along with the identity code)"]
pub type Des2R = crate::FieldReader;
#[doc = "Field `SIZE` reader - This is a 4-bit value that indicates the total contiguous size of the memory window used by this component in powers of 2 from the standard 4KB. If a component only requires the standard 4KB then this should read as 0x0, 4KB only, for 8KB set to 0x1, 16KB == 0x2, 32KB == 0x3, and so on."]
pub type SizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEDEC continuation code indicating the designer of the component (along with the identity code)"]
    #[inline(always)]
    pub fn des_2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This is a 4-bit value that indicates the total contiguous size of the memory window used by this component in powers of 2 from the standard 4KB. If a component only requires the standard 4KB then this should read as 0x0, 4KB only, for 8KB set to 0x1, 16KB == 0x2, 32KB == 0x3, and so on."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid4Spec;
impl crate::RegisterSpec for Periphid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid4::R`](R) reader structure"]
impl crate::Readable for Periphid4Spec {}
#[doc = "`reset()` method sets PERIPHID4 to value 0x04"]
impl crate::Resettable for Periphid4Spec {
    const RESET_VALUE: u32 = 0x04;
}
