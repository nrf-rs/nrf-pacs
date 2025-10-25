#[doc = "Register `LOOP` reader"]
pub type R = crate::R<LoopSpec>;
#[doc = "Register `LOOP` writer"]
pub type W = crate::W<LoopSpec>;
#[doc = "Number of playbacks of pattern cycles\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cnt {
    #[doc = "0: Looping disabled (stop at the end of the sequence)"]
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
#[doc = "Field `CNT` reader - Number of playbacks of pattern cycles"]
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
    #[doc = "Looping disabled (stop at the end of the sequence)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cnt::Disabled
    }
}
#[doc = "Field `CNT` writer - Number of playbacks of pattern cycles"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, Cnt>;
impl<'a, REG> CntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Looping disabled (stop at the end of the sequence)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cnt::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of playbacks of pattern cycles"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of playbacks of pattern cycles"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, LoopSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Number of playbacks of a loop\n\nYou can [`read`](crate::Reg::read) this register and get [`loop_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoopSpec;
impl crate::RegisterSpec for LoopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loop_::R`](R) reader structure"]
impl crate::Readable for LoopSpec {}
#[doc = "`write(|w| ..)` method takes [`loop_::W`](W) writer structure"]
impl crate::Writable for LoopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LoopSpec {}
