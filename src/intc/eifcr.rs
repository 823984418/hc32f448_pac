#[doc = "Register `EIFCR` reader"]
pub type R = crate::R<EifcrSpec>;
#[doc = "Register `EIFCR` writer"]
pub type W = crate::W<EifcrSpec>;
#[doc = "Field `EIFCLR0` reader - desc EIFCLR0"]
pub type Eifclr0R = crate::BitReader;
#[doc = "Field `EIFCLR0` writer - desc EIFCLR0"]
pub type Eifclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR1` reader - desc EIFCLR1"]
pub type Eifclr1R = crate::BitReader;
#[doc = "Field `EIFCLR1` writer - desc EIFCLR1"]
pub type Eifclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR2` reader - desc EIFCLR2"]
pub type Eifclr2R = crate::BitReader;
#[doc = "Field `EIFCLR2` writer - desc EIFCLR2"]
pub type Eifclr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR3` reader - desc EIFCLR3"]
pub type Eifclr3R = crate::BitReader;
#[doc = "Field `EIFCLR3` writer - desc EIFCLR3"]
pub type Eifclr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR4` reader - desc EIFCLR4"]
pub type Eifclr4R = crate::BitReader;
#[doc = "Field `EIFCLR4` writer - desc EIFCLR4"]
pub type Eifclr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR5` reader - desc EIFCLR5"]
pub type Eifclr5R = crate::BitReader;
#[doc = "Field `EIFCLR5` writer - desc EIFCLR5"]
pub type Eifclr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR6` reader - desc EIFCLR6"]
pub type Eifclr6R = crate::BitReader;
#[doc = "Field `EIFCLR6` writer - desc EIFCLR6"]
pub type Eifclr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR7` reader - desc EIFCLR7"]
pub type Eifclr7R = crate::BitReader;
#[doc = "Field `EIFCLR7` writer - desc EIFCLR7"]
pub type Eifclr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR8` reader - desc EIFCLR8"]
pub type Eifclr8R = crate::BitReader;
#[doc = "Field `EIFCLR8` writer - desc EIFCLR8"]
pub type Eifclr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR9` reader - desc EIFCLR9"]
pub type Eifclr9R = crate::BitReader;
#[doc = "Field `EIFCLR9` writer - desc EIFCLR9"]
pub type Eifclr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR10` reader - desc EIFCLR10"]
pub type Eifclr10R = crate::BitReader;
#[doc = "Field `EIFCLR10` writer - desc EIFCLR10"]
pub type Eifclr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR11` reader - desc EIFCLR11"]
pub type Eifclr11R = crate::BitReader;
#[doc = "Field `EIFCLR11` writer - desc EIFCLR11"]
pub type Eifclr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR12` reader - desc EIFCLR12"]
pub type Eifclr12R = crate::BitReader;
#[doc = "Field `EIFCLR12` writer - desc EIFCLR12"]
pub type Eifclr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR13` reader - desc EIFCLR13"]
pub type Eifclr13R = crate::BitReader;
#[doc = "Field `EIFCLR13` writer - desc EIFCLR13"]
pub type Eifclr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR14` reader - desc EIFCLR14"]
pub type Eifclr14R = crate::BitReader;
#[doc = "Field `EIFCLR14` writer - desc EIFCLR14"]
pub type Eifclr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIFCLR15` reader - desc EIFCLR15"]
pub type Eifclr15R = crate::BitReader;
#[doc = "Field `EIFCLR15` writer - desc EIFCLR15"]
pub type Eifclr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EIFCLR0"]
    #[inline(always)]
    pub fn eifclr0(&self) -> Eifclr0R {
        Eifclr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIFCLR1"]
    #[inline(always)]
    pub fn eifclr1(&self) -> Eifclr1R {
        Eifclr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EIFCLR2"]
    #[inline(always)]
    pub fn eifclr2(&self) -> Eifclr2R {
        Eifclr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EIFCLR3"]
    #[inline(always)]
    pub fn eifclr3(&self) -> Eifclr3R {
        Eifclr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EIFCLR4"]
    #[inline(always)]
    pub fn eifclr4(&self) -> Eifclr4R {
        Eifclr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EIFCLR5"]
    #[inline(always)]
    pub fn eifclr5(&self) -> Eifclr5R {
        Eifclr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EIFCLR6"]
    #[inline(always)]
    pub fn eifclr6(&self) -> Eifclr6R {
        Eifclr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EIFCLR7"]
    #[inline(always)]
    pub fn eifclr7(&self) -> Eifclr7R {
        Eifclr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EIFCLR8"]
    #[inline(always)]
    pub fn eifclr8(&self) -> Eifclr8R {
        Eifclr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EIFCLR9"]
    #[inline(always)]
    pub fn eifclr9(&self) -> Eifclr9R {
        Eifclr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EIFCLR10"]
    #[inline(always)]
    pub fn eifclr10(&self) -> Eifclr10R {
        Eifclr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EIFCLR11"]
    #[inline(always)]
    pub fn eifclr11(&self) -> Eifclr11R {
        Eifclr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EIFCLR12"]
    #[inline(always)]
    pub fn eifclr12(&self) -> Eifclr12R {
        Eifclr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EIFCLR13"]
    #[inline(always)]
    pub fn eifclr13(&self) -> Eifclr13R {
        Eifclr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EIFCLR14"]
    #[inline(always)]
    pub fn eifclr14(&self) -> Eifclr14R {
        Eifclr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EIFCLR15"]
    #[inline(always)]
    pub fn eifclr15(&self) -> Eifclr15R {
        Eifclr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EIFCR")
            .field("eifclr0", &self.eifclr0())
            .field("eifclr1", &self.eifclr1())
            .field("eifclr2", &self.eifclr2())
            .field("eifclr3", &self.eifclr3())
            .field("eifclr4", &self.eifclr4())
            .field("eifclr5", &self.eifclr5())
            .field("eifclr6", &self.eifclr6())
            .field("eifclr7", &self.eifclr7())
            .field("eifclr8", &self.eifclr8())
            .field("eifclr9", &self.eifclr9())
            .field("eifclr10", &self.eifclr10())
            .field("eifclr11", &self.eifclr11())
            .field("eifclr12", &self.eifclr12())
            .field("eifclr13", &self.eifclr13())
            .field("eifclr14", &self.eifclr14())
            .field("eifclr15", &self.eifclr15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc EIFCLR0"]
    #[inline(always)]
    pub fn eifclr0(&mut self) -> Eifclr0W<'_, EifcrSpec> {
        Eifclr0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc EIFCLR1"]
    #[inline(always)]
    pub fn eifclr1(&mut self) -> Eifclr1W<'_, EifcrSpec> {
        Eifclr1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc EIFCLR2"]
    #[inline(always)]
    pub fn eifclr2(&mut self) -> Eifclr2W<'_, EifcrSpec> {
        Eifclr2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc EIFCLR3"]
    #[inline(always)]
    pub fn eifclr3(&mut self) -> Eifclr3W<'_, EifcrSpec> {
        Eifclr3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc EIFCLR4"]
    #[inline(always)]
    pub fn eifclr4(&mut self) -> Eifclr4W<'_, EifcrSpec> {
        Eifclr4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc EIFCLR5"]
    #[inline(always)]
    pub fn eifclr5(&mut self) -> Eifclr5W<'_, EifcrSpec> {
        Eifclr5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc EIFCLR6"]
    #[inline(always)]
    pub fn eifclr6(&mut self) -> Eifclr6W<'_, EifcrSpec> {
        Eifclr6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc EIFCLR7"]
    #[inline(always)]
    pub fn eifclr7(&mut self) -> Eifclr7W<'_, EifcrSpec> {
        Eifclr7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc EIFCLR8"]
    #[inline(always)]
    pub fn eifclr8(&mut self) -> Eifclr8W<'_, EifcrSpec> {
        Eifclr8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc EIFCLR9"]
    #[inline(always)]
    pub fn eifclr9(&mut self) -> Eifclr9W<'_, EifcrSpec> {
        Eifclr9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc EIFCLR10"]
    #[inline(always)]
    pub fn eifclr10(&mut self) -> Eifclr10W<'_, EifcrSpec> {
        Eifclr10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc EIFCLR11"]
    #[inline(always)]
    pub fn eifclr11(&mut self) -> Eifclr11W<'_, EifcrSpec> {
        Eifclr11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc EIFCLR12"]
    #[inline(always)]
    pub fn eifclr12(&mut self) -> Eifclr12W<'_, EifcrSpec> {
        Eifclr12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc EIFCLR13"]
    #[inline(always)]
    pub fn eifclr13(&mut self) -> Eifclr13W<'_, EifcrSpec> {
        Eifclr13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc EIFCLR14"]
    #[inline(always)]
    pub fn eifclr14(&mut self) -> Eifclr14W<'_, EifcrSpec> {
        Eifclr14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc EIFCLR15"]
    #[inline(always)]
    pub fn eifclr15(&mut self) -> Eifclr15W<'_, EifcrSpec> {
        Eifclr15W::new(self, 15)
    }
}
#[doc = "desc EIFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EifcrSpec;
impl crate::RegisterSpec for EifcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eifcr::R`](R) reader structure"]
impl crate::Readable for EifcrSpec {}
#[doc = "`write(|w| ..)` method takes [`eifcr::W`](W) writer structure"]
impl crate::Writable for EifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EIFCR to value 0"]
impl crate::Resettable for EifcrSpec {}
