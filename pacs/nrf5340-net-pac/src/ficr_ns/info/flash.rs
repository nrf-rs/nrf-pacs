#[doc = "Register `FLASH` reader"]
pub type R = crate::R<FlashSpec>;
#[doc = "Flash variant\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Flash {
    #[doc = "128: 128 kByte FLASH"]
    K128 = 128,
    #[doc = "256: 256 kByte FLASH"]
    K256 = 256,
    #[doc = "512: 512 kByte FLASH"]
    K512 = 512,
    #[doc = "1024: 1 MByte FLASH"]
    K1024 = 1024,
    #[doc = "2048: 2 MByte FLASH"]
    K2048 = 2048,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Flash> for u32 {
    #[inline(always)]
    fn from(variant: Flash) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flash {
    type Ux = u32;
}
impl crate::IsEnum for Flash {}
#[doc = "Field `FLASH` reader - Flash variant"]
pub type FlashR = crate::FieldReader<Flash>;
impl FlashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flash> {
        match self.bits {
            128 => Some(Flash::K128),
            256 => Some(Flash::K256),
            512 => Some(Flash::K512),
            1024 => Some(Flash::K1024),
            2048 => Some(Flash::K2048),
            4294967295 => Some(Flash::Unspecified),
            _ => None,
        }
    }
    #[doc = "128 kByte FLASH"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        *self == Flash::K128
    }
    #[doc = "256 kByte FLASH"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == Flash::K256
    }
    #[doc = "512 kByte FLASH"]
    #[inline(always)]
    pub fn is_k512(&self) -> bool {
        *self == Flash::K512
    }
    #[doc = "1 MByte FLASH"]
    #[inline(always)]
    pub fn is_k1024(&self) -> bool {
        *self == Flash::K1024
    }
    #[doc = "2 MByte FLASH"]
    #[inline(always)]
    pub fn is_k2048(&self) -> bool {
        *self == Flash::K2048
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Flash::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(self.bits)
    }
}
#[doc = "Flash variant\n\nYou can [`read`](crate::Reg::read) this register and get [`flash::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSpec;
impl crate::RegisterSpec for FlashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash::R`](R) reader structure"]
impl crate::Readable for FlashSpec {}
#[doc = "`reset()` method sets FLASH to value 0xffff_ffff"]
impl crate::Resettable for FlashSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
