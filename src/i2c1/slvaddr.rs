#[doc = "Register `SLVADDR` reader"]
pub type R = crate::R<SlvaddrSpec>;
#[doc = "Register `SLVADDR` writer"]
pub type W = crate::W<SlvaddrSpec>;
#[doc = "Field `SLVADRR` reader - desc SLVADRR"]
pub type SlvadrrR = crate::FieldReader<u16>;
#[doc = "Field `SLVADRR` writer - desc SLVADRR"]
pub type SlvadrrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - desc SLVADRR"]
    #[inline(always)]
    pub fn slvadrr(&self) -> SlvadrrR {
        SlvadrrR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVADDR")
            .field("slvadrr", &self.slvadrr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - desc SLVADRR"]
    #[inline(always)]
    pub fn slvadrr(&mut self) -> SlvadrrW<'_, SlvaddrSpec> {
        SlvadrrW::new(self, 0)
    }
}
#[doc = "desc SLVADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`slvaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvaddrSpec;
impl crate::RegisterSpec for SlvaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvaddr::R`](R) reader structure"]
impl crate::Readable for SlvaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`slvaddr::W`](W) writer structure"]
impl crate::Writable for SlvaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLVADDR to value 0"]
impl crate::Resettable for SlvaddrSpec {}
