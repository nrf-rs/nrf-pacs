#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Number of values (duty cycles) in this sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cnt {
    #[doc = "0: Sequence is disabled, and shall not be started as it is empty"]
    Disabled = 0,
}
impl From<Cnt> for u16 {
    #[inline(always)]
    fn from(variant: Cnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cnt {
    type Ux = u16;
}
impl crate::IsEnum for Cnt {}
#[doc = "Field `CNT` reader - Number of values (duty cycles) in this sequence"]
pub type CntR = crate::FieldReader<Cnt>;
impl CntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cnt> {
        match self.bits {
            0 => Some(Cnt::Disabled),
            _ => None,
        }
    }
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cnt::Disabled
    }
}
#[doc = "Field `CNT` writer - Number of values (duty cycles) in this sequence"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 15, Cnt>;
impl<'a, REG> CntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cnt::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:14 - Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Description cluster: Number of values (duty cycles) in this sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {
    const RESET_VALUE: u32 = 0;
}
