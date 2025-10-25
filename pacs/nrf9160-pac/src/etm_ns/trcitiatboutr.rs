#[doc = "Register `TRCITIATBOUTR` reader"]
pub type R = crate::R<TrcitiatboutrSpec>;
#[doc = "Register `TRCITIATBOUTR` writer"]
pub type W = crate::W<TrcitiatboutrSpec>;
#[doc = "Field `ATVALID` reader - Drives the ATVALIDMI output pin."]
pub type AtvalidR = crate::BitReader;
#[doc = "Field `ATVALID` writer - Drives the ATVALIDMI output pin."]
pub type AtvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADY` reader - Drives the AFREADYMI output pin."]
pub type AfreadyR = crate::BitReader;
#[doc = "Field `AFREADY` writer - Drives the AFREADYMI output pin."]
pub type AfreadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drives the ATVALIDMI output pin."]
    #[inline(always)]
    pub fn atvalid(&self) -> AtvalidR {
        AtvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drives the AFREADYMI output pin."]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drives the ATVALIDMI output pin."]
    #[inline(always)]
    pub fn atvalid(&mut self) -> AtvalidW<'_, TrcitiatboutrSpec> {
        AtvalidW::new(self, 0)
    }
    #[doc = "Bit 1 - Drives the AFREADYMI output pin."]
    #[inline(always)]
    pub fn afready(&mut self) -> AfreadyW<'_, TrcitiatboutrSpec> {
        AfreadyW::new(self, 1)
    }
}
#[doc = "Sets the state of the output pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitiatboutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatboutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcitiatboutrSpec;
impl crate::RegisterSpec for TrcitiatboutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitiatboutr::R`](R) reader structure"]
impl crate::Readable for TrcitiatboutrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcitiatboutr::W`](W) writer structure"]
impl crate::Writable for TrcitiatboutrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCITIATBOUTR to value 0"]
impl crate::Resettable for TrcitiatboutrSpec {}
