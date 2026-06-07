#[doc = "Register `DNSEQCTLB4` reader"]
pub type R = crate::R<Dnseqctlb4Spec>;
#[doc = "Register `DNSEQCTLB4` writer"]
pub type W = crate::W<Dnseqctlb4Spec>;
#[doc = "Field `DNSDIST` reader - desc DNSDIST"]
pub type DnsdistR = crate::FieldReader<u32>;
#[doc = "Field `DNSDIST` writer - desc DNSDIST"]
pub type DnsdistW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DNSCNTB` reader - desc DNSCNTB"]
pub type DnscntbR = crate::FieldReader<u16>;
#[doc = "Field `DNSCNTB` writer - desc DNSCNTB"]
pub type DnscntbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - desc DNSDIST"]
    #[inline(always)]
    pub fn dnsdist(&self) -> DnsdistR {
        DnsdistR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc DNSCNTB"]
    #[inline(always)]
    pub fn dnscntb(&self) -> DnscntbR {
        DnscntbR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DNSEQCTLB4")
            .field("dnsdist", &self.dnsdist())
            .field("dnscntb", &self.dnscntb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - desc DNSDIST"]
    #[inline(always)]
    pub fn dnsdist(&mut self) -> DnsdistW<'_, Dnseqctlb4Spec> {
        DnsdistW::new(self, 0)
    }
    #[doc = "Bits 20:31 - desc DNSCNTB"]
    #[inline(always)]
    pub fn dnscntb(&mut self) -> DnscntbW<'_, Dnseqctlb4Spec> {
        DnscntbW::new(self, 20)
    }
}
#[doc = "desc DNSEQCTLB4\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctlb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctlb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dnseqctlb4Spec;
impl crate::RegisterSpec for Dnseqctlb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dnseqctlb4::R`](R) reader structure"]
impl crate::Readable for Dnseqctlb4Spec {}
#[doc = "`write(|w| ..)` method takes [`dnseqctlb4::W`](W) writer structure"]
impl crate::Writable for Dnseqctlb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DNSEQCTLB4 to value 0"]
impl crate::Resettable for Dnseqctlb4Spec {}
