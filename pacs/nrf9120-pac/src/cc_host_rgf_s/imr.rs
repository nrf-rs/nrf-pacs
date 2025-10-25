#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "The RNG SRAM to DIN DMA done interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramToDinMask {
    #[doc = "0: Do not mask RNG SRAM to DIN DMA done interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask RNG SRAM to DIN DMA done interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<SramToDinMask> for bool {
    #[inline(always)]
    fn from(variant: SramToDinMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_TO_DIN_MASK` reader - The RNG SRAM to DIN DMA done interrupt mask."]
pub type SramToDinMaskR = crate::BitReader<SramToDinMask>;
impl SramToDinMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramToDinMask {
        match self.bits {
            false => SramToDinMask::Irqenable,
            true => SramToDinMask::Irqdisable,
        }
    }
    #[doc = "Do not mask RNG SRAM to DIN DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == SramToDinMask::Irqenable
    }
    #[doc = "Mask RNG SRAM to DIN DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == SramToDinMask::Irqdisable
    }
}
#[doc = "Field `SRAM_TO_DIN_MASK` writer - The RNG SRAM to DIN DMA done interrupt mask."]
pub type SramToDinMaskW<'a, REG> = crate::BitWriter<'a, REG, SramToDinMask>;
impl<'a, REG> SramToDinMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask RNG SRAM to DIN DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(SramToDinMask::Irqenable)
    }
    #[doc = "Mask RNG SRAM to DIN DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(SramToDinMask::Irqdisable)
    }
}
#[doc = "The DOUT to RNG SRAM DMA done interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoutToSramMask {
    #[doc = "0: Do not mask DOUT to RNG SRAM DMA done interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask DOUT to RNG SRAM DMA done interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<DoutToSramMask> for bool {
    #[inline(always)]
    fn from(variant: DoutToSramMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT_TO_SRAM_MASK` reader - The DOUT to RNG SRAM DMA done interrupt mask."]
pub type DoutToSramMaskR = crate::BitReader<DoutToSramMask>;
impl DoutToSramMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DoutToSramMask {
        match self.bits {
            false => DoutToSramMask::Irqenable,
            true => DoutToSramMask::Irqdisable,
        }
    }
    #[doc = "Do not mask DOUT to RNG SRAM DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == DoutToSramMask::Irqenable
    }
    #[doc = "Mask DOUT to RNG SRAM DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == DoutToSramMask::Irqdisable
    }
}
#[doc = "Field `DOUT_TO_SRAM_MASK` writer - The DOUT to RNG SRAM DMA done interrupt mask."]
pub type DoutToSramMaskW<'a, REG> = crate::BitWriter<'a, REG, DoutToSramMask>;
impl<'a, REG> DoutToSramMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask DOUT to RNG SRAM DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(DoutToSramMask::Irqenable)
    }
    #[doc = "Mask DOUT to RNG SRAM DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(DoutToSramMask::Irqdisable)
    }
}
#[doc = "The memory to DIN DMA done interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemToDinMask {
    #[doc = "0: Do not mask memory to DIN DMA done interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask memory to DIN DMA done interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<MemToDinMask> for bool {
    #[inline(always)]
    fn from(variant: MemToDinMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM_TO_DIN_MASK` reader - The memory to DIN DMA done interrupt mask."]
pub type MemToDinMaskR = crate::BitReader<MemToDinMask>;
impl MemToDinMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemToDinMask {
        match self.bits {
            false => MemToDinMask::Irqenable,
            true => MemToDinMask::Irqdisable,
        }
    }
    #[doc = "Do not mask memory to DIN DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == MemToDinMask::Irqenable
    }
    #[doc = "Mask memory to DIN DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == MemToDinMask::Irqdisable
    }
}
#[doc = "Field `MEM_TO_DIN_MASK` writer - The memory to DIN DMA done interrupt mask."]
pub type MemToDinMaskW<'a, REG> = crate::BitWriter<'a, REG, MemToDinMask>;
impl<'a, REG> MemToDinMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask memory to DIN DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(MemToDinMask::Irqenable)
    }
    #[doc = "Mask memory to DIN DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(MemToDinMask::Irqdisable)
    }
}
#[doc = "The DOUT to memory DMA done interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoutToMemMask {
    #[doc = "0: Do not mask DOUT to memory DMA done interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask DOUT to memory DMA done interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<DoutToMemMask> for bool {
    #[inline(always)]
    fn from(variant: DoutToMemMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT_TO_MEM_MASK` reader - The DOUT to memory DMA done interrupt mask."]
pub type DoutToMemMaskR = crate::BitReader<DoutToMemMask>;
impl DoutToMemMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DoutToMemMask {
        match self.bits {
            false => DoutToMemMask::Irqenable,
            true => DoutToMemMask::Irqdisable,
        }
    }
    #[doc = "Do not mask DOUT to memory DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == DoutToMemMask::Irqenable
    }
    #[doc = "Mask DOUT to memory DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == DoutToMemMask::Irqdisable
    }
}
#[doc = "Field `DOUT_TO_MEM_MASK` writer - The DOUT to memory DMA done interrupt mask."]
pub type DoutToMemMaskW<'a, REG> = crate::BitWriter<'a, REG, DoutToMemMask>;
impl<'a, REG> DoutToMemMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask DOUT to memory DMA done interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(DoutToMemMask::Irqenable)
    }
    #[doc = "Mask DOUT to memory DMA done interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(DoutToMemMask::Irqdisable)
    }
}
#[doc = "The AHB error interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbErrMask {
    #[doc = "0: Do not mask AHB error interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask AHB error interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<AhbErrMask> for bool {
    #[inline(always)]
    fn from(variant: AhbErrMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_ERR_MASK` reader - The AHB error interrupt mask."]
pub type AhbErrMaskR = crate::BitReader<AhbErrMask>;
impl AhbErrMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbErrMask {
        match self.bits {
            false => AhbErrMask::Irqenable,
            true => AhbErrMask::Irqdisable,
        }
    }
    #[doc = "Do not mask AHB error interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == AhbErrMask::Irqenable
    }
    #[doc = "Mask AHB error interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == AhbErrMask::Irqdisable
    }
}
#[doc = "Field `AHB_ERR_MASK` writer - The AHB error interrupt mask."]
pub type AhbErrMaskW<'a, REG> = crate::BitWriter<'a, REG, AhbErrMask>;
impl<'a, REG> AhbErrMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask AHB error interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(AhbErrMask::Irqenable)
    }
    #[doc = "Mask AHB error interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(AhbErrMask::Irqdisable)
    }
}
#[doc = "The PKA end of operation interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkaMask {
    #[doc = "0: Do not mask PKA end of operation interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask PKA end of operation interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<PkaMask> for bool {
    #[inline(always)]
    fn from(variant: PkaMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKA_MASK` reader - The PKA end of operation interrupt mask."]
pub type PkaMaskR = crate::BitReader<PkaMask>;
impl PkaMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PkaMask {
        match self.bits {
            false => PkaMask::Irqenable,
            true => PkaMask::Irqdisable,
        }
    }
    #[doc = "Do not mask PKA end of operation interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == PkaMask::Irqenable
    }
    #[doc = "Mask PKA end of operation interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == PkaMask::Irqdisable
    }
}
#[doc = "Field `PKA_MASK` writer - The PKA end of operation interrupt mask."]
pub type PkaMaskW<'a, REG> = crate::BitWriter<'a, REG, PkaMask>;
impl<'a, REG> PkaMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask PKA end of operation interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(PkaMask::Irqenable)
    }
    #[doc = "Mask PKA end of operation interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(PkaMask::Irqdisable)
    }
}
#[doc = "The RNG interrupt mask.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngMask {
    #[doc = "0: Do not mask RNG interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask RNG interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<RngMask> for bool {
    #[inline(always)]
    fn from(variant: RngMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_MASK` reader - The RNG interrupt mask."]
pub type RngMaskR = crate::BitReader<RngMask>;
impl RngMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RngMask {
        match self.bits {
            false => RngMask::Irqenable,
            true => RngMask::Irqdisable,
        }
    }
    #[doc = "Do not mask RNG interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == RngMask::Irqenable
    }
    #[doc = "Mask RNG interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == RngMask::Irqdisable
    }
}
#[doc = "Field `RNG_MASK` writer - The RNG interrupt mask."]
pub type RngMaskW<'a, REG> = crate::BitWriter<'a, REG, RngMask>;
impl<'a, REG> RngMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask RNG interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(RngMask::Irqenable)
    }
    #[doc = "Mask RNG interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(RngMask::Irqdisable)
    }
}
impl R {
    #[doc = "Bit 4 - The RNG SRAM to DIN DMA done interrupt mask."]
    #[inline(always)]
    pub fn sram_to_din_mask(&self) -> SramToDinMaskR {
        SramToDinMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The DOUT to RNG SRAM DMA done interrupt mask."]
    #[inline(always)]
    pub fn dout_to_sram_mask(&self) -> DoutToSramMaskR {
        DoutToSramMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt mask."]
    #[inline(always)]
    pub fn mem_to_din_mask(&self) -> MemToDinMaskR {
        MemToDinMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt mask."]
    #[inline(always)]
    pub fn dout_to_mem_mask(&self) -> DoutToMemMaskR {
        DoutToMemMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The AHB error interrupt mask."]
    #[inline(always)]
    pub fn ahb_err_mask(&self) -> AhbErrMaskR {
        AhbErrMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt mask."]
    #[inline(always)]
    pub fn pka_mask(&self) -> PkaMaskR {
        PkaMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The RNG interrupt mask."]
    #[inline(always)]
    pub fn rng_mask(&self) -> RngMaskR {
        RngMaskR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The RNG SRAM to DIN DMA done interrupt mask."]
    #[inline(always)]
    pub fn sram_to_din_mask(&mut self) -> SramToDinMaskW<'_, ImrSpec> {
        SramToDinMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - The DOUT to RNG SRAM DMA done interrupt mask."]
    #[inline(always)]
    pub fn dout_to_sram_mask(&mut self) -> DoutToSramMaskW<'_, ImrSpec> {
        DoutToSramMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - The memory to DIN DMA done interrupt mask."]
    #[inline(always)]
    pub fn mem_to_din_mask(&mut self) -> MemToDinMaskW<'_, ImrSpec> {
        MemToDinMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - The DOUT to memory DMA done interrupt mask."]
    #[inline(always)]
    pub fn dout_to_mem_mask(&mut self) -> DoutToMemMaskW<'_, ImrSpec> {
        DoutToMemMaskW::new(self, 7)
    }
    #[doc = "Bit 8 - The AHB error interrupt mask."]
    #[inline(always)]
    pub fn ahb_err_mask(&mut self) -> AhbErrMaskW<'_, ImrSpec> {
        AhbErrMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - The PKA end of operation interrupt mask."]
    #[inline(always)]
    pub fn pka_mask(&mut self) -> PkaMaskW<'_, ImrSpec> {
        PkaMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - The RNG interrupt mask."]
    #[inline(always)]
    pub fn rng_mask(&mut self) -> RngMaskW<'_, ImrSpec> {
        RngMaskW::new(self, 10)
    }
}
#[doc = "Interrupt mask register. Each bit of this register holds the mask of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0x01ff_ffff"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
