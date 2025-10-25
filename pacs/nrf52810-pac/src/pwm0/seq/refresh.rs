#[doc = "Register `REFRESH` reader"]
pub type R = crate::R<RefreshSpec>;
#[doc = "Register `REFRESH` writer"]
pub type W = crate::W<RefreshSpec>;
#[doc = "Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Cnt {
    #[doc = "0: Update every PWM period"]
    Continuous = 0,
}
impl From<Cnt> for u32 {
    #[inline(always)]
    fn from(variant: Cnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cnt {
    type Ux = u32;
}
impl crate::IsEnum for Cnt {}
#[doc = "Field `CNT` reader - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
pub type CntR = crate::FieldReader<Cnt>;
impl CntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cnt> {
        match self.bits {
            0 => Some(Cnt::Continuous),
            _ => None,
        }
    }
    #[doc = "Update every PWM period"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Cnt::Continuous
    }
}
#[doc = "Field `CNT` writer - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 24, Cnt>;
impl<'a, REG> CntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Update every PWM period"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Cnt::Continuous)
    }
}
impl R {
    #[doc = "Bits 0:23 - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, RefreshSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefreshSpec;
impl crate::RegisterSpec for RefreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refresh::R`](R) reader structure"]
impl crate::Readable for RefreshSpec {}
#[doc = "`write(|w| ..)` method takes [`refresh::W`](W) writer structure"]
impl crate::Writable for RefreshSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFRESH to value 0x01"]
impl crate::Resettable for RefreshSpec {
    const RESET_VALUE: u32 = 0x01;
}
