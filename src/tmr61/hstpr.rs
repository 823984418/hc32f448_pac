#[doc = "Register `HSTPR` reader"]
pub type R = crate::R<HstprSpec>;
#[doc = "Register `HSTPR` writer"]
pub type W = crate::W<HstprSpec>;
#[doc = "Field `HSTP0` reader - desc HSTP0"]
pub type Hstp0R = crate::BitReader;
#[doc = "Field `HSTP0` writer - desc HSTP0"]
pub type Hstp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP1` reader - desc HSTP1"]
pub type Hstp1R = crate::BitReader;
#[doc = "Field `HSTP1` writer - desc HSTP1"]
pub type Hstp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP2` reader - desc HSTP2"]
pub type Hstp2R = crate::BitReader;
#[doc = "Field `HSTP2` writer - desc HSTP2"]
pub type Hstp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP3` reader - desc HSTP3"]
pub type Hstp3R = crate::BitReader;
#[doc = "Field `HSTP3` writer - desc HSTP3"]
pub type Hstp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPS` reader - desc STPS"]
pub type StpsR = crate::BitReader;
#[doc = "Field `STPS` writer - desc STPS"]
pub type StpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP8` reader - desc HSTP8"]
pub type Hstp8R = crate::BitReader;
#[doc = "Field `HSTP8` writer - desc HSTP8"]
pub type Hstp8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP9` reader - desc HSTP9"]
pub type Hstp9R = crate::BitReader;
#[doc = "Field `HSTP9` writer - desc HSTP9"]
pub type Hstp9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP16` reader - desc HSTP16"]
pub type Hstp16R = crate::BitReader;
#[doc = "Field `HSTP16` writer - desc HSTP16"]
pub type Hstp16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP17` reader - desc HSTP17"]
pub type Hstp17R = crate::BitReader;
#[doc = "Field `HSTP17` writer - desc HSTP17"]
pub type Hstp17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP18` reader - desc HSTP18"]
pub type Hstp18R = crate::BitReader;
#[doc = "Field `HSTP18` writer - desc HSTP18"]
pub type Hstp18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP19` reader - desc HSTP19"]
pub type Hstp19R = crate::BitReader;
#[doc = "Field `HSTP19` writer - desc HSTP19"]
pub type Hstp19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&self) -> Hstp0R {
        Hstp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&self) -> Hstp1R {
        Hstp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSTP2"]
    #[inline(always)]
    pub fn hstp2(&self) -> Hstp2R {
        Hstp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HSTP3"]
    #[inline(always)]
    pub fn hstp3(&self) -> Hstp3R {
        Hstp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - desc STPS"]
    #[inline(always)]
    pub fn stps(&self) -> StpsR {
        StpsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSTP8"]
    #[inline(always)]
    pub fn hstp8(&self) -> Hstp8R {
        Hstp8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HSTP9"]
    #[inline(always)]
    pub fn hstp9(&self) -> Hstp9R {
        Hstp9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HSTP16"]
    #[inline(always)]
    pub fn hstp16(&self) -> Hstp16R {
        Hstp16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HSTP17"]
    #[inline(always)]
    pub fn hstp17(&self) -> Hstp17R {
        Hstp17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HSTP18"]
    #[inline(always)]
    pub fn hstp18(&self) -> Hstp18R {
        Hstp18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HSTP19"]
    #[inline(always)]
    pub fn hstp19(&self) -> Hstp19R {
        Hstp19R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSTPR")
            .field("hstp0", &self.hstp0())
            .field("hstp1", &self.hstp1())
            .field("hstp2", &self.hstp2())
            .field("hstp3", &self.hstp3())
            .field("stps", &self.stps())
            .field("hstp8", &self.hstp8())
            .field("hstp9", &self.hstp9())
            .field("hstp16", &self.hstp16())
            .field("hstp17", &self.hstp17())
            .field("hstp18", &self.hstp18())
            .field("hstp19", &self.hstp19())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&mut self) -> Hstp0W<'_, HstprSpec> {
        Hstp0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&mut self) -> Hstp1W<'_, HstprSpec> {
        Hstp1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HSTP2"]
    #[inline(always)]
    pub fn hstp2(&mut self) -> Hstp2W<'_, HstprSpec> {
        Hstp2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HSTP3"]
    #[inline(always)]
    pub fn hstp3(&mut self) -> Hstp3W<'_, HstprSpec> {
        Hstp3W::new(self, 3)
    }
    #[doc = "Bit 7 - desc STPS"]
    #[inline(always)]
    pub fn stps(&mut self) -> StpsW<'_, HstprSpec> {
        StpsW::new(self, 7)
    }
    #[doc = "Bit 8 - desc HSTP8"]
    #[inline(always)]
    pub fn hstp8(&mut self) -> Hstp8W<'_, HstprSpec> {
        Hstp8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HSTP9"]
    #[inline(always)]
    pub fn hstp9(&mut self) -> Hstp9W<'_, HstprSpec> {
        Hstp9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HSTP16"]
    #[inline(always)]
    pub fn hstp16(&mut self) -> Hstp16W<'_, HstprSpec> {
        Hstp16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HSTP17"]
    #[inline(always)]
    pub fn hstp17(&mut self) -> Hstp17W<'_, HstprSpec> {
        Hstp17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HSTP18"]
    #[inline(always)]
    pub fn hstp18(&mut self) -> Hstp18W<'_, HstprSpec> {
        Hstp18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HSTP19"]
    #[inline(always)]
    pub fn hstp19(&mut self) -> Hstp19W<'_, HstprSpec> {
        Hstp19W::new(self, 19)
    }
}
#[doc = "desc HSTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstprSpec;
impl crate::RegisterSpec for HstprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpr::R`](R) reader structure"]
impl crate::Readable for HstprSpec {}
#[doc = "`write(|w| ..)` method takes [`hstpr::W`](W) writer structure"]
impl crate::Writable for HstprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTPR to value 0"]
impl crate::Resettable for HstprSpec {}
