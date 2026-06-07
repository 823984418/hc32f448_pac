#[doc = "Register `XTALDIVR` reader"]
pub type R = crate::R<XtaldivrSpec>;
#[doc = "Register `XTALDIVR` writer"]
pub type W = crate::W<XtaldivrSpec>;
#[doc = "Field `DEMON` reader - desc DEMON"]
pub type DemonR = crate::FieldReader<u16>;
#[doc = "Field `DEMON` writer - desc DEMON"]
pub type DemonW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NUMER` reader - desc NUMER"]
pub type NumerR = crate::FieldReader<u32>;
#[doc = "Field `NUMER` writer - desc NUMER"]
pub type NumerW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:10 - desc DEMON"]
    #[inline(always)]
    pub fn demon(&self) -> DemonR {
        DemonR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:28 - desc NUMER"]
    #[inline(always)]
    pub fn numer(&self) -> NumerR {
        NumerR::new((self.bits >> 12) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTALDIVR")
            .field("demon", &self.demon())
            .field("numer", &self.numer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - desc DEMON"]
    #[inline(always)]
    pub fn demon(&mut self) -> DemonW<'_, XtaldivrSpec> {
        DemonW::new(self, 0)
    }
    #[doc = "Bits 12:28 - desc NUMER"]
    #[inline(always)]
    pub fn numer(&mut self) -> NumerW<'_, XtaldivrSpec> {
        NumerW::new(self, 12)
    }
}
#[doc = "desc XTALDIVR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtaldivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtaldivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtaldivrSpec;
impl crate::RegisterSpec for XtaldivrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtaldivr::R`](R) reader structure"]
impl crate::Readable for XtaldivrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtaldivr::W`](W) writer structure"]
impl crate::Writable for XtaldivrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALDIVR to value 0x03d0_9040"]
impl crate::Resettable for XtaldivrSpec {
    const RESET_VALUE: u32 = 0x03d0_9040;
}
