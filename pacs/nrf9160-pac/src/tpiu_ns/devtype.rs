#[doc = "Register `DEVTYPE` reader"]
pub type R = crate::R<DevtypeSpec>;
#[doc = "The main type of the component\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Major {
    #[doc = "1: Peripheral is a trace sink."]
    TraceSource = 1,
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
            1 => Some(Major::TraceSource),
            _ => None,
        }
    }
    #[doc = "Peripheral is a trace sink."]
    #[inline(always)]
    pub fn is_trace_source(&self) -> bool {
        *self == Major::TraceSource
    }
}
#[doc = "The sub-type of the component\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub {
    #[doc = "1: Indicates that this component is a trace port component."]
    TracePort = 1,
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
            1 => Some(Sub::TracePort),
            _ => None,
        }
    }
    #[doc = "Indicates that this component is a trace port component."]
    #[inline(always)]
    pub fn is_trace_port(&self) -> bool {
        *self == Sub::TracePort
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
#[doc = "The DEVTYPE register provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevtypeSpec;
impl crate::RegisterSpec for DevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devtype::R`](R) reader structure"]
impl crate::Readable for DevtypeSpec {}
#[doc = "`reset()` method sets DEVTYPE to value 0"]
impl crate::Resettable for DevtypeSpec {}
