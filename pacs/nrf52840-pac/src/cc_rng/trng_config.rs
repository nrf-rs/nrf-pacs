#[doc = "Register `TRNG_CONFIG` reader"]
pub type R = crate::R<TrngConfigSpec>;
#[doc = "Register `TRNG_CONFIG` writer"]
pub type W = crate::W<TrngConfigSpec>;
#[doc = "Set the length of the oscillator ring (= the number of inverters) out of four possible configurations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RoscLen {
    #[doc = "0: Use shortest ROSC1 ring oscillator configuration."]
    Rosc1 = 0,
    #[doc = "1: Use ROSC2 ring oscillator configuration."]
    Rosc2 = 1,
    #[doc = "2: Use ROSC3 ring oscillator configuration."]
    Rosc3 = 2,
    #[doc = "3: Use longest ROSC4 ring oscillator configuration."]
    Rosc4 = 3,
}
impl From<RoscLen> for u8 {
    #[inline(always)]
    fn from(variant: RoscLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RoscLen {
    type Ux = u8;
}
impl crate::IsEnum for RoscLen {}
#[doc = "Field `ROSC_LEN` reader - Set the length of the oscillator ring (= the number of inverters) out of four possible configurations."]
pub type RoscLenR = crate::FieldReader<RoscLen>;
impl RoscLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RoscLen {
        match self.bits {
            0 => RoscLen::Rosc1,
            1 => RoscLen::Rosc2,
            2 => RoscLen::Rosc3,
            3 => RoscLen::Rosc4,
            _ => unreachable!(),
        }
    }
    #[doc = "Use shortest ROSC1 ring oscillator configuration."]
    #[inline(always)]
    pub fn is_rosc1(&self) -> bool {
        *self == RoscLen::Rosc1
    }
    #[doc = "Use ROSC2 ring oscillator configuration."]
    #[inline(always)]
    pub fn is_rosc2(&self) -> bool {
        *self == RoscLen::Rosc2
    }
    #[doc = "Use ROSC3 ring oscillator configuration."]
    #[inline(always)]
    pub fn is_rosc3(&self) -> bool {
        *self == RoscLen::Rosc3
    }
    #[doc = "Use longest ROSC4 ring oscillator configuration."]
    #[inline(always)]
    pub fn is_rosc4(&self) -> bool {
        *self == RoscLen::Rosc4
    }
}
#[doc = "Field `ROSC_LEN` writer - Set the length of the oscillator ring (= the number of inverters) out of four possible configurations."]
pub type RoscLenW<'a, REG> = crate::FieldWriter<'a, REG, 2, RoscLen, crate::Safe>;
impl<'a, REG> RoscLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use shortest ROSC1 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc1(self) -> &'a mut crate::W<REG> {
        self.variant(RoscLen::Rosc1)
    }
    #[doc = "Use ROSC2 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc2(self) -> &'a mut crate::W<REG> {
        self.variant(RoscLen::Rosc2)
    }
    #[doc = "Use ROSC3 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc3(self) -> &'a mut crate::W<REG> {
        self.variant(RoscLen::Rosc3)
    }
    #[doc = "Use longest ROSC4 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc4(self) -> &'a mut crate::W<REG> {
        self.variant(RoscLen::Rosc4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Set the length of the oscillator ring (= the number of inverters) out of four possible configurations."]
    #[inline(always)]
    pub fn rosc_len(&self) -> RoscLenR {
        RoscLenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set the length of the oscillator ring (= the number of inverters) out of four possible configurations."]
    #[inline(always)]
    pub fn rosc_len(&mut self) -> RoscLenW<'_, TrngConfigSpec> {
        RoscLenW::new(self, 0)
    }
}
#[doc = "TRNG ring oscillator length configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngConfigSpec;
impl crate::RegisterSpec for TrngConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_config::R`](R) reader structure"]
impl crate::Readable for TrngConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`trng_config::W`](W) writer structure"]
impl crate::Writable for TrngConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRNG_CONFIG to value 0"]
impl crate::Resettable for TrngConfigSpec {}
