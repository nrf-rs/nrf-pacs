#[doc = "Register `RECEIVE_CNF[%s]` reader"]
pub type R = crate::R<ReceiveCnfSpec>;
#[doc = "Register `RECEIVE_CNF[%s]` writer"]
pub type W = crate::W<ReceiveCnfSpec>;
#[doc = "Enable subscription to IPC channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen0 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen0> for bool {
    #[inline(always)]
    fn from(variant: Chen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN0` reader - Enable subscription to IPC channel 0"]
pub type Chen0R = crate::BitReader<Chen0>;
impl Chen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen0 {
        match self.bits {
            false => Chen0::Disable,
            true => Chen0::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen0::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen0::Enable
    }
}
#[doc = "Field `CHEN0` writer - Enable subscription to IPC channel 0"]
pub type Chen0W<'a, REG> = crate::BitWriter<'a, REG, Chen0>;
impl<'a, REG> Chen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen0::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen0::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen1 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen1> for bool {
    #[inline(always)]
    fn from(variant: Chen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN1` reader - Enable subscription to IPC channel 1"]
pub type Chen1R = crate::BitReader<Chen1>;
impl Chen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen1 {
        match self.bits {
            false => Chen1::Disable,
            true => Chen1::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen1::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen1::Enable
    }
}
#[doc = "Field `CHEN1` writer - Enable subscription to IPC channel 1"]
pub type Chen1W<'a, REG> = crate::BitWriter<'a, REG, Chen1>;
impl<'a, REG> Chen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen1::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen1::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen2 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen2> for bool {
    #[inline(always)]
    fn from(variant: Chen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN2` reader - Enable subscription to IPC channel 2"]
pub type Chen2R = crate::BitReader<Chen2>;
impl Chen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen2 {
        match self.bits {
            false => Chen2::Disable,
            true => Chen2::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen2::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen2::Enable
    }
}
#[doc = "Field `CHEN2` writer - Enable subscription to IPC channel 2"]
pub type Chen2W<'a, REG> = crate::BitWriter<'a, REG, Chen2>;
impl<'a, REG> Chen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen2::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen2::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen3 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen3> for bool {
    #[inline(always)]
    fn from(variant: Chen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN3` reader - Enable subscription to IPC channel 3"]
pub type Chen3R = crate::BitReader<Chen3>;
impl Chen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen3 {
        match self.bits {
            false => Chen3::Disable,
            true => Chen3::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen3::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen3::Enable
    }
}
#[doc = "Field `CHEN3` writer - Enable subscription to IPC channel 3"]
pub type Chen3W<'a, REG> = crate::BitWriter<'a, REG, Chen3>;
impl<'a, REG> Chen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen3::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen3::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen4 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen4> for bool {
    #[inline(always)]
    fn from(variant: Chen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN4` reader - Enable subscription to IPC channel 4"]
pub type Chen4R = crate::BitReader<Chen4>;
impl Chen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen4 {
        match self.bits {
            false => Chen4::Disable,
            true => Chen4::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen4::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen4::Enable
    }
}
#[doc = "Field `CHEN4` writer - Enable subscription to IPC channel 4"]
pub type Chen4W<'a, REG> = crate::BitWriter<'a, REG, Chen4>;
impl<'a, REG> Chen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen4::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen4::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen5 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen5> for bool {
    #[inline(always)]
    fn from(variant: Chen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN5` reader - Enable subscription to IPC channel 5"]
pub type Chen5R = crate::BitReader<Chen5>;
impl Chen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen5 {
        match self.bits {
            false => Chen5::Disable,
            true => Chen5::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen5::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen5::Enable
    }
}
#[doc = "Field `CHEN5` writer - Enable subscription to IPC channel 5"]
pub type Chen5W<'a, REG> = crate::BitWriter<'a, REG, Chen5>;
impl<'a, REG> Chen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen5::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen5::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen6 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen6> for bool {
    #[inline(always)]
    fn from(variant: Chen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN6` reader - Enable subscription to IPC channel 6"]
pub type Chen6R = crate::BitReader<Chen6>;
impl Chen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen6 {
        match self.bits {
            false => Chen6::Disable,
            true => Chen6::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen6::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen6::Enable
    }
}
#[doc = "Field `CHEN6` writer - Enable subscription to IPC channel 6"]
pub type Chen6W<'a, REG> = crate::BitWriter<'a, REG, Chen6>;
impl<'a, REG> Chen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen6::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen6::Enable)
    }
}
#[doc = "Enable subscription to IPC channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen7 {
    #[doc = "0: Disable events"]
    Disable = 0,
    #[doc = "1: Enable events"]
    Enable = 1,
}
impl From<Chen7> for bool {
    #[inline(always)]
    fn from(variant: Chen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN7` reader - Enable subscription to IPC channel 7"]
pub type Chen7R = crate::BitReader<Chen7>;
impl Chen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen7 {
        match self.bits {
            false => Chen7::Disable,
            true => Chen7::Enable,
        }
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen7::Disable
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen7::Enable
    }
}
#[doc = "Field `CHEN7` writer - Enable subscription to IPC channel 7"]
pub type Chen7W<'a, REG> = crate::BitWriter<'a, REG, Chen7>;
impl<'a, REG> Chen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen7::Disable)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen7::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable subscription to IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&self) -> Chen0R {
        Chen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable subscription to IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&self) -> Chen1R {
        Chen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable subscription to IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&self) -> Chen2R {
        Chen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable subscription to IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&self) -> Chen3R {
        Chen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable subscription to IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&self) -> Chen4R {
        Chen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable subscription to IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&self) -> Chen5R {
        Chen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable subscription to IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&self) -> Chen6R {
        Chen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable subscription to IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&self) -> Chen7R {
        Chen7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subscription to IPC channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn chen0(&mut self) -> Chen0W<ReceiveCnfSpec> {
        Chen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable subscription to IPC channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn chen1(&mut self) -> Chen1W<ReceiveCnfSpec> {
        Chen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable subscription to IPC channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn chen2(&mut self) -> Chen2W<ReceiveCnfSpec> {
        Chen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable subscription to IPC channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn chen3(&mut self) -> Chen3W<ReceiveCnfSpec> {
        Chen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable subscription to IPC channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn chen4(&mut self) -> Chen4W<ReceiveCnfSpec> {
        Chen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable subscription to IPC channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn chen5(&mut self) -> Chen5W<ReceiveCnfSpec> {
        Chen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable subscription to IPC channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn chen6(&mut self) -> Chen6W<ReceiveCnfSpec> {
        Chen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable subscription to IPC channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn chen7(&mut self) -> Chen7W<ReceiveCnfSpec> {
        Chen7W::new(self, 7)
    }
}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_cnf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_cnf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveCnfSpec;
impl crate::RegisterSpec for ReceiveCnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_cnf::R`](R) reader structure"]
impl crate::Readable for ReceiveCnfSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_cnf::W`](W) writer structure"]
impl crate::Writable for ReceiveCnfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECEIVE_CNF[%s]
to value 0"]
impl crate::Resettable for ReceiveCnfSpec {
    const RESET_VALUE: u32 = 0;
}
