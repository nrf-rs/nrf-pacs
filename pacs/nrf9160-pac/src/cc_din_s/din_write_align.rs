#[doc = "Register `DIN_WRITE_ALIGN` writer"]
pub type W = crate::W<DinWriteAlignSpec>;
#[doc = "Next CPU write to the DIN_BUFFER is the last word.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Last {
    #[doc = "1: The next CPU write is the last in the sequence."]
    Confirm = 1,
}
impl From<Last> for bool {
    #[inline(always)]
    fn from(variant: Last) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST` writer - Next CPU write to the DIN_BUFFER is the last word."]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG, Last>;
impl<'a, REG> LastW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The next CPU write is the last in the sequence."]
    #[inline(always)]
    pub fn confirm(self) -> &'a mut crate::W<REG> {
        self.variant(Last::Confirm)
    }
}
impl W {
    #[doc = "Bit 0 - Next CPU write to the DIN_BUFFER is the last word."]
    #[inline(always)]
    pub fn last(&mut self) -> LastW<'_, DinWriteAlignSpec> {
        LastW::new(self, 0)
    }
}
#[doc = "Indicates that the next CPU write to the DIN_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_write_align::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinWriteAlignSpec;
impl crate::RegisterSpec for DinWriteAlignSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`din_write_align::W`](W) writer structure"]
impl crate::Writable for DinWriteAlignSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIN_WRITE_ALIGN to value 0"]
impl crate::Resettable for DinWriteAlignSpec {}
