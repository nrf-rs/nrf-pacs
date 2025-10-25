#[doc = "Register `CIDR3` reader"]
pub type R = crate::R<Cidr3Spec>;
#[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code.\n\nValue on reset: 177"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prmbl3 {
    #[doc = "177: Bits\\[31:24\\] of the identification code."]
    Value = 177,
}
impl From<Prmbl3> for u8 {
    #[inline(always)]
    fn from(variant: Prmbl3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prmbl3 {
    type Ux = u8;
}
impl crate::IsEnum for Prmbl3 {}
#[doc = "Field `PRMBL_3` reader - Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code."]
pub type Prmbl3R = crate::FieldReader<Prmbl3>;
impl Prmbl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prmbl3> {
        match self.bits {
            177 => Some(Prmbl3::Value),
            _ => None,
        }
    }
    #[doc = "Bits\\[31:24\\] of the identification code."]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == Prmbl3::Value
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code."]
    #[inline(always)]
    pub fn prmbl_3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr3Spec;
impl crate::RegisterSpec for Cidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr3::R`](R) reader structure"]
impl crate::Readable for Cidr3Spec {}
#[doc = "`reset()` method sets CIDR3 to value 0xb1"]
impl crate::Resettable for Cidr3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
