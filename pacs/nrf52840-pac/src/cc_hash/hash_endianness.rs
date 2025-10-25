#[doc = "Register `HASH_ENDIANNESS` reader"]
pub type R = crate::R<HashEndiannessSpec>;
#[doc = "Register `HASH_ENDIANNESS` writer"]
pub type W = crate::W<HashEndiannessSpec>;
#[doc = "Endianness of HASH data and padding generation. The default value is little-endian.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Use little-endian format for data and padding"]
    LittleEndian = 0,
    #[doc = "1: Use big-endian format for data and padding"]
    BigEndian = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIAN` reader - Endianness of HASH data and padding generation. The default value is little-endian."]
pub type EndianR = crate::BitReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::LittleEndian,
            true => Endian::BigEndian,
        }
    }
    #[doc = "Use little-endian format for data and padding"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == Endian::LittleEndian
    }
    #[doc = "Use big-endian format for data and padding"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == Endian::BigEndian
    }
}
#[doc = "Field `ENDIAN` writer - Endianness of HASH data and padding generation. The default value is little-endian."]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use little-endian format for data and padding"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::LittleEndian)
    }
    #[doc = "Use big-endian format for data and padding"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::BigEndian)
    }
}
impl R {
    #[doc = "Bit 0 - Endianness of HASH data and padding generation. The default value is little-endian."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endianness of HASH data and padding generation. The default value is little-endian."]
    #[inline(always)]
    pub fn endian(&mut self) -> EndianW<'_, HashEndiannessSpec> {
        EndianW::new(self, 0)
    }
}
#[doc = "Configure the endianness of HASH data and padding generation.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_endianness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_endianness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashEndiannessSpec;
impl crate::RegisterSpec for HashEndiannessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_endianness::R`](R) reader structure"]
impl crate::Readable for HashEndiannessSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_endianness::W`](W) writer structure"]
impl crate::Writable for HashEndiannessSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_ENDIANNESS to value 0x01"]
impl crate::Resettable for HashEndiannessSpec {
    const RESET_VALUE: u32 = 0x01;
}
