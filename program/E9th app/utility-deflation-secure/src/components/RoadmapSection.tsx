import React from 'react';

const RoadmapSection: React.FC = () => {
  const phases = [
    {
      phase: "Phase 1",
      title: "Foundation",
      items: ["Token launch", "Staking v1", "Website & litepaper"],
      status: "current",
      color: "#7B61FF"
    },
    {
      phase: "Phase 2",
      title: "Integration",
      items: ["Partner integrations", "Holder dashboard", "Live burn tracker"],
      status: "upcoming",
      color: "#3EC3FF"
    },
    {
      phase: "Phase 3",
      title: "Expansion",
      items: ["Expanded marketplace", "Mobile app", "Scaling rewards"],
      status: "future",
      color: "#C9CED6"
    }
  ];

  return (
    <section className="py-20 px-4" style={{backgroundColor: '#0B0B10'}}>
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold mb-6 text-white">
            From Launch to Growth
          </h2>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
          {phases.map((phase, index) => (
            <div key={index} className="relative">
              {/* Connection Line */}
              {index < phases.length - 1 && (
                <div className="hidden md:block absolute top-8 left-full w-full h-0.5 z-0" 
                     style={{backgroundColor: 'rgba(201, 206, 214, 0.3)', transform: 'translateX(-50%)'}}></div>
              )}
              
              <div className="relative z-10 p-6 rounded-xl border transition-all duration-300 hover:scale-105"
                   style={{
                     backgroundColor: 'rgba(255, 255, 255, 0.05)',
                     borderColor: phase.color,
                     backdropFilter: 'blur(10px)'
                   }}>
                <div className="flex items-center mb-4">
                  <div className="w-12 h-12 rounded-full flex items-center justify-center font-bold text-white mr-4"
                       style={{backgroundColor: phase.color}}>
                    {index + 1}
                  </div>
                  <div>
                    <div className="text-sm font-semibold" style={{color: phase.color}}>{phase.phase}</div>
                    <div className="text-xl font-bold text-white">{phase.title}</div>
                  </div>
                </div>
                
                <ul className="space-y-2">
                  {phase.items.map((item, itemIndex) => (
                    <li key={itemIndex} className="flex items-center text-sm" style={{color: '#C9CED6'}}>
                      <div className="w-2 h-2 rounded-full mr-3" style={{backgroundColor: phase.color}}></div>
                      {item}
                    </li>
                  ))}
                </ul>

                {phase.status === 'current' && (
                  <div className="mt-4 px-3 py-1 rounded-full text-xs font-semibold inline-block"
                       style={{backgroundColor: 'rgba(123, 97, 255, 0.2)', color: '#7B61FF'}}>
                    In Progress
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>

        <div className="text-center">
          <button className="px-8 py-4 rounded-lg font-semibold text-lg transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-purple-500/25" 
                  style={{backgroundColor: '#7B61FF', color: 'white'}}>
            Follow Updates
          </button>
        </div>
      </div>
    </section>
  );
};

export default RoadmapSection;