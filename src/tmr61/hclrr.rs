#[doc = "Register `HCLRR` reader"]
pub type R = crate::R<HclrrSpec>;
#[doc = "Register `HCLRR` writer"]
pub type W = crate::W<HclrrSpec>;
#[doc = "Field `HCLE0` reader - desc HCLE0"]
pub type Hcle0R = crate::BitReader;
#[doc = "Field `HCLE0` writer - desc HCLE0"]
pub type Hcle0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE1` reader - desc HCLE1"]
pub type Hcle1R = crate::BitReader;
#[doc = "Field `HCLE1` writer - desc HCLE1"]
pub type Hcle1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE2` reader - desc HCLE2"]
pub type Hcle2R = crate::BitReader;
#[doc = "Field `HCLE2` writer - desc HCLE2"]
pub type Hcle2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE3` reader - desc HCLE3"]
pub type Hcle3R = crate::BitReader;
#[doc = "Field `HCLE3` writer - desc HCLE3"]
pub type Hcle3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLES` reader - desc CLES"]
pub type ClesR = crate::BitReader;
#[doc = "Field `CLES` writer - desc CLES"]
pub type ClesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE8` reader - desc HCLE8"]
pub type Hcle8R = crate::BitReader;
#[doc = "Field `HCLE8` writer - desc HCLE8"]
pub type Hcle8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE9` reader - desc HCLE9"]
pub type Hcle9R = crate::BitReader;
#[doc = "Field `HCLE9` writer - desc HCLE9"]
pub type Hcle9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE16` reader - desc HCLE16"]
pub type Hcle16R = crate::BitReader;
#[doc = "Field `HCLE16` writer - desc HCLE16"]
pub type Hcle16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE17` reader - desc HCLE17"]
pub type Hcle17R = crate::BitReader;
#[doc = "Field `HCLE17` writer - desc HCLE17"]
pub type Hcle17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE18` reader - desc HCLE18"]
pub type Hcle18R = crate::BitReader;
#[doc = "Field `HCLE18` writer - desc HCLE18"]
pub type Hcle18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE19` reader - desc HCLE19"]
pub type Hcle19R = crate::BitReader;
#[doc = "Field `HCLE19` writer - desc HCLE19"]
pub type Hcle19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCLE0"]
    #[inline(always)]
    pub fn hcle0(&self) -> Hcle0R {
        Hcle0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCLE1"]
    #[inline(always)]
    pub fn hcle1(&self) -> Hcle1R {
        Hcle1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCLE2"]
    #[inline(always)]
    pub fn hcle2(&self) -> Hcle2R {
        Hcle2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCLE3"]
    #[inline(always)]
    pub fn hcle3(&self) -> Hcle3R {
        Hcle3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CLES"]
    #[inline(always)]
    pub fn cles(&self) -> ClesR {
        ClesR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCLE8"]
    #[inline(always)]
    pub fn hcle8(&self) -> Hcle8R {
        Hcle8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCLE9"]
    #[inline(always)]
    pub fn hcle9(&self) -> Hcle9R {
        Hcle9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCLE16"]
    #[inline(always)]
    pub fn hcle16(&self) -> Hcle16R {
        Hcle16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCLE17"]
    #[inline(always)]
    pub fn hcle17(&self) -> Hcle17R {
        Hcle17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HCLE18"]
    #[inline(always)]
    pub fn hcle18(&self) -> Hcle18R {
        Hcle18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HCLE19"]
    #[inline(always)]
    pub fn hcle19(&self) -> Hcle19R {
        Hcle19R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCLRR")
            .field("hcle0", &self.hcle0())
            .field("hcle1", &self.hcle1())
            .field("hcle2", &self.hcle2())
            .field("hcle3", &self.hcle3())
            .field("cles", &self.cles())
            .field("hcle8", &self.hcle8())
            .field("hcle9", &self.hcle9())
            .field("hcle16", &self.hcle16())
            .field("hcle17", &self.hcle17())
            .field("hcle18", &self.hcle18())
            .field("hcle19", &self.hcle19())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HCLE0"]
    #[inline(always)]
    pub fn hcle0(&mut self) -> Hcle0W<'_, HclrrSpec> {
        Hcle0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCLE1"]
    #[inline(always)]
    pub fn hcle1(&mut self) -> Hcle1W<'_, HclrrSpec> {
        Hcle1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HCLE2"]
    #[inline(always)]
    pub fn hcle2(&mut self) -> Hcle2W<'_, HclrrSpec> {
        Hcle2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HCLE3"]
    #[inline(always)]
    pub fn hcle3(&mut self) -> Hcle3W<'_, HclrrSpec> {
        Hcle3W::new(self, 3)
    }
    #[doc = "Bit 7 - desc CLES"]
    #[inline(always)]
    pub fn cles(&mut self) -> ClesW<'_, HclrrSpec> {
        ClesW::new(self, 7)
    }
    #[doc = "Bit 8 - desc HCLE8"]
    #[inline(always)]
    pub fn hcle8(&mut self) -> Hcle8W<'_, HclrrSpec> {
        Hcle8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCLE9"]
    #[inline(always)]
    pub fn hcle9(&mut self) -> Hcle9W<'_, HclrrSpec> {
        Hcle9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HCLE16"]
    #[inline(always)]
    pub fn hcle16(&mut self) -> Hcle16W<'_, HclrrSpec> {
        Hcle16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HCLE17"]
    #[inline(always)]
    pub fn hcle17(&mut self) -> Hcle17W<'_, HclrrSpec> {
        Hcle17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HCLE18"]
    #[inline(always)]
    pub fn hcle18(&mut self) -> Hcle18W<'_, HclrrSpec> {
        Hcle18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HCLE19"]
    #[inline(always)]
    pub fn hcle19(&mut self) -> Hcle19W<'_, HclrrSpec> {
        Hcle19W::new(self, 19)
    }
}
#[doc = "desc HCLRR\n\nYou can [`read`](crate::Reg::read) this register and get [`hclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HclrrSpec;
impl crate::RegisterSpec for HclrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hclrr::R`](R) reader structure"]
impl crate::Readable for HclrrSpec {}
#[doc = "`write(|w| ..)` method takes [`hclrr::W`](W) writer structure"]
impl crate::Writable for HclrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCLRR to value 0"]
impl crate::Resettable for HclrrSpec {}
