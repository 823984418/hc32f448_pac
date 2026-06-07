#[doc = "Register `PWRC2` reader"]
pub type R = crate::R<Pwrc2Spec>;
#[doc = "Register `PWRC2` writer"]
pub type W = crate::W<Pwrc2Spec>;
#[doc = "Field `DVS` reader - desc DVS"]
pub type DvsR = crate::FieldReader;
#[doc = "Field `DVS` writer - desc DVS"]
pub type DvsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - desc DVS"]
    #[inline(always)]
    pub fn dvs(&self) -> DvsR {
        DvsR::new((self.bits >> 4) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC2").field("dvs", &self.dvs()).finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - desc DVS"]
    #[inline(always)]
    pub fn dvs(&mut self) -> DvsW<'_, Pwrc2Spec> {
        DvsW::new(self, 4)
    }
}
#[doc = "desc PWRC2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc2Spec;
impl crate::RegisterSpec for Pwrc2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc2::R`](R) reader structure"]
impl crate::Readable for Pwrc2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc2::W`](W) writer structure"]
impl crate::Writable for Pwrc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC2 to value 0xf0"]
impl crate::Resettable for Pwrc2Spec {
    const RESET_VALUE: u8 = 0xf0;
}
