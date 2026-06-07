#[doc = "Register `HCUPR` reader"]
pub type R = crate::R<HcuprSpec>;
#[doc = "Register `HCUPR` writer"]
pub type W = crate::W<HcuprSpec>;
#[doc = "Field `HCUP0` reader - desc HCUP0"]
pub type Hcup0R = crate::BitReader;
#[doc = "Field `HCUP0` writer - desc HCUP0"]
pub type Hcup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP1` reader - desc HCUP1"]
pub type Hcup1R = crate::BitReader;
#[doc = "Field `HCUP1` writer - desc HCUP1"]
pub type Hcup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP2` reader - desc HCUP2"]
pub type Hcup2R = crate::BitReader;
#[doc = "Field `HCUP2` writer - desc HCUP2"]
pub type Hcup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP3` reader - desc HCUP3"]
pub type Hcup3R = crate::BitReader;
#[doc = "Field `HCUP3` writer - desc HCUP3"]
pub type Hcup3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP4` reader - desc HCUP4"]
pub type Hcup4R = crate::BitReader;
#[doc = "Field `HCUP4` writer - desc HCUP4"]
pub type Hcup4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP5` reader - desc HCUP5"]
pub type Hcup5R = crate::BitReader;
#[doc = "Field `HCUP5` writer - desc HCUP5"]
pub type Hcup5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP6` reader - desc HCUP6"]
pub type Hcup6R = crate::BitReader;
#[doc = "Field `HCUP6` writer - desc HCUP6"]
pub type Hcup6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP7` reader - desc HCUP7"]
pub type Hcup7R = crate::BitReader;
#[doc = "Field `HCUP7` writer - desc HCUP7"]
pub type Hcup7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP8` reader - desc HCUP8"]
pub type Hcup8R = crate::BitReader;
#[doc = "Field `HCUP8` writer - desc HCUP8"]
pub type Hcup8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP9` reader - desc HCUP9"]
pub type Hcup9R = crate::BitReader;
#[doc = "Field `HCUP9` writer - desc HCUP9"]
pub type Hcup9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP16` reader - desc HCUP16"]
pub type Hcup16R = crate::BitReader;
#[doc = "Field `HCUP16` writer - desc HCUP16"]
pub type Hcup16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP17` reader - desc HCUP17"]
pub type Hcup17R = crate::BitReader;
#[doc = "Field `HCUP17` writer - desc HCUP17"]
pub type Hcup17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP18` reader - desc HCUP18"]
pub type Hcup18R = crate::BitReader;
#[doc = "Field `HCUP18` writer - desc HCUP18"]
pub type Hcup18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCUP19` reader - desc HCUP19"]
pub type Hcup19R = crate::BitReader;
#[doc = "Field `HCUP19` writer - desc HCUP19"]
pub type Hcup19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCUP0"]
    #[inline(always)]
    pub fn hcup0(&self) -> Hcup0R {
        Hcup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCUP1"]
    #[inline(always)]
    pub fn hcup1(&self) -> Hcup1R {
        Hcup1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCUP2"]
    #[inline(always)]
    pub fn hcup2(&self) -> Hcup2R {
        Hcup2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCUP3"]
    #[inline(always)]
    pub fn hcup3(&self) -> Hcup3R {
        Hcup3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCUP4"]
    #[inline(always)]
    pub fn hcup4(&self) -> Hcup4R {
        Hcup4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCUP5"]
    #[inline(always)]
    pub fn hcup5(&self) -> Hcup5R {
        Hcup5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCUP6"]
    #[inline(always)]
    pub fn hcup6(&self) -> Hcup6R {
        Hcup6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCUP7"]
    #[inline(always)]
    pub fn hcup7(&self) -> Hcup7R {
        Hcup7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCUP8"]
    #[inline(always)]
    pub fn hcup8(&self) -> Hcup8R {
        Hcup8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCUP9"]
    #[inline(always)]
    pub fn hcup9(&self) -> Hcup9R {
        Hcup9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCUP16"]
    #[inline(always)]
    pub fn hcup16(&self) -> Hcup16R {
        Hcup16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCUP17"]
    #[inline(always)]
    pub fn hcup17(&self) -> Hcup17R {
        Hcup17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HCUP18"]
    #[inline(always)]
    pub fn hcup18(&self) -> Hcup18R {
        Hcup18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HCUP19"]
    #[inline(always)]
    pub fn hcup19(&self) -> Hcup19R {
        Hcup19R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCUPR")
            .field("hcup0", &self.hcup0())
            .field("hcup1", &self.hcup1())
            .field("hcup2", &self.hcup2())
            .field("hcup3", &self.hcup3())
            .field("hcup4", &self.hcup4())
            .field("hcup5", &self.hcup5())
            .field("hcup6", &self.hcup6())
            .field("hcup7", &self.hcup7())
            .field("hcup8", &self.hcup8())
            .field("hcup9", &self.hcup9())
            .field("hcup16", &self.hcup16())
            .field("hcup17", &self.hcup17())
            .field("hcup18", &self.hcup18())
            .field("hcup19", &self.hcup19())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HCUP0"]
    #[inline(always)]
    pub fn hcup0(&mut self) -> Hcup0W<'_, HcuprSpec> {
        Hcup0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCUP1"]
    #[inline(always)]
    pub fn hcup1(&mut self) -> Hcup1W<'_, HcuprSpec> {
        Hcup1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HCUP2"]
    #[inline(always)]
    pub fn hcup2(&mut self) -> Hcup2W<'_, HcuprSpec> {
        Hcup2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HCUP3"]
    #[inline(always)]
    pub fn hcup3(&mut self) -> Hcup3W<'_, HcuprSpec> {
        Hcup3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc HCUP4"]
    #[inline(always)]
    pub fn hcup4(&mut self) -> Hcup4W<'_, HcuprSpec> {
        Hcup4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HCUP5"]
    #[inline(always)]
    pub fn hcup5(&mut self) -> Hcup5W<'_, HcuprSpec> {
        Hcup5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HCUP6"]
    #[inline(always)]
    pub fn hcup6(&mut self) -> Hcup6W<'_, HcuprSpec> {
        Hcup6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HCUP7"]
    #[inline(always)]
    pub fn hcup7(&mut self) -> Hcup7W<'_, HcuprSpec> {
        Hcup7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HCUP8"]
    #[inline(always)]
    pub fn hcup8(&mut self) -> Hcup8W<'_, HcuprSpec> {
        Hcup8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCUP9"]
    #[inline(always)]
    pub fn hcup9(&mut self) -> Hcup9W<'_, HcuprSpec> {
        Hcup9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HCUP16"]
    #[inline(always)]
    pub fn hcup16(&mut self) -> Hcup16W<'_, HcuprSpec> {
        Hcup16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HCUP17"]
    #[inline(always)]
    pub fn hcup17(&mut self) -> Hcup17W<'_, HcuprSpec> {
        Hcup17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HCUP18"]
    #[inline(always)]
    pub fn hcup18(&mut self) -> Hcup18W<'_, HcuprSpec> {
        Hcup18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HCUP19"]
    #[inline(always)]
    pub fn hcup19(&mut self) -> Hcup19W<'_, HcuprSpec> {
        Hcup19W::new(self, 19)
    }
}
#[doc = "desc HCUPR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcupr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcupr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcuprSpec;
impl crate::RegisterSpec for HcuprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcupr::R`](R) reader structure"]
impl crate::Readable for HcuprSpec {}
#[doc = "`write(|w| ..)` method takes [`hcupr::W`](W) writer structure"]
impl crate::Writable for HcuprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCUPR to value 0"]
impl crate::Resettable for HcuprSpec {}
