#[doc = "Register `STARTN` reader"]
pub type R = crate::R<StartnSpec>;
#[doc = "Register `STARTN` writer"]
pub type W = crate::W<StartnSpec>;
#[doc = "Start LTE modem\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startn {
    #[doc = "0: Start LTE modem"]
    Start = 0,
    #[doc = "1: Hold LTE modem disabled"]
    Hold = 1,
}
impl From<Startn> for bool {
    #[inline(always)]
    fn from(variant: Startn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTN` reader - Start LTE modem"]
pub type StartnR = crate::BitReader<Startn>;
impl StartnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startn {
        match self.bits {
            false => Startn::Start,
            true => Startn::Hold,
        }
    }
    #[doc = "Start LTE modem"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Startn::Start
    }
    #[doc = "Hold LTE modem disabled"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == Startn::Hold
    }
}
#[doc = "Field `STARTN` writer - Start LTE modem"]
pub type StartnW<'a, REG> = crate::BitWriter<'a, REG, Startn>;
impl<'a, REG> StartnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start LTE modem"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Startn::Start)
    }
    #[doc = "Hold LTE modem disabled"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(Startn::Hold)
    }
}
impl R {
    #[doc = "Bit 0 - Start LTE modem"]
    #[inline(always)]
    pub fn startn(&self) -> StartnR {
        StartnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start LTE modem"]
    #[inline(always)]
    #[must_use]
    pub fn startn(&mut self) -> StartnW<StartnSpec> {
        StartnW::new(self, 0)
    }
}
#[doc = "Start LTE modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartnSpec;
impl crate::RegisterSpec for StartnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startn::R`](R) reader structure"]
impl crate::Readable for StartnSpec {}
#[doc = "`write(|w| ..)` method takes [`startn::W`](W) writer structure"]
impl crate::Writable for StartnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTN to value 0x01"]
impl crate::Resettable for StartnSpec {
    const RESET_VALUE: u32 = 0x01;
}
