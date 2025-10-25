#[doc = "Register `PART` reader"]
pub type R = crate::R<PartSpec>;
#[doc = "Part code\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Part {
    #[doc = "37217: nRF9161"]
    N9161 = 37217,
    #[doc = "37216: nRF9160"]
    N9160 = 37216,
    #[doc = "37201: nRF9151"]
    N9151 = 37201,
    #[doc = "37169: nRF9131"]
    N9131 = 37169,
    #[doc = "37152: nRF9120"]
    N9120 = 37152,
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
            37217 => Some(Part::N9161),
            37216 => Some(Part::N9160),
            37201 => Some(Part::N9151),
            37169 => Some(Part::N9131),
            37152 => Some(Part::N9120),
            _ => None,
        }
    }
    #[doc = "nRF9161"]
    #[inline(always)]
    pub fn is_n9161(&self) -> bool {
        *self == Part::N9161
    }
    #[doc = "nRF9160"]
    #[inline(always)]
    pub fn is_n9160(&self) -> bool {
        *self == Part::N9160
    }
    #[doc = "nRF9151"]
    #[inline(always)]
    pub fn is_n9151(&self) -> bool {
        *self == Part::N9151
    }
    #[doc = "nRF9131"]
    #[inline(always)]
    pub fn is_n9131(&self) -> bool {
        *self == Part::N9131
    }
    #[doc = "nRF9120"]
    #[inline(always)]
    pub fn is_n9120(&self) -> bool {
        *self == Part::N9120
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
#[doc = "`reset()` method sets PART to value 0xffff_ffff"]
impl crate::Resettable for PartSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
