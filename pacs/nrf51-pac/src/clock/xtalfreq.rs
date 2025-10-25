#[doc = "Register `XTALFREQ` reader"]
pub type R = crate::R<XtalfreqSpec>;
#[doc = "Register `XTALFREQ` writer"]
pub type W = crate::W<XtalfreqSpec>;
#[doc = "External Xtal frequency selection.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xtalfreq {
    #[doc = "255: 16MHz xtal is used as source for the HFCLK oscillator."]
    _16mhz = 255,
    #[doc = "0: 32MHz xtal is used as source for the HFCLK oscillator."]
    _32mhz = 0,
}
impl From<Xtalfreq> for u8 {
    #[inline(always)]
    fn from(variant: Xtalfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xtalfreq {
    type Ux = u8;
}
impl crate::IsEnum for Xtalfreq {}
#[doc = "Field `XTALFREQ` reader - External Xtal frequency selection."]
pub type XtalfreqR = crate::FieldReader<Xtalfreq>;
impl XtalfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xtalfreq> {
        match self.bits {
            255 => Some(Xtalfreq::_16mhz),
            0 => Some(Xtalfreq::_32mhz),
            _ => None,
        }
    }
    #[doc = "16MHz xtal is used as source for the HFCLK oscillator."]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == Xtalfreq::_16mhz
    }
    #[doc = "32MHz xtal is used as source for the HFCLK oscillator."]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == Xtalfreq::_32mhz
    }
}
#[doc = "Field `XTALFREQ` writer - External Xtal frequency selection."]
pub type XtalfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8, Xtalfreq>;
impl<'a, REG> XtalfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16MHz xtal is used as source for the HFCLK oscillator."]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalfreq::_16mhz)
    }
    #[doc = "32MHz xtal is used as source for the HFCLK oscillator."]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalfreq::_32mhz)
    }
}
impl R {
    #[doc = "Bits 0:7 - External Xtal frequency selection."]
    #[inline(always)]
    pub fn xtalfreq(&self) -> XtalfreqR {
        XtalfreqR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Xtal frequency selection."]
    #[inline(always)]
    pub fn xtalfreq(&mut self) -> XtalfreqW<'_, XtalfreqSpec> {
        XtalfreqW::new(self, 0)
    }
}
#[doc = "Crystal frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalfreqSpec;
impl crate::RegisterSpec for XtalfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalfreq::R`](R) reader structure"]
impl crate::Readable for XtalfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalfreq::W`](W) writer structure"]
impl crate::Writable for XtalfreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALFREQ to value 0xffff_ffff"]
impl crate::Resettable for XtalfreqSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
