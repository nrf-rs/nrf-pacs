#[doc = "Register `DIN_DMA_SRAM_ENDIANNESS` reader"]
pub type R = crate::R<DinDmaSramEndiannessSpec>;
#[doc = "Register `DIN_DMA_SRAM_ENDIANNESS` writer"]
pub type W = crate::W<DinDmaSramEndiannessSpec>;
#[doc = "Endianness of DIN DMA transactions towards RNG SRAM. The default value is little-endian.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Use little-endian format for RNG SRAM DMA transactions"]
    LittleEndian = 0,
    #[doc = "1: Use big-endian format for RNG SRAM DMA transactions"]
    BigEndian = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIAN` reader - Endianness of DIN DMA transactions towards RNG SRAM. The default value is little-endian."]
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
    #[doc = "Use little-endian format for RNG SRAM DMA transactions"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == Endian::LittleEndian
    }
    #[doc = "Use big-endian format for RNG SRAM DMA transactions"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == Endian::BigEndian
    }
}
#[doc = "Field `ENDIAN` writer - Endianness of DIN DMA transactions towards RNG SRAM. The default value is little-endian."]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use little-endian format for RNG SRAM DMA transactions"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::LittleEndian)
    }
    #[doc = "Use big-endian format for RNG SRAM DMA transactions"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::BigEndian)
    }
}
impl R {
    #[doc = "Bit 0 - Endianness of DIN DMA transactions towards RNG SRAM. The default value is little-endian."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endianness of DIN DMA transactions towards RNG SRAM. The default value is little-endian."]
    #[inline(always)]
    pub fn endian(&mut self) -> EndianW<'_, DinDmaSramEndiannessSpec> {
        EndianW::new(self, 0)
    }
}
#[doc = "Configure the endianness of DIN DMA transactions towards RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`din_dma_sram_endianness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_dma_sram_endianness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinDmaSramEndiannessSpec;
impl crate::RegisterSpec for DinDmaSramEndiannessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_dma_sram_endianness::R`](R) reader structure"]
impl crate::Readable for DinDmaSramEndiannessSpec {}
#[doc = "`write(|w| ..)` method takes [`din_dma_sram_endianness::W`](W) writer structure"]
impl crate::Writable for DinDmaSramEndiannessSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIN_DMA_SRAM_ENDIANNESS to value 0"]
impl crate::Resettable for DinDmaSramEndiannessSpec {}
