#[doc = "Register `STALLSTAT` reader"]
pub type R = crate::R<StallstatSpec>;
#[doc = "Register `STALLSTAT` writer"]
pub type W = crate::W<StallstatSpec>;
#[doc = "Stall status for EasyDMA RAM reads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: No stall"]
    Nostall = 0,
    #[doc = "1: A stall has occurred"]
    Stall = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Stall status for EasyDMA RAM reads"]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::Nostall,
            true => Tx::Stall,
        }
    }
    #[doc = "No stall"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        *self == Tx::Nostall
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Tx::Stall
    }
}
#[doc = "Field `TX` writer - Stall status for EasyDMA RAM reads"]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No stall"]
    #[inline(always)]
    pub fn nostall(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Nostall)
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::Stall)
    }
}
#[doc = "Stall status for EasyDMA RAM writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: No stall"]
    Nostall = 0,
    #[doc = "1: A stall has occurred"]
    Stall = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - Stall status for EasyDMA RAM writes"]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            false => Rx::Nostall,
            true => Rx::Stall,
        }
    }
    #[doc = "No stall"]
    #[inline(always)]
    pub fn is_nostall(&self) -> bool {
        *self == Rx::Nostall
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Rx::Stall
    }
}
#[doc = "Field `RX` writer - Stall status for EasyDMA RAM writes"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG, Rx>;
impl<'a, REG> RxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No stall"]
    #[inline(always)]
    pub fn nostall(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Nostall)
    }
    #[doc = "A stall has occurred"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(Rx::Stall)
    }
}
impl R {
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, StallstatSpec> {
        TxW::new(self, 0)
    }
    #[doc = "Bit 1 - Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, StallstatSpec> {
        RxW::new(self, 1)
    }
}
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`stallstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stallstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StallstatSpec;
impl crate::RegisterSpec for StallstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stallstat::R`](R) reader structure"]
impl crate::Readable for StallstatSpec {}
#[doc = "`write(|w| ..)` method takes [`stallstat::W`](W) writer structure"]
impl crate::Writable for StallstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STALLSTAT to value 0"]
impl crate::Resettable for StallstatSpec {}
