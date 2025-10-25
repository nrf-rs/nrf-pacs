#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<Pidr2Spec>;
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Des1 {
    #[doc = "3: Arm. Bits\\[6:4\\] of the JEDEC JEP106 Identity Code"]
    Arm = 3,
}
impl From<Des1> for u8 {
    #[inline(always)]
    fn from(variant: Des1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Des1 {
    type Ux = u8;
}
impl crate::IsEnum for Des1 {}
#[doc = "Field `DES_1` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type Des1R = crate::FieldReader<Des1>;
impl Des1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Des1> {
        match self.bits {
            3 => Some(Des1::Arm),
            _ => None,
        }
    }
    #[doc = "Arm. Bits\\[6:4\\] of the JEDEC JEP106 Identity Code"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == Des1::Arm
    }
}
#[doc = "Field `JEDEC` reader - Always 1. Indicates that the JEDEC-assigned designer ID is used."]
pub type JedecR = crate::BitReader;
#[doc = "Peripheral revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Revision {
    #[doc = "0: This device is at r0p0"]
    Rev0p0 = 0,
}
impl From<Revision> for u8 {
    #[inline(always)]
    fn from(variant: Revision) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Revision {
    type Ux = u8;
}
impl crate::IsEnum for Revision {}
#[doc = "Field `REVISION` reader - Peripheral revision"]
pub type RevisionR = crate::FieldReader<Revision>;
impl RevisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Revision> {
        match self.bits {
            0 => Some(Revision::Rev0p0),
            _ => None,
        }
    }
    #[doc = "This device is at r0p0"]
    #[inline(always)]
    pub fn is_rev0p0(&self) -> bool {
        *self == Revision::Rev0p0
    }
}
impl R {
    #[doc = "Bits 0:2 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Always 1. Indicates that the JEDEC-assigned designer ID is used."]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr2Spec;
impl crate::RegisterSpec for Pidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for Pidr2Spec {}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for Pidr2Spec {
    const RESET_VALUE: u32 = 0x0b;
}
