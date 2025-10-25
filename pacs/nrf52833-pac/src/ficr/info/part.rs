#[doc = "Register `PART` reader"]
pub type R = crate::R<PartSpec>;
#[doc = "Part code\n\nValue on reset: 337971"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Part {
    #[doc = "337952: nRF52820"]
    N52820 = 337952,
    #[doc = "337971: nRF52833"]
    N52833 = 337971,
    #[doc = "337984: nRF52840"]
    N52840 = 337984,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Part> for u32 {
    #[inline(always)]
    fn from(variant: Part) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Part {
    type Ux = u32;
}
impl crate::IsEnum for Part {}
#[doc = "Field `PART` reader - Part code"]
pub type PartR = crate::FieldReader<Part>;
impl PartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Part> {
        match self.bits {
            337952 => Some(Part::N52820),
            337971 => Some(Part::N52833),
            337984 => Some(Part::N52840),
            4294967295 => Some(Part::Unspecified),
            _ => None,
        }
    }
    #[doc = "nRF52820"]
    #[inline(always)]
    pub fn is_n52820(&self) -> bool {
        *self == Part::N52820
    }
    #[doc = "nRF52833"]
    #[inline(always)]
    pub fn is_n52833(&self) -> bool {
        *self == Part::N52833
    }
    #[doc = "nRF52840"]
    #[inline(always)]
    pub fn is_n52840(&self) -> bool {
        *self == Part::N52840
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Part::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PartR {
        PartR::new(self.bits)
    }
}
#[doc = "Part code\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartSpec;
impl crate::RegisterSpec for PartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`part::R`](R) reader structure"]
impl crate::Readable for PartSpec {}
#[doc = "`reset()` method sets PART to value 0x0005_2833"]
impl crate::Resettable for PartSpec {
    const RESET_VALUE: u32 = 0x0005_2833;
}
