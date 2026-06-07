#[doc = "Register `PWRC4` reader"]
pub type R = crate::R<Pwrc4Spec>;
#[doc = "Register `PWRC4` writer"]
pub type W = crate::W<Pwrc4Spec>;
#[doc = "Field `ADBUFE` reader - desc ADBUFE"]
pub type AdbufeR = crate::BitReader;
#[doc = "Field `ADBUFE` writer - desc ADBUFE"]
pub type AdbufeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - desc ADBUFE"]
    #[inline(always)]
    pub fn adbufe(&self) -> AdbufeR {
        AdbufeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC4")
            .field("adbufe", &self.adbufe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - desc ADBUFE"]
    #[inline(always)]
    pub fn adbufe(&mut self) -> AdbufeW<'_, Pwrc4Spec> {
        AdbufeW::new(self, 7)
    }
}
#[doc = "desc PWRC4\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc4Spec;
impl crate::RegisterSpec for Pwrc4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc4::R`](R) reader structure"]
impl crate::Readable for Pwrc4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc4::W`](W) writer structure"]
impl crate::Writable for Pwrc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC4 to value 0"]
impl crate::Resettable for Pwrc4Spec {}
