#[doc = "Register `RREN` reader"]
pub type R = crate::R<RrenSpec>;
#[doc = "Register `RREN` writer"]
pub type W = crate::W<RrenSpec>;
#[doc = "Enable or disable RR\\[0\\] register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr0 {
    #[doc = "0: Disable RR\\[0\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[0\\] register"]
    Enabled = 1,
}
impl From<Rr0> for bool {
    #[inline(always)]
    fn from(variant: Rr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR0` reader - Enable or disable RR\\[0\\] register"]
pub type Rr0R = crate::BitReader<Rr0>;
impl Rr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr0 {
        match self.bits {
            false => Rr0::Disabled,
            true => Rr0::Enabled,
        }
    }
    #[doc = "Disable RR\\[0\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr0::Disabled
    }
    #[doc = "Enable RR\\[0\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr0::Enabled
    }
}
#[doc = "Field `RR0` writer - Enable or disable RR\\[0\\] register"]
pub type Rr0W<'a, REG> = crate::BitWriter<'a, REG, Rr0>;
impl<'a, REG> Rr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[0\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr0::Disabled)
    }
    #[doc = "Enable RR\\[0\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr0::Enabled)
    }
}
#[doc = "Enable or disable RR\\[1\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr1 {
    #[doc = "0: Disable RR\\[1\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[1\\] register"]
    Enabled = 1,
}
impl From<Rr1> for bool {
    #[inline(always)]
    fn from(variant: Rr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR1` reader - Enable or disable RR\\[1\\] register"]
pub type Rr1R = crate::BitReader<Rr1>;
impl Rr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr1 {
        match self.bits {
            false => Rr1::Disabled,
            true => Rr1::Enabled,
        }
    }
    #[doc = "Disable RR\\[1\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr1::Disabled
    }
    #[doc = "Enable RR\\[1\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr1::Enabled
    }
}
#[doc = "Field `RR1` writer - Enable or disable RR\\[1\\] register"]
pub type Rr1W<'a, REG> = crate::BitWriter<'a, REG, Rr1>;
impl<'a, REG> Rr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[1\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr1::Disabled)
    }
    #[doc = "Enable RR\\[1\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr1::Enabled)
    }
}
#[doc = "Enable or disable RR\\[2\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr2 {
    #[doc = "0: Disable RR\\[2\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[2\\] register"]
    Enabled = 1,
}
impl From<Rr2> for bool {
    #[inline(always)]
    fn from(variant: Rr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR2` reader - Enable or disable RR\\[2\\] register"]
pub type Rr2R = crate::BitReader<Rr2>;
impl Rr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr2 {
        match self.bits {
            false => Rr2::Disabled,
            true => Rr2::Enabled,
        }
    }
    #[doc = "Disable RR\\[2\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr2::Disabled
    }
    #[doc = "Enable RR\\[2\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr2::Enabled
    }
}
#[doc = "Field `RR2` writer - Enable or disable RR\\[2\\] register"]
pub type Rr2W<'a, REG> = crate::BitWriter<'a, REG, Rr2>;
impl<'a, REG> Rr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[2\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr2::Disabled)
    }
    #[doc = "Enable RR\\[2\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr2::Enabled)
    }
}
#[doc = "Enable or disable RR\\[3\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr3 {
    #[doc = "0: Disable RR\\[3\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[3\\] register"]
    Enabled = 1,
}
impl From<Rr3> for bool {
    #[inline(always)]
    fn from(variant: Rr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR3` reader - Enable or disable RR\\[3\\] register"]
pub type Rr3R = crate::BitReader<Rr3>;
impl Rr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr3 {
        match self.bits {
            false => Rr3::Disabled,
            true => Rr3::Enabled,
        }
    }
    #[doc = "Disable RR\\[3\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr3::Disabled
    }
    #[doc = "Enable RR\\[3\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr3::Enabled
    }
}
#[doc = "Field `RR3` writer - Enable or disable RR\\[3\\] register"]
pub type Rr3W<'a, REG> = crate::BitWriter<'a, REG, Rr3>;
impl<'a, REG> Rr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[3\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr3::Disabled)
    }
    #[doc = "Enable RR\\[3\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr3::Enabled)
    }
}
#[doc = "Enable or disable RR\\[4\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr4 {
    #[doc = "0: Disable RR\\[4\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[4\\] register"]
    Enabled = 1,
}
impl From<Rr4> for bool {
    #[inline(always)]
    fn from(variant: Rr4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR4` reader - Enable or disable RR\\[4\\] register"]
pub type Rr4R = crate::BitReader<Rr4>;
impl Rr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr4 {
        match self.bits {
            false => Rr4::Disabled,
            true => Rr4::Enabled,
        }
    }
    #[doc = "Disable RR\\[4\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr4::Disabled
    }
    #[doc = "Enable RR\\[4\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr4::Enabled
    }
}
#[doc = "Field `RR4` writer - Enable or disable RR\\[4\\] register"]
pub type Rr4W<'a, REG> = crate::BitWriter<'a, REG, Rr4>;
impl<'a, REG> Rr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[4\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr4::Disabled)
    }
    #[doc = "Enable RR\\[4\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr4::Enabled)
    }
}
#[doc = "Enable or disable RR\\[5\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr5 {
    #[doc = "0: Disable RR\\[5\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[5\\] register"]
    Enabled = 1,
}
impl From<Rr5> for bool {
    #[inline(always)]
    fn from(variant: Rr5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR5` reader - Enable or disable RR\\[5\\] register"]
pub type Rr5R = crate::BitReader<Rr5>;
impl Rr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr5 {
        match self.bits {
            false => Rr5::Disabled,
            true => Rr5::Enabled,
        }
    }
    #[doc = "Disable RR\\[5\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr5::Disabled
    }
    #[doc = "Enable RR\\[5\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr5::Enabled
    }
}
#[doc = "Field `RR5` writer - Enable or disable RR\\[5\\] register"]
pub type Rr5W<'a, REG> = crate::BitWriter<'a, REG, Rr5>;
impl<'a, REG> Rr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[5\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr5::Disabled)
    }
    #[doc = "Enable RR\\[5\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr5::Enabled)
    }
}
#[doc = "Enable or disable RR\\[6\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr6 {
    #[doc = "0: Disable RR\\[6\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[6\\] register"]
    Enabled = 1,
}
impl From<Rr6> for bool {
    #[inline(always)]
    fn from(variant: Rr6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR6` reader - Enable or disable RR\\[6\\] register"]
pub type Rr6R = crate::BitReader<Rr6>;
impl Rr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr6 {
        match self.bits {
            false => Rr6::Disabled,
            true => Rr6::Enabled,
        }
    }
    #[doc = "Disable RR\\[6\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr6::Disabled
    }
    #[doc = "Enable RR\\[6\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr6::Enabled
    }
}
#[doc = "Field `RR6` writer - Enable or disable RR\\[6\\] register"]
pub type Rr6W<'a, REG> = crate::BitWriter<'a, REG, Rr6>;
impl<'a, REG> Rr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[6\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr6::Disabled)
    }
    #[doc = "Enable RR\\[6\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr6::Enabled)
    }
}
#[doc = "Enable or disable RR\\[7\\] register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr7 {
    #[doc = "0: Disable RR\\[7\\] register"]
    Disabled = 0,
    #[doc = "1: Enable RR\\[7\\] register"]
    Enabled = 1,
}
impl From<Rr7> for bool {
    #[inline(always)]
    fn from(variant: Rr7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR7` reader - Enable or disable RR\\[7\\] register"]
pub type Rr7R = crate::BitReader<Rr7>;
impl Rr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr7 {
        match self.bits {
            false => Rr7::Disabled,
            true => Rr7::Enabled,
        }
    }
    #[doc = "Disable RR\\[7\\] register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rr7::Disabled
    }
    #[doc = "Enable RR\\[7\\] register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rr7::Enabled
    }
}
#[doc = "Field `RR7` writer - Enable or disable RR\\[7\\] register"]
pub type Rr7W<'a, REG> = crate::BitWriter<'a, REG, Rr7>;
impl<'a, REG> Rr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RR\\[7\\] register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr7::Disabled)
    }
    #[doc = "Enable RR\\[7\\] register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rr7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable RR\\[0\\] register"]
    #[inline(always)]
    pub fn rr0(&self) -> Rr0R {
        Rr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\] register"]
    #[inline(always)]
    pub fn rr1(&self) -> Rr1R {
        Rr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\] register"]
    #[inline(always)]
    pub fn rr2(&self) -> Rr2R {
        Rr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\] register"]
    #[inline(always)]
    pub fn rr3(&self) -> Rr3R {
        Rr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\] register"]
    #[inline(always)]
    pub fn rr4(&self) -> Rr4R {
        Rr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\] register"]
    #[inline(always)]
    pub fn rr5(&self) -> Rr5R {
        Rr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\] register"]
    #[inline(always)]
    pub fn rr6(&self) -> Rr6R {
        Rr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\] register"]
    #[inline(always)]
    pub fn rr7(&self) -> Rr7R {
        Rr7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable RR\\[0\\] register"]
    #[inline(always)]
    pub fn rr0(&mut self) -> Rr0W<'_, RrenSpec> {
        Rr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\] register"]
    #[inline(always)]
    pub fn rr1(&mut self) -> Rr1W<'_, RrenSpec> {
        Rr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\] register"]
    #[inline(always)]
    pub fn rr2(&mut self) -> Rr2W<'_, RrenSpec> {
        Rr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\] register"]
    #[inline(always)]
    pub fn rr3(&mut self) -> Rr3W<'_, RrenSpec> {
        Rr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\] register"]
    #[inline(always)]
    pub fn rr4(&mut self) -> Rr4W<'_, RrenSpec> {
        Rr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\] register"]
    #[inline(always)]
    pub fn rr5(&mut self) -> Rr5W<'_, RrenSpec> {
        Rr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\] register"]
    #[inline(always)]
    pub fn rr6(&mut self) -> Rr6W<'_, RrenSpec> {
        Rr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\] register"]
    #[inline(always)]
    pub fn rr7(&mut self) -> Rr7W<'_, RrenSpec> {
        Rr7W::new(self, 7)
    }
}
#[doc = "Enable register for reload request registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrenSpec;
impl crate::RegisterSpec for RrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rren::R`](R) reader structure"]
impl crate::Readable for RrenSpec {}
#[doc = "`write(|w| ..)` method takes [`rren::W`](W) writer structure"]
impl crate::Writable for RrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RREN to value 0x01"]
impl crate::Resettable for RrenSpec {
    const RESET_VALUE: u32 = 0x01;
}
