#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Mono or stereo operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    #[doc = "0: Sample and store one pair (left + right) of 16-bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    Stereo = 0,
    #[doc = "1: Sample and store two successive left samples (16 bits each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    Mono = 1,
}
impl From<Operation> for bool {
    #[inline(always)]
    fn from(variant: Operation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERATION` reader - Mono or stereo operation"]
pub type OperationR = crate::BitReader<Operation>;
impl OperationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Operation {
        match self.bits {
            false => Operation::Stereo,
            true => Operation::Mono,
        }
    }
    #[doc = "Sample and store one pair (left + right) of 16-bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == Operation::Stereo
    }
    #[doc = "Sample and store two successive left samples (16 bits each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == Operation::Mono
    }
}
#[doc = "Field `OPERATION` writer - Mono or stereo operation"]
pub type OperationW<'a, REG> = crate::BitWriter<'a, REG, Operation>;
impl<'a, REG> OperationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample and store one pair (left + right) of 16-bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(Operation::Stereo)
    }
    #[doc = "Sample and store two successive left samples (16 bits each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(Operation::Mono)
    }
}
#[doc = "Defines on which PDM_CLK edge left (or mono) is sampled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "0: Left (or mono) is sampled on falling edge of PDM_CLK"]
    LeftFalling = 0,
    #[doc = "1: Left (or mono) is sampled on rising edge of PDM_CLK"]
    LeftRising = 1,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - Defines on which PDM_CLK edge left (or mono) is sampled"]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            false => Edge::LeftFalling,
            true => Edge::LeftRising,
        }
    }
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    #[inline(always)]
    pub fn is_left_falling(&self) -> bool {
        *self == Edge::LeftFalling
    }
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    #[inline(always)]
    pub fn is_left_rising(&self) -> bool {
        *self == Edge::LeftRising
    }
}
#[doc = "Field `EDGE` writer - Defines on which PDM_CLK edge left (or mono) is sampled"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    #[inline(always)]
    pub fn left_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::LeftFalling)
    }
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    #[inline(always)]
    pub fn left_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::LeftRising)
    }
}
impl R {
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline(always)]
    pub fn operation(&self) -> OperationR {
        OperationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge left (or mono) is sampled"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline(always)]
    #[must_use]
    pub fn operation(&mut self) -> OperationW<ModeSpec> {
        OperationW::new(self, 0)
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge left (or mono) is sampled"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EdgeW<ModeSpec> {
        EdgeW::new(self, 1)
    }
}
#[doc = "Defines the routing of the connected PDM microphones' signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0;
}
