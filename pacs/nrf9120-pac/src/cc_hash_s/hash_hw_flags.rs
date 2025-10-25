#[doc = "Register `HASH_HW_FLAGS` reader"]
pub type R = crate::R<HashHwFlagsSpec>;
#[doc = "Indicates the number of concurrent words the hash is using to compute signature.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cw {
    #[doc = "1: One concurrent word used by hash during signature generation"]
    One = 1,
    #[doc = "2: Two concurrent words used by hash during signature generation"]
    Two = 2,
}
impl From<Cw> for u8 {
    #[inline(always)]
    fn from(variant: Cw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cw {
    type Ux = u8;
}
impl crate::IsEnum for Cw {}
#[doc = "Field `CW` reader - Indicates the number of concurrent words the hash is using to compute signature."]
pub type CwR = crate::FieldReader<Cw>;
impl CwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cw> {
        match self.bits {
            1 => Some(Cw::One),
            2 => Some(Cw::Two),
            _ => None,
        }
    }
    #[doc = "One concurrent word used by hash during signature generation"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Cw::One
    }
    #[doc = "Two concurrent words used by hash during signature generation"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Cw::Two
    }
}
#[doc = "Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch {
    #[doc = "0: One Hi value is updated at a time."]
    One = 0,
    #[doc = "1: All Hi values are updated at the same time."]
    All = 1,
}
impl From<Ch> for u8 {
    #[inline(always)]
    fn from(variant: Ch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch {
    type Ux = u8;
}
impl crate::IsEnum for Ch {}
#[doc = "Field `CH` reader - Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi."]
pub type ChR = crate::FieldReader<Ch>;
impl ChR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch> {
        match self.bits {
            0 => Some(Ch::One),
            1 => Some(Ch::All),
            _ => None,
        }
    }
    #[doc = "One Hi value is updated at a time."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Ch::One
    }
    #[doc = "All Hi values are updated at the same time."]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Ch::All
    }
}
#[doc = "Determine the granularity of word size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dw {
    #[doc = "0: 32 bits word data."]
    _32bits = 0,
    #[doc = "1: 64 bits word data."]
    _64bits = 1,
}
impl From<Dw> for u8 {
    #[inline(always)]
    fn from(variant: Dw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dw {
    type Ux = u8;
}
impl crate::IsEnum for Dw {}
#[doc = "Field `DW` reader - Determine the granularity of word size."]
pub type DwR = crate::FieldReader<Dw>;
impl DwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dw> {
        match self.bits {
            0 => Some(Dw::_32bits),
            1 => Some(Dw::_64bits),
            _ => None,
        }
    }
    #[doc = "32 bits word data."]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == Dw::_32bits
    }
    #[doc = "64 bits word data."]
    #[inline(always)]
    pub fn is_64bits(&self) -> bool {
        *self == Dw::_64bits
    }
}
#[doc = "Field `SHA_512_EXISTS` reader - If this flag is set, the engine include SHA-512 support."]
pub type Sha512ExistsR = crate::BitReader;
#[doc = "Field `PAD_EXISTS` reader - If this flag is set, the engine include pad block support."]
pub type PadExistsR = crate::BitReader;
#[doc = "Field `MD5_EXISTS` reader - If this flag is set, the engine include MD5 support."]
pub type Md5ExistsR = crate::BitReader;
#[doc = "Field `HMAC_EXISTS` reader - If this flag is set, the engine include HMAC support."]
pub type HmacExistsR = crate::BitReader;
#[doc = "Field `SHA_256_EXISTS` reader - If this flag is set, the engine include SHA-256 support."]
pub type Sha256ExistsR = crate::BitReader;
#[doc = "Field `HASH_COMPARE_EXISTS` reader - If this flag is set, the engine include compare digest logic."]
pub type HashCompareExistsR = crate::BitReader;
#[doc = "Field `DUMP_HASH_TO_DOUT_EXISTS` reader - If this flag is set, the engine include HASH to DOUT support."]
pub type DumpHashToDoutExistsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Indicates the number of concurrent words the hash is using to compute signature."]
    #[inline(always)]
    pub fn cw(&self) -> CwR {
        CwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicate if Hi adders are present for each Hi value or 1 adder is shared for all Hi."]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Determine the granularity of word size."]
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - If this flag is set, the engine include SHA-512 support."]
    #[inline(always)]
    pub fn sha_512_exists(&self) -> Sha512ExistsR {
        Sha512ExistsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If this flag is set, the engine include pad block support."]
    #[inline(always)]
    pub fn pad_exists(&self) -> PadExistsR {
        PadExistsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this flag is set, the engine include MD5 support."]
    #[inline(always)]
    pub fn md5_exists(&self) -> Md5ExistsR {
        Md5ExistsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If this flag is set, the engine include HMAC support."]
    #[inline(always)]
    pub fn hmac_exists(&self) -> HmacExistsR {
        HmacExistsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - If this flag is set, the engine include SHA-256 support."]
    #[inline(always)]
    pub fn sha_256_exists(&self) -> Sha256ExistsR {
        Sha256ExistsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If this flag is set, the engine include compare digest logic."]
    #[inline(always)]
    pub fn hash_compare_exists(&self) -> HashCompareExistsR {
        HashCompareExistsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If this flag is set, the engine include HASH to DOUT support."]
    #[inline(always)]
    pub fn dump_hash_to_dout_exists(&self) -> DumpHashToDoutExistsR {
        DumpHashToDoutExistsR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Hardware configuration of the HASH engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hw_flags::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHwFlagsSpec;
impl crate::RegisterSpec for HashHwFlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hw_flags::R`](R) reader structure"]
impl crate::Readable for HashHwFlagsSpec {}
#[doc = "`reset()` method sets HASH_HW_FLAGS to value 0x0001_2001"]
impl crate::Resettable for HashHwFlagsSpec {
    const RESET_VALUE: u32 = 0x0001_2001;
}
