#[doc = "Register `ENDIANNESS` reader"]
pub type R = crate::R<EndiannessSpec>;
#[doc = "Register `ENDIANNESS` writer"]
pub type W = crate::W<EndiannessSpec>;
#[doc = "DOUT write endianness.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoutWrBg {
    #[doc = "0: Configure DOUT write as little-endian"]
    LittleEndian = 0,
    #[doc = "1: Configure DOUT write as big-endian"]
    BigEndian = 1,
}
impl From<DoutWrBg> for bool {
    #[inline(always)]
    fn from(variant: DoutWrBg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT_WR_BG` reader - DOUT write endianness."]
pub type DoutWrBgR = crate::BitReader<DoutWrBg>;
impl DoutWrBgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DoutWrBg {
        match self.bits {
            false => DoutWrBg::LittleEndian,
            true => DoutWrBg::BigEndian,
        }
    }
    #[doc = "Configure DOUT write as little-endian"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == DoutWrBg::LittleEndian
    }
    #[doc = "Configure DOUT write as big-endian"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == DoutWrBg::BigEndian
    }
}
#[doc = "Field `DOUT_WR_BG` writer - DOUT write endianness."]
pub type DoutWrBgW<'a, REG> = crate::BitWriter<'a, REG, DoutWrBg>;
impl<'a, REG> DoutWrBgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure DOUT write as little-endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DoutWrBg::LittleEndian)
    }
    #[doc = "Configure DOUT write as big-endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DoutWrBg::BigEndian)
    }
}
#[doc = "DIN read endianness.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DinRdBg {
    #[doc = "0: Configure DIN read as little-endian"]
    LittleEndian = 0,
    #[doc = "1: Configure DIN read as big-endian"]
    BigEndian = 1,
}
impl From<DinRdBg> for bool {
    #[inline(always)]
    fn from(variant: DinRdBg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN_RD_BG` reader - DIN read endianness."]
pub type DinRdBgR = crate::BitReader<DinRdBg>;
impl DinRdBgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DinRdBg {
        match self.bits {
            false => DinRdBg::LittleEndian,
            true => DinRdBg::BigEndian,
        }
    }
    #[doc = "Configure DIN read as little-endian"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == DinRdBg::LittleEndian
    }
    #[doc = "Configure DIN read as big-endian"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == DinRdBg::BigEndian
    }
}
#[doc = "Field `DIN_RD_BG` writer - DIN read endianness."]
pub type DinRdBgW<'a, REG> = crate::BitWriter<'a, REG, DinRdBg>;
impl<'a, REG> DinRdBgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure DIN read as little-endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DinRdBg::LittleEndian)
    }
    #[doc = "Configure DIN read as big-endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DinRdBg::BigEndian)
    }
}
#[doc = "DOUT write word endianness.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoutWrWbg {
    #[doc = "0: Configure DOUT write word as little-endian"]
    LittleEndian = 0,
    #[doc = "1: Configure DOUT write word as big-endian"]
    BigEndian = 1,
}
impl From<DoutWrWbg> for bool {
    #[inline(always)]
    fn from(variant: DoutWrWbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT_WR_WBG` reader - DOUT write word endianness."]
pub type DoutWrWbgR = crate::BitReader<DoutWrWbg>;
impl DoutWrWbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DoutWrWbg {
        match self.bits {
            false => DoutWrWbg::LittleEndian,
            true => DoutWrWbg::BigEndian,
        }
    }
    #[doc = "Configure DOUT write word as little-endian"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == DoutWrWbg::LittleEndian
    }
    #[doc = "Configure DOUT write word as big-endian"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == DoutWrWbg::BigEndian
    }
}
#[doc = "Field `DOUT_WR_WBG` writer - DOUT write word endianness."]
pub type DoutWrWbgW<'a, REG> = crate::BitWriter<'a, REG, DoutWrWbg>;
impl<'a, REG> DoutWrWbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure DOUT write word as little-endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DoutWrWbg::LittleEndian)
    }
    #[doc = "Configure DOUT write word as big-endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DoutWrWbg::BigEndian)
    }
}
#[doc = "DIN read word endianness.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DinRdWbg {
    #[doc = "0: Configure DIN read word as little-endian"]
    LittleEndian = 0,
    #[doc = "1: Configure DIN read word as big-endian"]
    BigEndian = 1,
}
impl From<DinRdWbg> for bool {
    #[inline(always)]
    fn from(variant: DinRdWbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN_RD_WBG` reader - DIN read word endianness."]
pub type DinRdWbgR = crate::BitReader<DinRdWbg>;
impl DinRdWbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DinRdWbg {
        match self.bits {
            false => DinRdWbg::LittleEndian,
            true => DinRdWbg::BigEndian,
        }
    }
    #[doc = "Configure DIN read word as little-endian"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == DinRdWbg::LittleEndian
    }
    #[doc = "Configure DIN read word as big-endian"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == DinRdWbg::BigEndian
    }
}
#[doc = "Field `DIN_RD_WBG` writer - DIN read word endianness."]
pub type DinRdWbgW<'a, REG> = crate::BitWriter<'a, REG, DinRdWbg>;
impl<'a, REG> DinRdWbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure DIN read word as little-endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DinRdWbg::LittleEndian)
    }
    #[doc = "Configure DIN read word as big-endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(DinRdWbg::BigEndian)
    }
}
impl R {
    #[doc = "Bit 3 - DOUT write endianness."]
    #[inline(always)]
    pub fn dout_wr_bg(&self) -> DoutWrBgR {
        DoutWrBgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - DIN read endianness."]
    #[inline(always)]
    pub fn din_rd_bg(&self) -> DinRdBgR {
        DinRdBgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - DOUT write word endianness."]
    #[inline(always)]
    pub fn dout_wr_wbg(&self) -> DoutWrWbgR {
        DoutWrWbgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - DIN read word endianness."]
    #[inline(always)]
    pub fn din_rd_wbg(&self) -> DinRdWbgR {
        DinRdWbgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DOUT write endianness."]
    #[inline(always)]
    pub fn dout_wr_bg(&mut self) -> DoutWrBgW<'_, EndiannessSpec> {
        DoutWrBgW::new(self, 3)
    }
    #[doc = "Bit 7 - DIN read endianness."]
    #[inline(always)]
    pub fn din_rd_bg(&mut self) -> DinRdBgW<'_, EndiannessSpec> {
        DinRdBgW::new(self, 7)
    }
    #[doc = "Bit 11 - DOUT write word endianness."]
    #[inline(always)]
    pub fn dout_wr_wbg(&mut self) -> DoutWrWbgW<'_, EndiannessSpec> {
        DoutWrWbgW::new(self, 11)
    }
    #[doc = "Bit 15 - DIN read word endianness."]
    #[inline(always)]
    pub fn din_rd_wbg(&mut self) -> DinRdWbgW<'_, EndiannessSpec> {
        DinRdWbgW::new(self, 15)
    }
}
#[doc = "This register defines the endianness of the Host-accessible registers, and can only be written once.\n\nYou can [`read`](crate::Reg::read) this register and get [`endianness::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endianness::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndiannessSpec;
impl crate::RegisterSpec for EndiannessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endianness::R`](R) reader structure"]
impl crate::Readable for EndiannessSpec {}
#[doc = "`write(|w| ..)` method takes [`endianness::W`](W) writer structure"]
impl crate::Writable for EndiannessSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENDIANNESS to value 0"]
impl crate::Resettable for EndiannessSpec {}
