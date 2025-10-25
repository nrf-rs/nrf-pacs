#[doc = "Register `RAM` reader"]
pub type R = crate::R<RamSpec>;
#[doc = "RAM variant\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ram {
    #[doc = "16: 16 kByte RAM"]
    K16 = 16,
    #[doc = "32: 32 kByte RAM"]
    K32 = 32,
    #[doc = "64: 64 kByte RAM"]
    K64 = 64,
    #[doc = "128: 128 kByte RAM"]
    K128 = 128,
    #[doc = "256: 256 kByte RAM"]
    K256 = 256,
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
            16 => Some(Ram::K16),
            32 => Some(Ram::K32),
            64 => Some(Ram::K64),
            128 => Some(Ram::K128),
            256 => Some(Ram::K256),
            4294967295 => Some(Ram::Unspecified),
            _ => None,
        }
    }
    #[doc = "16 kByte RAM"]
    #[inline(always)]
    pub fn is_k16(&self) -> bool {
        *self == Ram::K16
    }
    #[doc = "32 kByte RAM"]
    #[inline(always)]
    pub fn is_k32(&self) -> bool {
        *self == Ram::K32
    }
    #[doc = "64 kByte RAM"]
    #[inline(always)]
    pub fn is_k64(&self) -> bool {
        *self == Ram::K64
    }
    #[doc = "128 kByte RAM"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        *self == Ram::K128
    }
    #[doc = "256 kByte RAM"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == Ram::K256
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
#[doc = "`reset()` method sets RAM to value 0xffff_ffff"]
impl crate::Resettable for RamSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
