#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<Pidr4Spec>;
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Des2 {
    #[doc = "4: JEDEC continuation code."]
    Code = 4,
}
impl From<Des2> for u8 {
    #[inline(always)]
    fn from(variant: Des2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Des2 {
    type Ux = u8;
}
impl crate::IsEnum for Des2 {}
#[doc = "Field `DES_2` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type Des2R = crate::FieldReader<Des2>;
impl Des2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Des2> {
        match self.bits {
            4 => Some(Des2::Code),
            _ => None,
        }
    }
    #[doc = "JEDEC continuation code."]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        *self == Des2::Code
    }
}
#[doc = "Field `SIZE` reader - Always 0b0000. Indicates that the device only occupies 4KB of memory."]
pub type SizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Always 0b0000. Indicates that the device only occupies 4KB of memory."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr4Spec;
impl crate::RegisterSpec for Pidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for Pidr4Spec {}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for Pidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
