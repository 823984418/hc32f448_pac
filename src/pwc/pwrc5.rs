#[doc = "Register `PWRC5` reader"]
pub type R = crate::R<Pwrc5Spec>;
#[doc = "Register `PWRC5` writer"]
pub type W = crate::W<Pwrc5Spec>;
#[doc = "Field `VVDRSD` reader - desc VVDRSD"]
pub type VvdrsdR = crate::BitReader;
#[doc = "Field `VVDRSD` writer - desc VVDRSD"]
pub type VvdrsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMBSD` reader - desc SRAMBSD"]
pub type SrambsdR = crate::BitReader;
#[doc = "Field `SRAMBSD` writer - desc SRAMBSD"]
pub type SrambsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDIS` reader - desc CSDIS"]
pub type CsdisR = crate::BitReader;
#[doc = "Field `CSDIS` writer - desc CSDIS"]
pub type CsdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc VVDRSD"]
    #[inline(always)]
    pub fn vvdrsd(&self) -> VvdrsdR {
        VvdrsdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SRAMBSD"]
    #[inline(always)]
    pub fn srambsd(&self) -> SrambsdR {
        SrambsdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CSDIS"]
    #[inline(always)]
    pub fn csdis(&self) -> CsdisR {
        CsdisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC5")
            .field("vvdrsd", &self.vvdrsd())
            .field("srambsd", &self.srambsd())
            .field("csdis", &self.csdis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc VVDRSD"]
    #[inline(always)]
    pub fn vvdrsd(&mut self) -> VvdrsdW<'_, Pwrc5Spec> {
        VvdrsdW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SRAMBSD"]
    #[inline(always)]
    pub fn srambsd(&mut self) -> SrambsdW<'_, Pwrc5Spec> {
        SrambsdW::new(self, 1)
    }
    #[doc = "Bit 7 - desc CSDIS"]
    #[inline(always)]
    pub fn csdis(&mut self) -> CsdisW<'_, Pwrc5Spec> {
        CsdisW::new(self, 7)
    }
}
#[doc = "desc PWRC5\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc5Spec;
impl crate::RegisterSpec for Pwrc5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc5::R`](R) reader structure"]
impl crate::Readable for Pwrc5Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc5::W`](W) writer structure"]
impl crate::Writable for Pwrc5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC5 to value 0"]
impl crate::Resettable for Pwrc5Spec {}
