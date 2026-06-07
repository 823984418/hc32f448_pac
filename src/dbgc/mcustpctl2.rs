#[doc = "Register `MCUSTPCTL2` reader"]
pub type R = crate::R<Mcustpctl2Spec>;
#[doc = "Register `MCUSTPCTL2` writer"]
pub type W = crate::W<Mcustpctl2Spec>;
#[doc = "Field `M32STP` reader - desc M32STP"]
pub type M32stpR = crate::BitReader;
#[doc = "Field `M32STP` writer - desc M32STP"]
pub type M32stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M33STP` reader - desc M33STP"]
pub type M33stpR = crate::BitReader;
#[doc = "Field `M33STP` writer - desc M33STP"]
pub type M33stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M34STP` reader - desc M34STP"]
pub type M34stpR = crate::BitReader;
#[doc = "Field `M34STP` writer - desc M34STP"]
pub type M34stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M35STP` reader - desc M35STP"]
pub type M35stpR = crate::BitReader;
#[doc = "Field `M35STP` writer - desc M35STP"]
pub type M35stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M36STP` reader - desc M36STP"]
pub type M36stpR = crate::BitReader;
#[doc = "Field `M36STP` writer - desc M36STP"]
pub type M36stpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc M32STP"]
    #[inline(always)]
    pub fn m32stp(&self) -> M32stpR {
        M32stpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc M33STP"]
    #[inline(always)]
    pub fn m33stp(&self) -> M33stpR {
        M33stpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc M34STP"]
    #[inline(always)]
    pub fn m34stp(&self) -> M34stpR {
        M34stpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc M35STP"]
    #[inline(always)]
    pub fn m35stp(&self) -> M35stpR {
        M35stpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc M36STP"]
    #[inline(always)]
    pub fn m36stp(&self) -> M36stpR {
        M36stpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUSTPCTL2")
            .field("m32stp", &self.m32stp())
            .field("m33stp", &self.m33stp())
            .field("m34stp", &self.m34stp())
            .field("m35stp", &self.m35stp())
            .field("m36stp", &self.m36stp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc M32STP"]
    #[inline(always)]
    pub fn m32stp(&mut self) -> M32stpW<'_, Mcustpctl2Spec> {
        M32stpW::new(self, 0)
    }
    #[doc = "Bit 1 - desc M33STP"]
    #[inline(always)]
    pub fn m33stp(&mut self) -> M33stpW<'_, Mcustpctl2Spec> {
        M33stpW::new(self, 1)
    }
    #[doc = "Bit 2 - desc M34STP"]
    #[inline(always)]
    pub fn m34stp(&mut self) -> M34stpW<'_, Mcustpctl2Spec> {
        M34stpW::new(self, 2)
    }
    #[doc = "Bit 3 - desc M35STP"]
    #[inline(always)]
    pub fn m35stp(&mut self) -> M35stpW<'_, Mcustpctl2Spec> {
        M35stpW::new(self, 3)
    }
    #[doc = "Bit 4 - desc M36STP"]
    #[inline(always)]
    pub fn m36stp(&mut self) -> M36stpW<'_, Mcustpctl2Spec> {
        M36stpW::new(self, 4)
    }
}
#[doc = "desc MCUSTPCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustpctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustpctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcustpctl2Spec;
impl crate::RegisterSpec for Mcustpctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcustpctl2::R`](R) reader structure"]
impl crate::Readable for Mcustpctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcustpctl2::W`](W) writer structure"]
impl crate::Writable for Mcustpctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUSTPCTL2 to value 0"]
impl crate::Resettable for Mcustpctl2Spec {}
