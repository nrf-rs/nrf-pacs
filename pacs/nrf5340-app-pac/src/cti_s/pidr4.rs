#[doc = "Register `PIDR4` reader"]
pub struct R(crate::R<PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DES_2_A {
    #[doc = "4: JEDEC continuation code."]
    CODE = 4,
}
impl From<DES_2_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DES_2` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub struct DES_2_R(crate::FieldReader<u8, DES_2_A>);
impl DES_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DES_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DES_2_A> {
        match self.bits {
            4 => Some(DES_2_A::CODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CODE`"]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        **self == DES_2_A::CODE
    }
}
impl core::ops::Deref for DES_2_R {
    type Target = crate::FieldReader<u8, DES_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` reader - Always 0b0000. Indicates that the device only occupies 4KB of memory."]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Always 0b0000. Indicates that the device only occupies 4KB of memory."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](index.html) module"]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr4::R](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for PIDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
