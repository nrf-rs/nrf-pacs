#[doc = "Register `TRCCNTCTLR[%s]` reader"]
pub type R = crate::R<TrccntctlrSpec>;
#[doc = "Register `TRCCNTCTLR[%s]` writer"]
pub type W = crate::W<TrccntctlrSpec>;
#[doc = "Field `CNTEVENT` reader - Selects an event, that when it occurs causes counter n to decrement."]
pub type CnteventR = crate::FieldReader;
#[doc = "Field `CNTEVENT` writer - Selects an event, that when it occurs causes counter n to decrement."]
pub type CnteventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLDEVENT` reader - Selects an event, that when it occurs causes a reload event for counter n."]
pub type RldeventR = crate::FieldReader;
#[doc = "Field `RLDEVENT` writer - Selects an event, that when it occurs causes a reload event for counter n."]
pub type RldeventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Controls whether a reload event occurs for counter n, when counter n reaches zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rldself {
    #[doc = "0: The counter is in Normal mode."]
    Disabled = 0,
    #[doc = "1: The counter is in Self-reload mode."]
    Enabled = 1,
}
impl From<Rldself> for bool {
    #[inline(always)]
    fn from(variant: Rldself) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDSELF` reader - Controls whether a reload event occurs for counter n, when counter n reaches zero."]
pub type RldselfR = crate::BitReader<Rldself>;
impl RldselfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rldself {
        match self.bits {
            false => Rldself::Disabled,
            true => Rldself::Enabled,
        }
    }
    #[doc = "The counter is in Normal mode."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rldself::Disabled
    }
    #[doc = "The counter is in Self-reload mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rldself::Enabled
    }
}
#[doc = "Field `RLDSELF` writer - Controls whether a reload event occurs for counter n, when counter n reaches zero."]
pub type RldselfW<'a, REG> = crate::BitWriter<'a, REG, Rldself>;
impl<'a, REG> RldselfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter is in Normal mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rldself::Disabled)
    }
    #[doc = "The counter is in Self-reload mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rldself::Enabled)
    }
}
#[doc = "For TRCCNTCTLR3 and TRCCNTCTLR1, this bit controls whether counter n decrements when a reload event occurs for counter n-1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntchain {
    #[doc = "0: Counter n does not decrement when a reload event for counter n-1 occurs."]
    Disabled = 0,
    #[doc = "1: Counter n decrements when a reload event for counter n-1 occurs. This concatenates counter n and counter n-1, to provide a larger count value."]
    Enabled = 1,
}
impl From<Cntchain> for bool {
    #[inline(always)]
    fn from(variant: Cntchain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTCHAIN` reader - For TRCCNTCTLR3 and TRCCNTCTLR1, this bit controls whether counter n decrements when a reload event occurs for counter n-1."]
pub type CntchainR = crate::BitReader<Cntchain>;
impl CntchainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntchain {
        match self.bits {
            false => Cntchain::Disabled,
            true => Cntchain::Enabled,
        }
    }
    #[doc = "Counter n does not decrement when a reload event for counter n-1 occurs."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cntchain::Disabled
    }
    #[doc = "Counter n decrements when a reload event for counter n-1 occurs. This concatenates counter n and counter n-1, to provide a larger count value."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cntchain::Enabled
    }
}
#[doc = "Field `CNTCHAIN` writer - For TRCCNTCTLR3 and TRCCNTCTLR1, this bit controls whether counter n decrements when a reload event occurs for counter n-1."]
pub type CntchainW<'a, REG> = crate::BitWriter<'a, REG, Cntchain>;
impl<'a, REG> CntchainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter n does not decrement when a reload event for counter n-1 occurs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cntchain::Disabled)
    }
    #[doc = "Counter n decrements when a reload event for counter n-1 occurs. This concatenates counter n and counter n-1, to provide a larger count value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cntchain::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects an event, that when it occurs causes counter n to decrement."]
    #[inline(always)]
    pub fn cntevent(&self) -> CnteventR {
        CnteventR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Selects an event, that when it occurs causes a reload event for counter n."]
    #[inline(always)]
    pub fn rldevent(&self) -> RldeventR {
        RldeventR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Controls whether a reload event occurs for counter n, when counter n reaches zero."]
    #[inline(always)]
    pub fn rldself(&self) -> RldselfR {
        RldselfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - For TRCCNTCTLR3 and TRCCNTCTLR1, this bit controls whether counter n decrements when a reload event occurs for counter n-1."]
    #[inline(always)]
    pub fn cntchain(&self) -> CntchainR {
        CntchainR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects an event, that when it occurs causes counter n to decrement."]
    #[inline(always)]
    pub fn cntevent(&mut self) -> CnteventW<'_, TrccntctlrSpec> {
        CnteventW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Selects an event, that when it occurs causes a reload event for counter n."]
    #[inline(always)]
    pub fn rldevent(&mut self) -> RldeventW<'_, TrccntctlrSpec> {
        RldeventW::new(self, 8)
    }
    #[doc = "Bit 16 - Controls whether a reload event occurs for counter n, when counter n reaches zero."]
    #[inline(always)]
    pub fn rldself(&mut self) -> RldselfW<'_, TrccntctlrSpec> {
        RldselfW::new(self, 16)
    }
    #[doc = "Bit 17 - For TRCCNTCTLR3 and TRCCNTCTLR1, this bit controls whether counter n decrements when a reload event occurs for counter n-1."]
    #[inline(always)]
    pub fn cntchain(&mut self) -> CntchainW<'_, TrccntctlrSpec> {
        CntchainW::new(self, 17)
    }
}
#[doc = "Description collection: Controls the operation of counter n. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccntctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrccntctlrSpec;
impl crate::RegisterSpec for TrccntctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccntctlr::R`](R) reader structure"]
impl crate::Readable for TrccntctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trccntctlr::W`](W) writer structure"]
impl crate::Writable for TrccntctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCNTCTLR[%s] to value 0"]
impl crate::Resettable for TrccntctlrSpec {}
