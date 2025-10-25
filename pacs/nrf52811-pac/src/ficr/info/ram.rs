#[doc = "Register `RAM` reader"]
pub type R = crate::R<RamSpec>;
#[doc = "RAM variant\n\nValue on reset: 24"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ram {
    #[doc = "24: 24 kByte RAM"]
    K24 = 24,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Ram> for u32 {
    #[inline(always)]
    fn from(variant: Ram) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram {
    type Ux = u32;
}
impl crate::IsEnum for Ram {}
#[doc = "Field `RAM` reader - RAM variant"]
pub type RamR = crate::FieldReader<Ram>;
impl RamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ram> {
        match self.bits {
            24 => Some(Ram::K24),
            4294967295 => Some(Ram::Unspecified),
            _ => None,
        }
    }
    #[doc = "24 kByte RAM"]
    #[inline(always)]
    pub fn is_k24(&self) -> bool {
        *self == Ram::K24
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Ram::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(self.bits)
    }
}
#[doc = "RAM variant\n\nYou can [`read`](crate::Reg::read) this register and get [`ram::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamSpec;
impl crate::RegisterSpec for RamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram::R`](R) reader structure"]
impl crate::Readable for RamSpec {}
#[doc = "`reset()` method sets RAM to value 0x18"]
impl crate::Resettable for RamSpec {
    const RESET_VALUE: u32 = 0x18;
}
