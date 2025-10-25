#[doc = "Register `PPFC` reader"]
pub type R = crate::R<PpfcSpec>;
#[doc = "Pre-programmed factory code present.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ppfc {
    #[doc = "255: Not present."]
    NotPresent = 255,
    #[doc = "0: Present."]
    Present = 0,
}
impl From<Ppfc> for u8 {
    #[inline(always)]
    fn from(variant: Ppfc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ppfc {
    type Ux = u8;
}
impl crate::IsEnum for Ppfc {}
#[doc = "Field `PPFC` reader - Pre-programmed factory code present."]
pub type PpfcR = crate::FieldReader<Ppfc>;
impl PpfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ppfc> {
        match self.bits {
            255 => Some(Ppfc::NotPresent),
            0 => Some(Ppfc::Present),
            _ => None,
        }
    }
    #[doc = "Not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Ppfc::NotPresent
    }
    #[doc = "Present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Ppfc::Present
    }
}
impl R {
    #[doc = "Bits 0:7 - Pre-programmed factory code present."]
    #[inline(always)]
    pub fn ppfc(&self) -> PpfcR {
        PpfcR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Pre-programmed factory code present.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpfcSpec;
impl crate::RegisterSpec for PpfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppfc::R`](R) reader structure"]
impl crate::Readable for PpfcSpec {}
#[doc = "`reset()` method sets PPFC to value 0xffff_ffff"]
impl crate::Resettable for PpfcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
