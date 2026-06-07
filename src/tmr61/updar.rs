#[doc = "Register `UPDAR` reader"]
pub type R = crate::R<UpdarSpec>;
#[doc = "Register `UPDAR` writer"]
pub type W = crate::W<UpdarSpec>;
#[doc = "Field `UPDA` reader - desc UPDA"]
pub type UpdaR = crate::FieldReader<u16>;
#[doc = "Field `UPDA` writer - desc UPDA"]
pub type UpdaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc UPDA"]
    #[inline(always)]
    pub fn upda(&self) -> UpdaR {
        UpdaR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDAR").field("upda", &self.upda()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - desc UPDA"]
    #[inline(always)]
    pub fn upda(&mut self) -> UpdaW<'_, UpdarSpec> {
        UpdaW::new(self, 0)
    }
}
#[doc = "desc UPDAR\n\nYou can [`read`](crate::Reg::read) this register and get [`updar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`updar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpdarSpec;
impl crate::RegisterSpec for UpdarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`updar::R`](R) reader structure"]
impl crate::Readable for UpdarSpec {}
#[doc = "`write(|w| ..)` method takes [`updar::W`](W) writer structure"]
impl crate::Writable for UpdarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPDAR to value 0"]
impl crate::Resettable for UpdarSpec {}
