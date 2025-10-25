#[doc = "Register `ISOSPLIT` reader"]
pub type R = crate::R<IsosplitSpec>;
#[doc = "Register `ISOSPLIT` writer"]
pub type W = crate::W<IsosplitSpec>;
#[doc = "Controls the split of ISO buffers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Split {
    #[doc = "0: Full buffer dedicated to either ISO IN or OUT"]
    OneDir = 0,
    #[doc = "128: Lower half for IN, upper half for OUT"]
    HalfIn = 128,
}
impl From<Split> for u16 {
    #[inline(always)]
    fn from(variant: Split) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Split {
    type Ux = u16;
}
impl crate::IsEnum for Split {}
#[doc = "Field `SPLIT` reader - Controls the split of ISO buffers"]
pub type SplitR = crate::FieldReader<Split>;
impl SplitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Split> {
        match self.bits {
            0 => Some(Split::OneDir),
            128 => Some(Split::HalfIn),
            _ => None,
        }
    }
    #[doc = "Full buffer dedicated to either ISO IN or OUT"]
    #[inline(always)]
    pub fn is_one_dir(&self) -> bool {
        *self == Split::OneDir
    }
    #[doc = "Lower half for IN, upper half for OUT"]
    #[inline(always)]
    pub fn is_half_in(&self) -> bool {
        *self == Split::HalfIn
    }
}
#[doc = "Field `SPLIT` writer - Controls the split of ISO buffers"]
pub type SplitW<'a, REG> = crate::FieldWriter<'a, REG, 16, Split>;
impl<'a, REG> SplitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Full buffer dedicated to either ISO IN or OUT"]
    #[inline(always)]
    pub fn one_dir(self) -> &'a mut crate::W<REG> {
        self.variant(Split::OneDir)
    }
    #[doc = "Lower half for IN, upper half for OUT"]
    #[inline(always)]
    pub fn half_in(self) -> &'a mut crate::W<REG> {
        self.variant(Split::HalfIn)
    }
}
impl R {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&self) -> SplitR {
        SplitR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&mut self) -> SplitW<'_, IsosplitSpec> {
        SplitW::new(self, 0)
    }
}
#[doc = "Controls the split of ISO buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`isosplit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isosplit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsosplitSpec;
impl crate::RegisterSpec for IsosplitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isosplit::R`](R) reader structure"]
impl crate::Readable for IsosplitSpec {}
#[doc = "`write(|w| ..)` method takes [`isosplit::W`](W) writer structure"]
impl crate::Writable for IsosplitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISOSPLIT to value 0"]
impl crate::Resettable for IsosplitSpec {}
