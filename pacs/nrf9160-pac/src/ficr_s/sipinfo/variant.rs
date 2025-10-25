#[doc = "Register `VARIANT[%s]` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Variant {
    #[doc = "65: Unspecified"]
    A = 65,
    #[doc = "66: Unspecified"]
    B = 66,
    #[doc = "67: Unspecified"]
    C = 67,
    #[doc = "73: Unspecified"]
    I = 73,
    #[doc = "76: Unspecified"]
    L = 76,
    #[doc = "83: Unspecified"]
    S = 83,
}
impl From<Variant> for u8 {
    #[inline(always)]
    fn from(variant: Variant) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Variant {
    type Ux = u8;
}
impl crate::IsEnum for Variant {}
#[doc = "Field `VARIANT` reader - "]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            65 => Some(Variant::A),
            66 => Some(Variant::B),
            67 => Some(Variant::C),
            73 => Some(Variant::I),
            76 => Some(Variant::L),
            83 => Some(Variant::S),
            _ => None,
        }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Variant::A
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Variant::B
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Variant::C
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == Variant::I
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == Variant::L
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_s(&self) -> bool {
        *self == Variant::S
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Description collection: SIP VARIANT, encoded in ASCII, for example SIAA, SIBA or SICA. See Ordering information for details.\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT[%s] to value 0xff"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u8 = 0xff;
}
