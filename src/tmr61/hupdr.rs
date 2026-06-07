#[doc = "Register `HUPDR` reader"]
pub type R = crate::R<HupdrSpec>;
#[doc = "Register `HUPDR` writer"]
pub type W = crate::W<HupdrSpec>;
#[doc = "Field `HUPD0` reader - desc HUPD0"]
pub type Hupd0R = crate::BitReader;
#[doc = "Field `HUPD0` writer - desc HUPD0"]
pub type Hupd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD1` reader - desc HUPD1"]
pub type Hupd1R = crate::BitReader;
#[doc = "Field `HUPD1` writer - desc HUPD1"]
pub type Hupd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD2` reader - desc HUPD2"]
pub type Hupd2R = crate::BitReader;
#[doc = "Field `HUPD2` writer - desc HUPD2"]
pub type Hupd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD3` reader - desc HUPD3"]
pub type Hupd3R = crate::BitReader;
#[doc = "Field `HUPD3` writer - desc HUPD3"]
pub type Hupd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDS` reader - desc UPDS"]
pub type UpdsR = crate::BitReader;
#[doc = "Field `UPDS` writer - desc UPDS"]
pub type UpdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD8` reader - desc HUPD8"]
pub type Hupd8R = crate::BitReader;
#[doc = "Field `HUPD8` writer - desc HUPD8"]
pub type Hupd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD9` reader - desc HUPD9"]
pub type Hupd9R = crate::BitReader;
#[doc = "Field `HUPD9` writer - desc HUPD9"]
pub type Hupd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD16` reader - desc HUPD16"]
pub type Hupd16R = crate::BitReader;
#[doc = "Field `HUPD16` writer - desc HUPD16"]
pub type Hupd16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD17` reader - desc HUPD17"]
pub type Hupd17R = crate::BitReader;
#[doc = "Field `HUPD17` writer - desc HUPD17"]
pub type Hupd17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD18` reader - desc HUPD18"]
pub type Hupd18R = crate::BitReader;
#[doc = "Field `HUPD18` writer - desc HUPD18"]
pub type Hupd18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUPD19` reader - desc HUPD19"]
pub type Hupd19R = crate::BitReader;
#[doc = "Field `HUPD19` writer - desc HUPD19"]
pub type Hupd19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HUPD0"]
    #[inline(always)]
    pub fn hupd0(&self) -> Hupd0R {
        Hupd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HUPD1"]
    #[inline(always)]
    pub fn hupd1(&self) -> Hupd1R {
        Hupd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HUPD2"]
    #[inline(always)]
    pub fn hupd2(&self) -> Hupd2R {
        Hupd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HUPD3"]
    #[inline(always)]
    pub fn hupd3(&self) -> Hupd3R {
        Hupd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UPDS"]
    #[inline(always)]
    pub fn upds(&self) -> UpdsR {
        UpdsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HUPD8"]
    #[inline(always)]
    pub fn hupd8(&self) -> Hupd8R {
        Hupd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HUPD9"]
    #[inline(always)]
    pub fn hupd9(&self) -> Hupd9R {
        Hupd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HUPD16"]
    #[inline(always)]
    pub fn hupd16(&self) -> Hupd16R {
        Hupd16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HUPD17"]
    #[inline(always)]
    pub fn hupd17(&self) -> Hupd17R {
        Hupd17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HUPD18"]
    #[inline(always)]
    pub fn hupd18(&self) -> Hupd18R {
        Hupd18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HUPD19"]
    #[inline(always)]
    pub fn hupd19(&self) -> Hupd19R {
        Hupd19R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUPDR")
            .field("hupd0", &self.hupd0())
            .field("hupd1", &self.hupd1())
            .field("hupd2", &self.hupd2())
            .field("hupd3", &self.hupd3())
            .field("upds", &self.upds())
            .field("hupd8", &self.hupd8())
            .field("hupd9", &self.hupd9())
            .field("hupd16", &self.hupd16())
            .field("hupd17", &self.hupd17())
            .field("hupd18", &self.hupd18())
            .field("hupd19", &self.hupd19())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HUPD0"]
    #[inline(always)]
    pub fn hupd0(&mut self) -> Hupd0W<'_, HupdrSpec> {
        Hupd0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HUPD1"]
    #[inline(always)]
    pub fn hupd1(&mut self) -> Hupd1W<'_, HupdrSpec> {
        Hupd1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HUPD2"]
    #[inline(always)]
    pub fn hupd2(&mut self) -> Hupd2W<'_, HupdrSpec> {
        Hupd2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HUPD3"]
    #[inline(always)]
    pub fn hupd3(&mut self) -> Hupd3W<'_, HupdrSpec> {
        Hupd3W::new(self, 3)
    }
    #[doc = "Bit 7 - desc UPDS"]
    #[inline(always)]
    pub fn upds(&mut self) -> UpdsW<'_, HupdrSpec> {
        UpdsW::new(self, 7)
    }
    #[doc = "Bit 8 - desc HUPD8"]
    #[inline(always)]
    pub fn hupd8(&mut self) -> Hupd8W<'_, HupdrSpec> {
        Hupd8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HUPD9"]
    #[inline(always)]
    pub fn hupd9(&mut self) -> Hupd9W<'_, HupdrSpec> {
        Hupd9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HUPD16"]
    #[inline(always)]
    pub fn hupd16(&mut self) -> Hupd16W<'_, HupdrSpec> {
        Hupd16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HUPD17"]
    #[inline(always)]
    pub fn hupd17(&mut self) -> Hupd17W<'_, HupdrSpec> {
        Hupd17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HUPD18"]
    #[inline(always)]
    pub fn hupd18(&mut self) -> Hupd18W<'_, HupdrSpec> {
        Hupd18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HUPD19"]
    #[inline(always)]
    pub fn hupd19(&mut self) -> Hupd19W<'_, HupdrSpec> {
        Hupd19W::new(self, 19)
    }
}
#[doc = "desc HUPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HupdrSpec;
impl crate::RegisterSpec for HupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hupdr::R`](R) reader structure"]
impl crate::Readable for HupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`hupdr::W`](W) writer structure"]
impl crate::Writable for HupdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUPDR to value 0"]
impl crate::Resettable for HupdrSpec {}
