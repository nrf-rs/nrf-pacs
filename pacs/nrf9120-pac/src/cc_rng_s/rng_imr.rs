#[doc = "Register `RNG_IMR` reader"]
pub type R = crate::R<RngImrSpec>;
#[doc = "Register `RNG_IMR` writer"]
pub type W = crate::W<RngImrSpec>;
#[doc = "See RNG_ISR for explanation on this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhrValidMask {
    #[doc = "0: Do not mask EHR interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask EHR interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<EhrValidMask> for bool {
    #[inline(always)]
    fn from(variant: EhrValidMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHR_VALID_MASK` reader - See RNG_ISR for explanation on this interrupt."]
pub type EhrValidMaskR = crate::BitReader<EhrValidMask>;
impl EhrValidMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhrValidMask {
        match self.bits {
            false => EhrValidMask::Irqenable,
            true => EhrValidMask::Irqdisable,
        }
    }
    #[doc = "Do not mask EHR interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == EhrValidMask::Irqenable
    }
    #[doc = "Mask EHR interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == EhrValidMask::Irqdisable
    }
}
#[doc = "Field `EHR_VALID_MASK` writer - See RNG_ISR for explanation on this interrupt."]
pub type EhrValidMaskW<'a, REG> = crate::BitWriter<'a, REG, EhrValidMask>;
impl<'a, REG> EhrValidMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask EHR interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(EhrValidMask::Irqenable)
    }
    #[doc = "Mask EHR interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(EhrValidMask::Irqdisable)
    }
}
#[doc = "See RNG_ISR for explanation on this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocorrErrMask {
    #[doc = "0: Do not mask autocorrelation interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask autocorrelation interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<AutocorrErrMask> for bool {
    #[inline(always)]
    fn from(variant: AutocorrErrMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCORR_ERR_MASK` reader - See RNG_ISR for explanation on this interrupt."]
pub type AutocorrErrMaskR = crate::BitReader<AutocorrErrMask>;
impl AutocorrErrMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutocorrErrMask {
        match self.bits {
            false => AutocorrErrMask::Irqenable,
            true => AutocorrErrMask::Irqdisable,
        }
    }
    #[doc = "Do not mask autocorrelation interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == AutocorrErrMask::Irqenable
    }
    #[doc = "Mask autocorrelation interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == AutocorrErrMask::Irqdisable
    }
}
#[doc = "Field `AUTOCORR_ERR_MASK` writer - See RNG_ISR for explanation on this interrupt."]
pub type AutocorrErrMaskW<'a, REG> = crate::BitWriter<'a, REG, AutocorrErrMask>;
impl<'a, REG> AutocorrErrMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask autocorrelation interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(AutocorrErrMask::Irqenable)
    }
    #[doc = "Mask autocorrelation interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(AutocorrErrMask::Irqdisable)
    }
}
#[doc = "See RNG_ISR for explanation on this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrngtErrMask {
    #[doc = "0: Do not mask the CRNGT error interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask the CRNGT error interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<CrngtErrMask> for bool {
    #[inline(always)]
    fn from(variant: CrngtErrMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRNGT_ERR_MASK` reader - See RNG_ISR for explanation on this interrupt."]
pub type CrngtErrMaskR = crate::BitReader<CrngtErrMask>;
impl CrngtErrMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrngtErrMask {
        match self.bits {
            false => CrngtErrMask::Irqenable,
            true => CrngtErrMask::Irqdisable,
        }
    }
    #[doc = "Do not mask the CRNGT error interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == CrngtErrMask::Irqenable
    }
    #[doc = "Mask the CRNGT error interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == CrngtErrMask::Irqdisable
    }
}
#[doc = "Field `CRNGT_ERR_MASK` writer - See RNG_ISR for explanation on this interrupt."]
pub type CrngtErrMaskW<'a, REG> = crate::BitWriter<'a, REG, CrngtErrMask>;
impl<'a, REG> CrngtErrMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask the CRNGT error interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(CrngtErrMask::Irqenable)
    }
    #[doc = "Mask the CRNGT error interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(CrngtErrMask::Irqdisable)
    }
}
#[doc = "See RNG_ISR for explanation on this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VncErrMask {
    #[doc = "0: Do not mask the von Neumann corrector error interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask the von Neumann corrector error interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<VncErrMask> for bool {
    #[inline(always)]
    fn from(variant: VncErrMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VNC_ERR_MASK` reader - See RNG_ISR for explanation on this interrupt."]
pub type VncErrMaskR = crate::BitReader<VncErrMask>;
impl VncErrMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VncErrMask {
        match self.bits {
            false => VncErrMask::Irqenable,
            true => VncErrMask::Irqdisable,
        }
    }
    #[doc = "Do not mask the von Neumann corrector error interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == VncErrMask::Irqenable
    }
    #[doc = "Mask the von Neumann corrector error interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == VncErrMask::Irqdisable
    }
}
#[doc = "Field `VNC_ERR_MASK` writer - See RNG_ISR for explanation on this interrupt."]
pub type VncErrMaskW<'a, REG> = crate::BitWriter<'a, REG, VncErrMask>;
impl<'a, REG> VncErrMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask the von Neumann corrector error interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(VncErrMask::Irqenable)
    }
    #[doc = "Mask the von Neumann corrector error interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(VncErrMask::Irqdisable)
    }
}
#[doc = "See RNG_ISR for explanation on this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchdogMask {
    #[doc = "0: Do not mask the watchdog interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask the watchdog interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<WatchdogMask> for bool {
    #[inline(always)]
    fn from(variant: WatchdogMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WATCHDOG_MASK` reader - See RNG_ISR for explanation on this interrupt."]
pub type WatchdogMaskR = crate::BitReader<WatchdogMask>;
impl WatchdogMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WatchdogMask {
        match self.bits {
            false => WatchdogMask::Irqenable,
            true => WatchdogMask::Irqdisable,
        }
    }
    #[doc = "Do not mask the watchdog interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == WatchdogMask::Irqenable
    }
    #[doc = "Mask the watchdog interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == WatchdogMask::Irqdisable
    }
}
#[doc = "Field `WATCHDOG_MASK` writer - See RNG_ISR for explanation on this interrupt."]
pub type WatchdogMaskW<'a, REG> = crate::BitWriter<'a, REG, WatchdogMask>;
impl<'a, REG> WatchdogMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask the watchdog interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(WatchdogMask::Irqenable)
    }
    #[doc = "Mask the watchdog interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(WatchdogMask::Irqdisable)
    }
}
#[doc = "See RNG_ISR for explanation on this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDoneMask {
    #[doc = "0: Do not mask the RNG DMA completion interrupt i.e. interrupt is generated"]
    Irqenable = 0,
    #[doc = "1: Mask the RNG DMA completion interrupt i.e. no interrupt is generated"]
    Irqdisable = 1,
}
impl From<DmaDoneMask> for bool {
    #[inline(always)]
    fn from(variant: DmaDoneMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_DONE_MASK` reader - See RNG_ISR for explanation on this interrupt."]
pub type DmaDoneMaskR = crate::BitReader<DmaDoneMask>;
impl DmaDoneMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaDoneMask {
        match self.bits {
            false => DmaDoneMask::Irqenable,
            true => DmaDoneMask::Irqdisable,
        }
    }
    #[doc = "Do not mask the RNG DMA completion interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn is_irqenable(&self) -> bool {
        *self == DmaDoneMask::Irqenable
    }
    #[doc = "Mask the RNG DMA completion interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn is_irqdisable(&self) -> bool {
        *self == DmaDoneMask::Irqdisable
    }
}
#[doc = "Field `DMA_DONE_MASK` writer - See RNG_ISR for explanation on this interrupt."]
pub type DmaDoneMaskW<'a, REG> = crate::BitWriter<'a, REG, DmaDoneMask>;
impl<'a, REG> DmaDoneMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not mask the RNG DMA completion interrupt i.e. interrupt is generated"]
    #[inline(always)]
    pub fn irqenable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneMask::Irqenable)
    }
    #[doc = "Mask the RNG DMA completion interrupt i.e. no interrupt is generated"]
    #[inline(always)]
    pub fn irqdisable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaDoneMask::Irqdisable)
    }
}
impl R {
    #[doc = "Bit 0 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn ehr_valid_mask(&self) -> EhrValidMaskR {
        EhrValidMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn autocorr_err_mask(&self) -> AutocorrErrMaskR {
        AutocorrErrMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn crngt_err_mask(&self) -> CrngtErrMaskR {
        CrngtErrMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn vnc_err_mask(&self) -> VncErrMaskR {
        VncErrMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn watchdog_mask(&self) -> WatchdogMaskR {
        WatchdogMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn dma_done_mask(&self) -> DmaDoneMaskR {
        DmaDoneMaskR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn ehr_valid_mask(&mut self) -> EhrValidMaskW<'_, RngImrSpec> {
        EhrValidMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn autocorr_err_mask(&mut self) -> AutocorrErrMaskW<'_, RngImrSpec> {
        AutocorrErrMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn crngt_err_mask(&mut self) -> CrngtErrMaskW<'_, RngImrSpec> {
        CrngtErrMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn vnc_err_mask(&mut self) -> VncErrMaskW<'_, RngImrSpec> {
        VncErrMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn watchdog_mask(&mut self) -> WatchdogMaskW<'_, RngImrSpec> {
        WatchdogMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - See RNG_ISR for explanation on this interrupt."]
    #[inline(always)]
    pub fn dma_done_mask(&mut self) -> DmaDoneMaskW<'_, RngImrSpec> {
        DmaDoneMaskW::new(self, 5)
    }
}
#[doc = "Interrupt mask register. Each bit of this register holds the mask of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngImrSpec;
impl crate::RegisterSpec for RngImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_imr::R`](R) reader structure"]
impl crate::Readable for RngImrSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_imr::W`](W) writer structure"]
impl crate::Writable for RngImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_IMR to value 0x3f"]
impl crate::Resettable for RngImrSpec {
    const RESET_VALUE: u32 = 0x3f;
}
