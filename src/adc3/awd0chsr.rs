#[doc = "Register `AWD0CHSR` reader"]
pub type R = crate::R<Awd0chsrSpec>;
#[doc = "Register `AWD0CHSR` writer"]
pub type W = crate::W<Awd0chsrSpec>;
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
        f.debug_struct("AWD0CHSR")
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AwdchW<'_, Awd0chsrSpec> {
        AwdchW::new(self, 0)
    }
}
#[doc = "desc AWD0CHSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awd0chsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd0chsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd0chsrSpec;
impl crate::RegisterSpec for Awd0chsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awd0chsr::R`](R) reader structure"]
impl crate::Readable for Awd0chsrSpec {}
#[doc = "`write(|w| ..)` method takes [`awd0chsr::W`](W) writer structure"]
impl crate::Writable for Awd0chsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWD0CHSR to value 0"]
impl crate::Resettable for Awd0chsrSpec {}
