#[doc = "Register `PERIPHID2` reader"]
pub type R = crate::R<Periphid2Spec>;
#[doc = "Field `DES_1` reader - Bits 6:4 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
pub type Des1R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - Always set. Indicates that a JEDEC assigned value is used"]
pub type JedecR = crate::BitReader;
#[doc = "Field `REVISION` reader - The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bits 6:4 of the JEDEC identity code indicating the designer of the component (along with the continuation code)"]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Always set. Indicates that a JEDEC assigned value is used"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - The Revision field is an incremental value starting at 0x0 for the first design of this component. This only increases by 1 for both major and minor revisions and is simply used as a look-up to establish the exact major/minor revision."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid2Spec;
impl crate::RegisterSpec for Periphid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid2::R`](R) reader structure"]
impl crate::Readable for Periphid2Spec {}
#[doc = "`reset()` method sets PERIPHID2 to value 0x4b"]
impl crate::Resettable for Periphid2Spec {
    const RESET_VALUE: u32 = 0x4b;
}
