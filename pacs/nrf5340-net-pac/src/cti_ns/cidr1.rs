#[doc = "Register `CIDR1` reader"]
pub struct R(crate::R<CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_1` reader - Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code."]
pub type PRMBL_1_R = crate::FieldReader<u8, PRMBL_1_A>;
#[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMBL_1_A {
    #[doc = "0: Bits\\[11:8\\]
of the identification code."]
    VALUE = 0,
}
impl From<PRMBL_1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_1_A) -> Self {
        variant as _
    }
}
impl PRMBL_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRMBL_1_A> {
        match self.bits {
            0 => Some(PRMBL_1_A::VALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == PRMBL_1_A::VALUE
    }
}
#[doc = "Field `CLASS` reader - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code"]
pub type CLASS_R = crate::FieldReader<u8, CLASS_A>;
#[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLASS_A {
    #[doc = "9: Indicates that the component is a CoreSight component."]
    CORESIGHT = 9,
}
impl From<CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLASS_A) -> Self {
        variant as _
    }
}
impl CLASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLASS_A> {
        match self.bits {
            9 => Some(CLASS_A::CORESIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CORESIGHT`"]
    #[inline(always)]
    pub fn is_coresight(&self) -> bool {
        *self == CLASS_A::CORESIGHT
    }
}
impl R {
    #[doc = "Bits 0:3 - Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Component ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr1](index.html) module"]
pub struct CIDR1_SPEC;
impl crate::RegisterSpec for CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr1::R](R) reader structure"]
impl crate::Readable for CIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR1 to value 0x90"]
impl crate::Resettable for CIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
