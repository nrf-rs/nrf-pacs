#[doc = "Register `RXEN` reader"]
pub type R = crate::R<RxenSpec>;
#[doc = "Register `RXEN` writer"]
pub type W = crate::W<RxenSpec>;
#[doc = "Reception (RX) enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxen {
    #[doc = "0: Reception disabled and now data will be written to the RXD.PTR address."]
    Disabled = 0,
    #[doc = "1: Reception enabled."]
    Enabled = 1,
}
impl From<Rxen> for bool {
    #[inline(always)]
    fn from(variant: Rxen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEN` reader - Reception (RX) enable"]
pub type RxenR = crate::BitReader<Rxen>;
impl RxenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxen {
        match self.bits {
            false => Rxen::Disabled,
            true => Rxen::Enabled,
        }
    }
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxen::Disabled
    }
    #[doc = "Reception enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxen::Enabled
    }
}
#[doc = "Field `RXEN` writer - Reception (RX) enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG, Rxen>;
impl<'a, REG> RxenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxen::Disabled)
    }
    #[doc = "Reception enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Reception (RX) enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception (RX) enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, RxenSpec> {
        RxenW::new(self, 0)
    }
}
#[doc = "Reception (RX) enable\n\nYou can [`read`](crate::Reg::read) this register and get [`rxen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxenSpec;
impl crate::RegisterSpec for RxenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxen::R`](R) reader structure"]
impl crate::Readable for RxenSpec {}
#[doc = "`write(|w| ..)` method takes [`rxen::W`](W) writer structure"]
impl crate::Writable for RxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXEN to value 0"]
impl crate::Resettable for RxenSpec {}
