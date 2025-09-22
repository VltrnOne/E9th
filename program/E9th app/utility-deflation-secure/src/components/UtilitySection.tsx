import React from 'react';

const UtilitySection: React.FC = () => {
  const utilities = [
    {
      icon: "🎟️",
      title: "Discounts on services",
      description: "Pay less with E9TH.",
      color: "#7B61FF"
    },
    {
      icon: "🚀",
      title: "Early access",
      description: "Be first in line for new products and drops.",
      color: "#3EC3FF"
    },
    {
      icon: "🔒",
      title: "Staking",
      description: "Lock tokens, earn transparent rewards.",
      color: "#7B61FF"
    },
    {
      icon: "🎁",
      title: "Airdrops",
      description: "Batched and verifiable, rewarding the community.",
      color: "#3EC3FF"
    },
    {
      icon: "🔑",
      title: "Exclusives",
      description: "Gated software, solutions, and premium offerings.",
      color: "#7B61FF"
    }
  ];

  return (
    <section className="py-20 px-4 relative">
      {/* Background */}
      <div className="absolute inset-0" style={{backgroundColor: '#0B0B10'}}>
        <div className="absolute inset-0 opacity-10">
          <img 
            src="https://d64gsuwffb70l.cloudfront.net/68d0ef5ed4cf87616e216bdd_1758523281512_29096860.webp"
            alt="Solana Network"
            className="w-full h-full object-cover"
          />
        </div>
      </div>

      <div className="relative z-10 max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold mb-6 text-white">
            Hold E9TH. Unlock More.
          </h2>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mb-12">
          {utilities.map((utility, index) => (
            <div key={index} className="p-6 rounded-xl border transition-all duration-300 hover:scale-105 hover:shadow-xl group"
                 style={{
                   backgroundColor: 'rgba(255, 255, 255, 0.05)',
                   borderColor: utility.color,
                   backdropFilter: 'blur(10px)'
                 }}>
              <div className="text-4xl mb-4">{utility.icon}</div>
              <h3 className="text-xl font-bold mb-3 text-white group-hover:text-transparent group-hover:bg-clip-text group-hover:bg-gradient-to-r"
                  style={{backgroundImage: `linear-gradient(45deg, ${utility.color}, #ffffff)`}}>
                {utility.title}
              </h3>
              <p className="leading-relaxed" style={{color: '#C9CED6'}}>
                {utility.description}
              </p>
            </div>
          ))}
        </div>

        <div className="text-center">
          <button className="px-8 py-4 rounded-lg font-semibold text-lg transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-blue-500/25" 
                  style={{backgroundColor: '#3EC3FF', color: '#0B0B10'}}>
            Explore Utilities
          </button>
        </div>
      </div>
    </section>
  );
};

export default UtilitySection;