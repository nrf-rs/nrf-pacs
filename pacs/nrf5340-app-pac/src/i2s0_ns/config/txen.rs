#[doc = "Register `TXEN` reader"]
pub type R = crate::R<TxenSpec>;
#[doc = "Register `TXEN` writer"]
pub type W = crate::W<TxenSpec>;
#[doc = "Transmission (TX) enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txen {
    #[doc = "0: Transmission disabled and now data will be read from the RXD.TXD address."]
    Disabled = 0,
    #[doc = "1: Transmission enabled."]
    Enabled = 1,
}
impl From<Txen> for bool {
    #[inline(always)]
    fn from(variant: Txen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEN` reader - Transmission (TX) enable"]
pub type TxenR = crate::BitReader<Txen>;
impl TxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txen {
        match self.bits {
            false => Txen::Disabled,
            true => Txen::Enabled,
        }
    }
    #[doc = "Transmission disabled and now data will be read from the RXD.TXD address."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txen::Disabled
    }
    #[doc = "Transmission enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txen::Enabled
    }
}
#[doc = "Field `TXEN` writer - Transmission (TX) enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG, Txen>;
impl<'a, REG> TxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission disabled and now data will be read from the RXD.TXD address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txen::Disabled)
    }
    #[doc = "Transmission enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission (TX) enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission (TX) enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, TxenSpec> {
        TxenW::new(self, 0)
    }
}
#[doc = "Transmission (TX) enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxenSpec;
impl crate::RegisterSpec for TxenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txen::R`](R) reader structure"]
impl crate::Readable for TxenSpec {}
#[doc = "`write(|w| ..)` method takes [`txen::W`](W) writer structure"]
impl crate::Writable for TxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXEN to value 0x01"]
impl crate::Resettable for TxenSpec {
    const RESET_VALUE: u32 = 0x01;
}
