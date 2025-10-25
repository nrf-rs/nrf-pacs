#[doc = "Register `TRCDEVTYPE` reader"]
pub type R = crate::R<TrcdevtypeSpec>;
#[doc = "The main type of the component\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Major {
    #[doc = "3: Peripheral is a trace source."]
    TraceSource = 3,
}
impl From<Major> for u8 {
    #[inline(always)]
    fn from(variant: Major) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Major {
    type Ux = u8;
}
impl crate::IsEnum for Major {}
#[doc = "Field `MAJOR` reader - The main type of the component"]
pub type MajorR = crate::FieldReader<Major>;
impl MajorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Major> {
        match self.bits {
            3 => Some(Major::TraceSource),
            _ => None,
        }
    }
    #[doc = "Peripheral is a trace source."]
    #[inline(always)]
    pub fn is_trace_source(&self) -> bool {
        *self == Major::TraceSource
    }
}
#[doc = "The sub-type of the component\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub {
    #[doc = "1: Peripheral is a processor trace source."]
    ProcessorTrace = 1,
}
impl From<Sub> for u8 {
    #[inline(always)]
    fn from(variant: Sub) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sub {
    type Ux = u8;
}
impl crate::IsEnum for Sub {}
#[doc = "Field `SUB` reader - The sub-type of the component"]
pub type SubR = crate::FieldReader<Sub>;
impl SubR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sub> {
        match self.bits {
            1 => Some(Sub::ProcessorTrace),
            _ => None,
        }
    }
    #[doc = "Peripheral is a processor trace source."]
    #[inline(always)]
    pub fn is_processor_trace(&self) -> bool {
        *self == Sub::ProcessorTrace
    }
}
impl R {
    #[doc = "Bits 0:3 - The main type of the component"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The sub-type of the component"]
    #[inline(always)]
    pub fn sub(&self) -> SubR {
        SubR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Controls the single-shot comparator.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcdevtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcdevtypeSpec;
impl crate::RegisterSpec for TrcdevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcdevtype::R`](R) reader structure"]
impl crate::Readable for TrcdevtypeSpec {}
#[doc = "`reset()` method sets TRCDEVTYPE to value 0"]
impl crate::Resettable for TrcdevtypeSpec {}
