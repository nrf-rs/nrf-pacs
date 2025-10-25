#[doc = "Register `VARIANT` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Variant {
    #[doc = "1363886401: QKAA"]
    Qkaa = 1363886401,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Variant> for u32 {
    #[inline(always)]
    fn from(variant: Variant) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Variant {
    type Ux = u32;
}
impl crate::IsEnum for Variant {}
#[doc = "Field `VARIANT` reader - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            1363886401 => Some(Variant::Qkaa),
            4294967295 => Some(Variant::Unspecified),
            _ => None,
        }
    }
    #[doc = "QKAA"]
    #[inline(always)]
    pub fn is_qkaa(&self) -> bool {
        *self == Variant::Qkaa
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Variant::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Part Variant, Hardware version and Production configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT to value 0xffff_ffff"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
