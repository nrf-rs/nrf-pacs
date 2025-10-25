#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare0> for bool {
    #[inline(always)]
    fn from(variant: Compare0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
pub type Compare0R = crate::BitReader<Compare0>;
impl Compare0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0 {
        match self.bits {
            false => Compare0::Disabled,
            true => Compare0::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare0::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare0WO> for bool {
    #[inline(always)]
    fn from(variant: Compare0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` writer - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
pub type Compare0W<'a, REG> = crate::BitWriter<'a, REG, Compare0WO>;
impl<'a, REG> Compare0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare1> for bool {
    #[inline(always)]
    fn from(variant: Compare1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
pub type Compare1R = crate::BitReader<Compare1>;
impl Compare1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1 {
        match self.bits {
            false => Compare1::Disabled,
            true => Compare1::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare1::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare1WO> for bool {
    #[inline(always)]
    fn from(variant: Compare1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` writer - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
pub type Compare1W<'a, REG> = crate::BitWriter<'a, REG, Compare1WO>;
impl<'a, REG> Compare1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare2> for bool {
    #[inline(always)]
    fn from(variant: Compare2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
pub type Compare2R = crate::BitReader<Compare2>;
impl Compare2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2 {
        match self.bits {
            false => Compare2::Disabled,
            true => Compare2::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare2::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare2WO> for bool {
    #[inline(always)]
    fn from(variant: Compare2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` writer - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
pub type Compare2W<'a, REG> = crate::BitWriter<'a, REG, Compare2WO>;
impl<'a, REG> Compare2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare3> for bool {
    #[inline(always)]
    fn from(variant: Compare3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
pub type Compare3R = crate::BitReader<Compare3>;
impl Compare3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3 {
        match self.bits {
            false => Compare3::Disabled,
            true => Compare3::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare3::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare3WO> for bool {
    #[inline(always)]
    fn from(variant: Compare3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` writer - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
pub type Compare3W<'a, REG> = crate::BitWriter<'a, REG, Compare3WO>;
impl<'a, REG> Compare3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare4 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare4> for bool {
    #[inline(always)]
    fn from(variant: Compare4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` reader - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
pub type Compare4R = crate::BitReader<Compare4>;
impl Compare4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare4 {
        match self.bits {
            false => Compare4::Disabled,
            true => Compare4::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare4::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare4::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare4WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare4WO> for bool {
    #[inline(always)]
    fn from(variant: Compare4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` writer - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
pub type Compare4W<'a, REG> = crate::BitWriter<'a, REG, Compare4WO>;
impl<'a, REG> Compare4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare5 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare5> for bool {
    #[inline(always)]
    fn from(variant: Compare5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` reader - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
pub type Compare5R = crate::BitReader<Compare5>;
impl Compare5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare5 {
        match self.bits {
            false => Compare5::Disabled,
            true => Compare5::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare5::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare5::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare5WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare5WO> for bool {
    #[inline(always)]
    fn from(variant: Compare5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` writer - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
pub type Compare5W<'a, REG> = crate::BitWriter<'a, REG, Compare5WO>;
impl<'a, REG> Compare5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare6 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare6> for bool {
    #[inline(always)]
    fn from(variant: Compare6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE6` reader - Write '1' to disable interrupt for event COMPARE\\[6\\]"]
pub type Compare6R = crate::BitReader<Compare6>;
impl Compare6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare6 {
        match self.bits {
            false => Compare6::Disabled,
            true => Compare6::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare6::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare6::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare6WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare6WO> for bool {
    #[inline(always)]
    fn from(variant: Compare6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE6` writer - Write '1' to disable interrupt for event COMPARE\\[6\\]"]
pub type Compare6W<'a, REG> = crate::BitWriter<'a, REG, Compare6WO>;
impl<'a, REG> Compare6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare7 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare7> for bool {
    #[inline(always)]
    fn from(variant: Compare7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE7` reader - Write '1' to disable interrupt for event COMPARE\\[7\\]"]
pub type Compare7R = crate::BitReader<Compare7>;
impl Compare7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare7 {
        match self.bits {
            false => Compare7::Disabled,
            true => Compare7::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare7::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare7::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare7WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Compare7WO> for bool {
    #[inline(always)]
    fn from(variant: Compare7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE7` writer - Write '1' to disable interrupt for event COMPARE\\[7\\]"]
pub type Compare7W<'a, REG> = crate::BitWriter<'a, REG, Compare7WO>;
impl<'a, REG> Compare7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7WO::Clear)
    }
}
impl R {
    #[doc = "Bit 16 - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> Compare0R {
        Compare0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> Compare1R {
        Compare1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> Compare2R {
        Compare2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> Compare3R {
        Compare3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&self) -> Compare4R {
        Compare4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&self) -> Compare5R {
        Compare5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event COMPARE\\[6\\]"]
    #[inline(always)]
    pub fn compare6(&self) -> Compare6R {
        Compare6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event COMPARE\\[7\\]"]
    #[inline(always)]
    pub fn compare7(&self) -> Compare7R {
        Compare7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&mut self) -> Compare0W<'_, IntenclrSpec> {
        Compare0W::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&mut self) -> Compare1W<'_, IntenclrSpec> {
        Compare1W::new(self, 17)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&mut self) -> Compare2W<'_, IntenclrSpec> {
        Compare2W::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&mut self) -> Compare3W<'_, IntenclrSpec> {
        Compare3W::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&mut self) -> Compare4W<'_, IntenclrSpec> {
        Compare4W::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&mut self) -> Compare5W<'_, IntenclrSpec> {
        Compare5W::new(self, 21)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event COMPARE\\[6\\]"]
    #[inline(always)]
    pub fn compare6(&mut self) -> Compare6W<'_, IntenclrSpec> {
        Compare6W::new(self, 22)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event COMPARE\\[7\\]"]
    #[inline(always)]
    pub fn compare7(&mut self) -> Compare7W<'_, IntenclrSpec> {
        Compare7W::new(self, 23)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
