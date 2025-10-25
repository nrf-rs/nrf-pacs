#[doc = "Register `CCACTRL` reader"]
pub type R = crate::R<CcactrlSpec>;
#[doc = "Register `CCACTRL` writer"]
pub type W = crate::W<CcactrlSpec>;
#[doc = "CCA mode of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccamode {
    #[doc = "0: Energy above threshold"]
    EdMode = 0,
    #[doc = "1: Carrier seen"]
    CarrierMode = 1,
    #[doc = "2: Energy above threshold AND carrier seen"]
    CarrierAndEdMode = 2,
    #[doc = "3: Energy above threshold OR carrier seen"]
    CarrierOrEdMode = 3,
    #[doc = "4: Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    EdModeTest1 = 4,
}
impl From<Ccamode> for u8 {
    #[inline(always)]
    fn from(variant: Ccamode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccamode {
    type Ux = u8;
}
impl crate::IsEnum for Ccamode {}
#[doc = "Field `CCAMODE` reader - CCA mode of operation"]
pub type CcamodeR = crate::FieldReader<Ccamode>;
impl CcamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccamode> {
        match self.bits {
            0 => Some(Ccamode::EdMode),
            1 => Some(Ccamode::CarrierMode),
            2 => Some(Ccamode::CarrierAndEdMode),
            3 => Some(Ccamode::CarrierOrEdMode),
            4 => Some(Ccamode::EdModeTest1),
            _ => None,
        }
    }
    #[doc = "Energy above threshold"]
    #[inline(always)]
    pub fn is_ed_mode(&self) -> bool {
        *self == Ccamode::EdMode
    }
    #[doc = "Carrier seen"]
    #[inline(always)]
    pub fn is_carrier_mode(&self) -> bool {
        *self == Ccamode::CarrierMode
    }
    #[doc = "Energy above threshold AND carrier seen"]
    #[inline(always)]
    pub fn is_carrier_and_ed_mode(&self) -> bool {
        *self == Ccamode::CarrierAndEdMode
    }
    #[doc = "Energy above threshold OR carrier seen"]
    #[inline(always)]
    pub fn is_carrier_or_ed_mode(&self) -> bool {
        *self == Ccamode::CarrierOrEdMode
    }
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    #[inline(always)]
    pub fn is_ed_mode_test1(&self) -> bool {
        *self == Ccamode::EdModeTest1
    }
}
#[doc = "Field `CCAMODE` writer - CCA mode of operation"]
pub type CcamodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccamode>;
impl<'a, REG> CcamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Energy above threshold"]
    #[inline(always)]
    pub fn ed_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Ccamode::EdMode)
    }
    #[doc = "Carrier seen"]
    #[inline(always)]
    pub fn carrier_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Ccamode::CarrierMode)
    }
    #[doc = "Energy above threshold AND carrier seen"]
    #[inline(always)]
    pub fn carrier_and_ed_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Ccamode::CarrierAndEdMode)
    }
    #[doc = "Energy above threshold OR carrier seen"]
    #[inline(always)]
    pub fn carrier_or_ed_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Ccamode::CarrierOrEdMode)
    }
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    #[inline(always)]
    pub fn ed_mode_test1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccamode::EdModeTest1)
    }
}
#[doc = "Field `CCAEDTHRES` reader - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
pub type CcaedthresR = crate::FieldReader;
#[doc = "Field `CCAEDTHRES` writer - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
pub type CcaedthresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CCACORRTHRES` reader - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
pub type CcacorrthresR = crate::FieldReader;
#[doc = "Field `CCACORRTHRES` writer - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
pub type CcacorrthresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CCACORRCNT` reader - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
pub type CcacorrcntR = crate::FieldReader;
#[doc = "Field `CCACORRCNT` writer - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
pub type CcacorrcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&self) -> CcamodeR {
        CcamodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&self) -> CcaedthresR {
        CcaedthresR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&self) -> CcacorrthresR {
        CcacorrthresR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&self) -> CcacorrcntR {
        CcacorrcntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CCA mode of operation"]
    #[inline(always)]
    pub fn ccamode(&mut self) -> CcamodeW<'_, CcactrlSpec> {
        CcamodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn ccaedthres(&mut self) -> CcaedthresW<'_, CcactrlSpec> {
        CcaedthresW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode, and CarrierOrEdMode."]
    #[inline(always)]
    pub fn ccacorrthres(&mut self) -> CcacorrthresW<'_, CcactrlSpec> {
        CcacorrthresW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn ccacorrcnt(&mut self) -> CcacorrcntW<'_, CcactrlSpec> {
        CcacorrcntW::new(self, 24)
    }
}
#[doc = "IEEE 802.15.4 clear channel assessment control\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcactrlSpec;
impl crate::RegisterSpec for CcactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccactrl::R`](R) reader structure"]
impl crate::Readable for CcactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ccactrl::W`](W) writer structure"]
impl crate::Writable for CcactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCACTRL to value 0x052d_0000"]
impl crate::Resettable for CcactrlSpec {
    const RESET_VALUE: u32 = 0x052d_0000;
}
