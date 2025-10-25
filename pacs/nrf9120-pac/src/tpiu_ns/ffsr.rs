#[doc = "Register `FFSR` reader"]
pub type R = crate::R<FfsrSpec>;
#[doc = "Register `FFSR` writer"]
pub type W = crate::W<FfsrSpec>;
#[doc = "Flush in progress.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flinprog {
    #[doc = "0: A flush is not in progress."]
    NotInProgress = 0,
    #[doc = "1: A flush is in progress."]
    InProgress = 1,
}
impl From<Flinprog> for bool {
    #[inline(always)]
    fn from(variant: Flinprog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLINPROG` reader - Flush in progress."]
pub type FlinprogR = crate::BitReader<Flinprog>;
impl FlinprogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flinprog {
        match self.bits {
            false => Flinprog::NotInProgress,
            true => Flinprog::InProgress,
        }
    }
    #[doc = "A flush is not in progress."]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == Flinprog::NotInProgress
    }
    #[doc = "A flush is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Flinprog::InProgress
    }
}
#[doc = "Field `FLINPROG` writer - Flush in progress."]
pub type FlinprogW<'a, REG> = crate::BitWriter<'a, REG, Flinprog>;
impl<'a, REG> FlinprogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A flush is not in progress."]
    #[inline(always)]
    pub fn not_in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Flinprog::NotInProgress)
    }
    #[doc = "A flush is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(Flinprog::InProgress)
    }
}
#[doc = "The formatter has received a stop request signal and all trace data and post-amble is sent. Any additional trace data on the ATB interface is ignored and atreadys goes HIGH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftstopped {
    #[doc = "0: Formatter has not stopped."]
    Running = 0,
    #[doc = "1: Formatter has stopped."]
    Stopped = 1,
}
impl From<Ftstopped> for bool {
    #[inline(always)]
    fn from(variant: Ftstopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTSTOPPED` reader - The formatter has received a stop request signal and all trace data and post-amble is sent. Any additional trace data on the ATB interface is ignored and atreadys goes HIGH."]
pub type FtstoppedR = crate::BitReader<Ftstopped>;
impl FtstoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftstopped {
        match self.bits {
            false => Ftstopped::Running,
            true => Ftstopped::Stopped,
        }
    }
    #[doc = "Formatter has not stopped."]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Ftstopped::Running
    }
    #[doc = "Formatter has stopped."]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Ftstopped::Stopped
    }
}
#[doc = "Field `FTSTOPPED` writer - The formatter has received a stop request signal and all trace data and post-amble is sent. Any additional trace data on the ATB interface is ignored and atreadys goes HIGH."]
pub type FtstoppedW<'a, REG> = crate::BitWriter<'a, REG, Ftstopped>;
impl<'a, REG> FtstoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Formatter has not stopped."]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(Ftstopped::Running)
    }
    #[doc = "Formatter has stopped."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Ftstopped::Stopped)
    }
}
#[doc = "Indicates whether the TRACECTL pin is available for use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpresent {
    #[doc = "0: TRACECTL pin is not present."]
    NotPresent = 0,
    #[doc = "1: TRACECTL pin is present."]
    Present = 1,
}
impl From<Tcpresent> for bool {
    #[inline(always)]
    fn from(variant: Tcpresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPRESENT` reader - Indicates whether the TRACECTL pin is available for use."]
pub type TcpresentR = crate::BitReader<Tcpresent>;
impl TcpresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpresent {
        match self.bits {
            false => Tcpresent::NotPresent,
            true => Tcpresent::Present,
        }
    }
    #[doc = "TRACECTL pin is not present."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Tcpresent::NotPresent
    }
    #[doc = "TRACECTL pin is present."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Tcpresent::Present
    }
}
#[doc = "Field `TCPRESENT` writer - Indicates whether the TRACECTL pin is available for use."]
pub type TcpresentW<'a, REG> = crate::BitWriter<'a, REG, Tcpresent>;
impl<'a, REG> TcpresentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRACECTL pin is not present."]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(Tcpresent::NotPresent)
    }
    #[doc = "TRACECTL pin is present."]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Tcpresent::Present)
    }
}
impl R {
    #[doc = "Bit 0 - Flush in progress."]
    #[inline(always)]
    pub fn flinprog(&self) -> FlinprogR {
        FlinprogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The formatter has received a stop request signal and all trace data and post-amble is sent. Any additional trace data on the ATB interface is ignored and atreadys goes HIGH."]
    #[inline(always)]
    pub fn ftstopped(&self) -> FtstoppedR {
        FtstoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates whether the TRACECTL pin is available for use."]
    #[inline(always)]
    pub fn tcpresent(&self) -> TcpresentR {
        TcpresentR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush in progress."]
    #[inline(always)]
    pub fn flinprog(&mut self) -> FlinprogW<'_, FfsrSpec> {
        FlinprogW::new(self, 0)
    }
    #[doc = "Bit 1 - The formatter has received a stop request signal and all trace data and post-amble is sent. Any additional trace data on the ATB interface is ignored and atreadys goes HIGH."]
    #[inline(always)]
    pub fn ftstopped(&mut self) -> FtstoppedW<'_, FfsrSpec> {
        FtstoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates whether the TRACECTL pin is available for use."]
    #[inline(always)]
    pub fn tcpresent(&mut self) -> TcpresentW<'_, FfsrSpec> {
        TcpresentW::new(self, 2)
    }
}
#[doc = "The FFSR register indicates the current status of the formatter and flush features available in the TPIU.\n\nYou can [`read`](crate::Reg::read) this register and get [`ffsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfsrSpec;
impl crate::RegisterSpec for FfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffsr::R`](R) reader structure"]
impl crate::Readable for FfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffsr::W`](W) writer structure"]
impl crate::Writable for FfsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFSR to value 0"]
impl crate::Resettable for FfsrSpec {}
