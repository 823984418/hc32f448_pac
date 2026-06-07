#[doc = "Register `AVCHSELR` reader"]
pub type R = crate::R<AvchselrSpec>;
#[doc = "Register `AVCHSELR` writer"]
pub type W = crate::W<AvchselrSpec>;
#[doc = "Field `AVCHSEL` reader - desc AVCHSEL"]
pub type AvchselR = crate::FieldReader<u16>;
#[doc = "Field `AVCHSEL` writer - desc AVCHSEL"]
pub type AvchselW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc AVCHSEL"]
    #[inline(always)]
    pub fn avchsel(&self) -> AvchselR {
        AvchselR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AVCHSELR")
            .field("avchsel", &self.avchsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - desc AVCHSEL"]
    #[inline(always)]
    pub fn avchsel(&mut self) -> AvchselW<'_, AvchselrSpec> {
        AvchselW::new(self, 0)
    }
}
#[doc = "desc AVCHSELR\n\nYou can [`read`](crate::Reg::read) this register and get [`avchselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avchselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvchselrSpec;
impl crate::RegisterSpec for AvchselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avchselr::R`](R) reader structure"]
impl crate::Readable for AvchselrSpec {}
#[doc = "`write(|w| ..)` method takes [`avchselr::W`](W) writer structure"]
impl crate::Writable for AvchselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AVCHSELR to value 0"]
impl crate::Resettable for AvchselrSpec {}
