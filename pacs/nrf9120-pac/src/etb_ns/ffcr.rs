#[doc = "Register `FFCR` reader"]
pub type R = crate::R<FfcrSpec>;
#[doc = "Register `FFCR` writer"]
pub type W = crate::W<FfcrSpec>;
#[doc = "Field `ENFTC` reader - Do not embed Triggers into the formatted stream. Trace disable cycles and triggers are indicated by TRACECTL, where fitted. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
pub type EnftcR = crate::BitReader;
#[doc = "Field `ENFTC` writer - Do not embed Triggers into the formatted stream. Trace disable cycles and triggers are indicated by TRACECTL, where fitted. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
pub type EnftcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENFCONT` reader - Continuous mode in the ETB corresponds to normal mode with the embedding of triggers. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
pub type EnfcontR = crate::BitReader;
#[doc = "Field `ENFCONT` writer - Continuous mode in the ETB corresponds to normal mode with the embedding of triggers. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
pub type EnfcontW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FONFLIN` reader - Set this bit to enable use of the flushin connection. This is clear on reset."]
pub type FonflinR = crate::BitReader;
#[doc = "Field `FONFLIN` writer - Set this bit to enable use of the flushin connection. This is clear on reset."]
pub type FonflinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FONTRIG` reader - Generate flush using Trigger event. Set this bit to cause a flush of data in the system when a Trigger Event occurs. This bit is clear on reset. A Trigger Event is defined as when the Trigger counter reaches zero (where fitted) or, in the case of the trigger counter being zero (or not fitted), when trigin is HIGH."]
pub type FontrigR = crate::BitReader;
#[doc = "Field `FONTRIG` writer - Generate flush using Trigger event. Set this bit to cause a flush of data in the system when a Trigger Event occurs. This bit is clear on reset. A Trigger Event is defined as when the Trigger counter reaches zero (where fitted) or, in the case of the trigger counter being zero (or not fitted), when trigin is HIGH."]
pub type FontrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FONMAN` reader - Setting this bit causes a flush to be generated. This is cleared when this flush has been serviced. This bit is clear on reset."]
pub type FonmanR = crate::BitReader;
#[doc = "Field `FONMAN` writer - Setting this bit causes a flush to be generated. This is cleared when this flush has been serviced. This bit is clear on reset."]
pub type FonmanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGIN` reader - Indicate a trigger on trigin being asserted."]
pub type TriginR = crate::BitReader;
#[doc = "Field `TRIGIN` writer - Indicate a trigger on trigin being asserted."]
pub type TriginW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGEVT` reader - Indicate a trigger on a Trigger Event."]
pub type TrigevtR = crate::BitReader;
#[doc = "Field `TRIGEVT` writer - Indicate a trigger on a Trigger Event."]
pub type TrigevtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGFL` reader - Indicates a trigger on Flush completion (afreadys being returned)."]
pub type TrigflR = crate::BitReader;
#[doc = "Field `TRIGFL` writer - Indicates a trigger on Flush completion (afreadys being returned)."]
pub type TrigflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPFL` reader - This forces the FIFO to drain off any part-completed packets. Setting this bit enables this function but this is clear on reset (disabled)."]
pub type StopflR = crate::BitReader;
#[doc = "Field `STOPFL` writer - This forces the FIFO to drain off any part-completed packets. Setting this bit enables this function but this is clear on reset (disabled)."]
pub type StopflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPTRIG` reader - Stop the formatter after a Trigger Event is observed. Reset to disabled (zero)."]
pub type StoptrigR = crate::BitReader;
#[doc = "Field `STOPTRIG` writer - Stop the formatter after a Trigger Event is observed. Reset to disabled (zero)."]
pub type StoptrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Do not embed Triggers into the formatted stream. Trace disable cycles and triggers are indicated by TRACECTL, where fitted. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
    #[inline(always)]
    pub fn enftc(&self) -> EnftcR {
        EnftcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode in the ETB corresponds to normal mode with the embedding of triggers. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
    #[inline(always)]
    pub fn enfcont(&self) -> EnfcontR {
        EnfcontR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable use of the flushin connection. This is clear on reset."]
    #[inline(always)]
    pub fn fonflin(&self) -> FonflinR {
        FonflinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate flush using Trigger event. Set this bit to cause a flush of data in the system when a Trigger Event occurs. This bit is clear on reset. A Trigger Event is defined as when the Trigger counter reaches zero (where fitted) or, in the case of the trigger counter being zero (or not fitted), when trigin is HIGH."]
    #[inline(always)]
    pub fn fontrig(&self) -> FontrigR {
        FontrigR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting this bit causes a flush to be generated. This is cleared when this flush has been serviced. This bit is clear on reset."]
    #[inline(always)]
    pub fn fonman(&self) -> FonmanR {
        FonmanR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicate a trigger on trigin being asserted."]
    #[inline(always)]
    pub fn trigin(&self) -> TriginR {
        TriginR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicate a trigger on a Trigger Event."]
    #[inline(always)]
    pub fn trigevt(&self) -> TrigevtR {
        TrigevtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates a trigger on Flush completion (afreadys being returned)."]
    #[inline(always)]
    pub fn trigfl(&self) -> TrigflR {
        TrigflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - This forces the FIFO to drain off any part-completed packets. Setting this bit enables this function but this is clear on reset (disabled)."]
    #[inline(always)]
    pub fn stopfl(&self) -> StopflR {
        StopflR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stop the formatter after a Trigger Event is observed. Reset to disabled (zero)."]
    #[inline(always)]
    pub fn stoptrig(&self) -> StoptrigR {
        StoptrigR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Do not embed Triggers into the formatted stream. Trace disable cycles and triggers are indicated by TRACECTL, where fitted. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
    #[inline(always)]
    pub fn enftc(&mut self) -> EnftcW<'_, FfcrSpec> {
        EnftcW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous mode in the ETB corresponds to normal mode with the embedding of triggers. Can only be changed when FtStopped is HIGH. This bit is clear on reset."]
    #[inline(always)]
    pub fn enfcont(&mut self) -> EnfcontW<'_, FfcrSpec> {
        EnfcontW::new(self, 1)
    }
    #[doc = "Bit 4 - Set this bit to enable use of the flushin connection. This is clear on reset."]
    #[inline(always)]
    pub fn fonflin(&mut self) -> FonflinW<'_, FfcrSpec> {
        FonflinW::new(self, 4)
    }
    #[doc = "Bit 5 - Generate flush using Trigger event. Set this bit to cause a flush of data in the system when a Trigger Event occurs. This bit is clear on reset. A Trigger Event is defined as when the Trigger counter reaches zero (where fitted) or, in the case of the trigger counter being zero (or not fitted), when trigin is HIGH."]
    #[inline(always)]
    pub fn fontrig(&mut self) -> FontrigW<'_, FfcrSpec> {
        FontrigW::new(self, 5)
    }
    #[doc = "Bit 6 - Setting this bit causes a flush to be generated. This is cleared when this flush has been serviced. This bit is clear on reset."]
    #[inline(always)]
    pub fn fonman(&mut self) -> FonmanW<'_, FfcrSpec> {
        FonmanW::new(self, 6)
    }
    #[doc = "Bit 8 - Indicate a trigger on trigin being asserted."]
    #[inline(always)]
    pub fn trigin(&mut self) -> TriginW<'_, FfcrSpec> {
        TriginW::new(self, 8)
    }
    #[doc = "Bit 9 - Indicate a trigger on a Trigger Event."]
    #[inline(always)]
    pub fn trigevt(&mut self) -> TrigevtW<'_, FfcrSpec> {
        TrigevtW::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates a trigger on Flush completion (afreadys being returned)."]
    #[inline(always)]
    pub fn trigfl(&mut self) -> TrigflW<'_, FfcrSpec> {
        TrigflW::new(self, 10)
    }
    #[doc = "Bit 12 - This forces the FIFO to drain off any part-completed packets. Setting this bit enables this function but this is clear on reset (disabled)."]
    #[inline(always)]
    pub fn stopfl(&mut self) -> StopflW<'_, FfcrSpec> {
        StopflW::new(self, 12)
    }
    #[doc = "Bit 13 - Stop the formatter after a Trigger Event is observed. Reset to disabled (zero)."]
    #[inline(always)]
    pub fn stoptrig(&mut self) -> StoptrigW<'_, FfcrSpec> {
        StoptrigW::new(self, 13)
    }
}
#[doc = "ETB Formatter and Flush Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfcrSpec;
impl crate::RegisterSpec for FfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffcr::R`](R) reader structure"]
impl crate::Readable for FfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffcr::W`](W) writer structure"]
impl crate::Writable for FfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFCR to value 0"]
impl crate::Resettable for FfcrSpec {}
