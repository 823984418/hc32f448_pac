#[doc = "Register `AWD1CHSR` reader"]
pub type R = crate::R<Awd1chsrSpec>;
#[doc = "Register `AWD1CHSR` writer"]
pub type W = crate::W<Awd1chsrSpec>;
#[doc = "Field `AWDCH` reader - desc AWDCH"]
pub type AwdchR = crate::FieldReader;
#[doc = "Field `AWDCH` writer - desc AWDCH"]
pub type AwdchW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&self) -> AwdchR {
        AwdchR::new(self.bits & 0x1f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD1CHSR")
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AwdchW<'_, Awd1chsrSpec> {
        AwdchW::new(self, 0)
    }
}
#[doc = "desc AWD1CHSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awd1chsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1chsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd1chsrSpec;
impl crate::RegisterSpec for Awd1chsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awd1chsr::R`](R) reader structure"]
impl crate::Readable for Awd1chsrSpec {}
#[doc = "`write(|w| ..)` method takes [`awd1chsr::W`](W) writer structure"]
impl crate::Writable for Awd1chsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWD1CHSR to value 0"]
impl crate::Resettable for Awd1chsrSpec {}
