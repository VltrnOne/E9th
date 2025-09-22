import React from 'react';
import { useAppContext } from '@/contexts/AppContext';
import { useIsMobile } from '@/hooks/use-mobile';
import Navigation from './Navigation';
import HeroSection from './HeroSection';
import ValueProposition from './ValueProposition';
import UtilitySection from './UtilitySection';
import TokenomicsSection from './TokenomicsSection';
import SecuritySection from './SecuritySection';
import RoadmapSection from './RoadmapSection';
import CommunitySection from './CommunitySection';
import Footer from './Footer';

const AppLayout: React.FC = () => {
  const { sidebarOpen, toggleSidebar } = useAppContext();
  const isMobile = useIsMobile();

  return (
    <div className="min-h-screen" style={{backgroundColor: '#0B0B10'}}>
      <Navigation />
      <main>
        <HeroSection />
        <ValueProposition />
        <UtilitySection />
        <TokenomicsSection />
        <SecuritySection />
        <RoadmapSection />
        <CommunitySection />
      </main>
      <Footer />
    </div>
  );
};

export default AppLayout;
