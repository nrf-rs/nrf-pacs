#[doc = "Register `DPDMVALUE` reader"]
pub type R = crate::R<DpdmvalueSpec>;
#[doc = "Register `DPDMVALUE` writer"]
pub type W = crate::W<DpdmvalueSpec>;
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "1: D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    Resume = 1,
    #[doc = "2: D+ forced high, D- forced low (J state)"]
    J = 2,
    #[doc = "4: D+ forced low, D- forced high (K state)"]
    K = 4,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            1 => Some(State::Resume),
            2 => Some(State::J),
            4 => Some(State::K),
            _ => None,
        }
    }
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == State::Resume
    }
    #[doc = "D+ forced high, D- forced low (J state)"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == State::J
    }
    #[doc = "D+ forced low, D- forced high (K state)"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == State::K
    }
}
#[doc = "Field `STATE` writer - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 5, State>;
impl<'a, REG> StateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(State::Resume)
    }
    #[doc = "D+ forced high, D- forced low (J state)"]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(State::J)
    }
    #[doc = "D+ forced low, D- forced high (K state)"]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(State::K)
    }
}
impl R {
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn state(&mut self) -> StateW<'_, DpdmvalueSpec> {
        StateW::new(self, 0)
    }
}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing).\n\nYou can [`read`](crate::Reg::read) this register and get [`dpdmvalue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdmvalue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpdmvalueSpec;
impl crate::RegisterSpec for DpdmvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpdmvalue::R`](R) reader structure"]
impl crate::Readable for DpdmvalueSpec {}
#[doc = "`write(|w| ..)` method takes [`dpdmvalue::W`](W) writer structure"]
impl crate::Writable for DpdmvalueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPDMVALUE to value 0"]
impl crate::Resettable for DpdmvalueSpec {}
