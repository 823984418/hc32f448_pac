#[doc = "Register `HCPAR` reader"]
pub type R = crate::R<HcparSpec>;
#[doc = "Register `HCPAR` writer"]
pub type W = crate::W<HcparSpec>;
#[doc = "Field `HCPA0` reader - desc HCPA0"]
pub type Hcpa0R = crate::BitReader;
#[doc = "Field `HCPA0` writer - desc HCPA0"]
pub type Hcpa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA1` reader - desc HCPA1"]
pub type Hcpa1R = crate::BitReader;
#[doc = "Field `HCPA1` writer - desc HCPA1"]
pub type Hcpa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA2` reader - desc HCPA2"]
pub type Hcpa2R = crate::BitReader;
#[doc = "Field `HCPA2` writer - desc HCPA2"]
pub type Hcpa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA3` reader - desc HCPA3"]
pub type Hcpa3R = crate::BitReader;
#[doc = "Field `HCPA3` writer - desc HCPA3"]
pub type Hcpa3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA8` reader - desc HCPA8"]
pub type Hcpa8R = crate::BitReader;
#[doc = "Field `HCPA8` writer - desc HCPA8"]
pub type Hcpa8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA9` reader - desc HCPA9"]
pub type Hcpa9R = crate::BitReader;
#[doc = "Field `HCPA9` writer - desc HCPA9"]
pub type Hcpa9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA16` reader - desc HCPA16"]
pub type Hcpa16R = crate::BitReader;
#[doc = "Field `HCPA16` writer - desc HCPA16"]
pub type Hcpa16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA17` reader - desc HCPA17"]
pub type Hcpa17R = crate::BitReader;
#[doc = "Field `HCPA17` writer - desc HCPA17"]
pub type Hcpa17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA18` reader - desc HCPA18"]
pub type Hcpa18R = crate::BitReader;
#[doc = "Field `HCPA18` writer - desc HCPA18"]
pub type Hcpa18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA19` reader - desc HCPA19"]
pub type Hcpa19R = crate::BitReader;
#[doc = "Field `HCPA19` writer - desc HCPA19"]
pub type Hcpa19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA24` reader - desc HCPA24"]
pub type Hcpa24R = crate::BitReader;
#[doc = "Field `HCPA24` writer - desc HCPA24"]
pub type Hcpa24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPA25` reader - desc HCPA25"]
pub type Hcpa25R = crate::BitReader;
#[doc = "Field `HCPA25` writer - desc HCPA25"]
pub type Hcpa25W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCPA0"]
    #[inline(always)]
    pub fn hcpa0(&self) -> Hcpa0R {
        Hcpa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCPA1"]
    #[inline(always)]
    pub fn hcpa1(&self) -> Hcpa1R {
        Hcpa1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCPA2"]
    #[inline(always)]
    pub fn hcpa2(&self) -> Hcpa2R {
        Hcpa2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCPA3"]
    #[inline(always)]
    pub fn hcpa3(&self) -> Hcpa3R {
        Hcpa3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCPA8"]
    #[inline(always)]
    pub fn hcpa8(&self) -> Hcpa8R {
        Hcpa8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCPA9"]
    #[inline(always)]
    pub fn hcpa9(&self) -> Hcpa9R {
        Hcpa9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCPA16"]
    #[inline(always)]
    pub fn hcpa16(&self) -> Hcpa16R {
        Hcpa16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCPA17"]
    #[inline(always)]
    pub fn hcpa17(&self) -> Hcpa17R {
        Hcpa17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HCPA18"]
    #[inline(always)]
    pub fn hcpa18(&self) -> Hcpa18R {
        Hcpa18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HCPA19"]
    #[inline(always)]
    pub fn hcpa19(&self) -> Hcpa19R {
        Hcpa19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - desc HCPA24"]
    #[inline(always)]
    pub fn hcpa24(&self) -> Hcpa24R {
        Hcpa24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc HCPA25"]
    #[inline(always)]
    pub fn hcpa25(&self) -> Hcpa25R {
        Hcpa25R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCPAR")
            .field("hcpa0", &self.hcpa0())
            .field("hcpa1", &self.hcpa1())
            .field("hcpa2", &self.hcpa2())
            .field("hcpa3", &self.hcpa3())
            .field("hcpa8", &self.hcpa8())
            .field("hcpa9", &self.hcpa9())
            .field("hcpa16", &self.hcpa16())
            .field("hcpa17", &self.hcpa17())
            .field("hcpa18", &self.hcpa18())
            .field("hcpa19", &self.hcpa19())
            .field("hcpa24", &self.hcpa24())
            .field("hcpa25", &self.hcpa25())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HCPA0"]
    #[inline(always)]
    pub fn hcpa0(&mut self) -> Hcpa0W<'_, HcparSpec> {
        Hcpa0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCPA1"]
    #[inline(always)]
    pub fn hcpa1(&mut self) -> Hcpa1W<'_, HcparSpec> {
        Hcpa1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HCPA2"]
    #[inline(always)]
    pub fn hcpa2(&mut self) -> Hcpa2W<'_, HcparSpec> {
        Hcpa2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HCPA3"]
    #[inline(always)]
    pub fn hcpa3(&mut self) -> Hcpa3W<'_, HcparSpec> {
        Hcpa3W::new(self, 3)
    }
    #[doc = "Bit 8 - desc HCPA8"]
    #[inline(always)]
    pub fn hcpa8(&mut self) -> Hcpa8W<'_, HcparSpec> {
        Hcpa8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCPA9"]
    #[inline(always)]
    pub fn hcpa9(&mut self) -> Hcpa9W<'_, HcparSpec> {
        Hcpa9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HCPA16"]
    #[inline(always)]
    pub fn hcpa16(&mut self) -> Hcpa16W<'_, HcparSpec> {
        Hcpa16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HCPA17"]
    #[inline(always)]
    pub fn hcpa17(&mut self) -> Hcpa17W<'_, HcparSpec> {
        Hcpa17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HCPA18"]
    #[inline(always)]
    pub fn hcpa18(&mut self) -> Hcpa18W<'_, HcparSpec> {
        Hcpa18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HCPA19"]
    #[inline(always)]
    pub fn hcpa19(&mut self) -> Hcpa19W<'_, HcparSpec> {
        Hcpa19W::new(self, 19)
    }
    #[doc = "Bit 24 - desc HCPA24"]
    #[inline(always)]
    pub fn hcpa24(&mut self) -> Hcpa24W<'_, HcparSpec> {
        Hcpa24W::new(self, 24)
    }
    #[doc = "Bit 25 - desc HCPA25"]
    #[inline(always)]
    pub fn hcpa25(&mut self) -> Hcpa25W<'_, HcparSpec> {
        Hcpa25W::new(self, 25)
    }
}
#[doc = "desc HCPAR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcparSpec;
impl crate::RegisterSpec for HcparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcpar::R`](R) reader structure"]
impl crate::Readable for HcparSpec {}
#[doc = "`write(|w| ..)` method takes [`hcpar::W`](W) writer structure"]
impl crate::Writable for HcparSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCPAR to value 0"]
impl crate::Resettable for HcparSpec {}
