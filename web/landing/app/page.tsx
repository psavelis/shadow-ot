'use client'

import { HeroSection } from '@/components/home/HeroSection'
import { RealmsSection } from '@/components/home/RealmsSection'
import { FeaturesSection } from '@/components/home/FeaturesSection'
import { StatsSection } from '@/components/home/StatsSection'
import { BlockchainSection } from '@/components/home/BlockchainSection'
import { DownloadSection } from '@/components/home/DownloadSection'
import { NewsSection } from '@/components/home/NewsSection'
import { CommunitySection } from '@/components/home/CommunitySection'

export default function Home() {
  return (
    <>
      <HeroSection />
      <RealmsSection />
      <FeaturesSection />
      <StatsSection />
      <BlockchainSection />
      <DownloadSection />
      <NewsSection />
      <CommunitySection />
    </>
  )
}
