import React from 'react';

const CommunitySection: React.FC = () => {
  const socialLinks = [
    {
      name: "Twitter/X",
      handle: "@E9TH",
      description: "Latest updates and announcements",
      icon: "🐦",
      color: "#3EC3FF"
    },
    {
      name: "Discord",
      handle: "Join Community",
      description: "Chat with holders and team",
      icon: "💬",
      color: "#7B61FF"
    },
    {
      name: "Explorer",
      handle: "View Contract",
      description: "On-chain transparency",
      icon: "🔍",
      color: "#3EC3FF"
    }
  ];

  return (
    <section className="py-20 px-4" style={{backgroundColor: '#0B0B10'}}>
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold mb-6 text-white">
            Join the Movement
          </h2>
          <p className="text-xl max-w-3xl mx-auto" style={{color: '#C9CED6'}}>
            E9TH is more than a token — it's a gateway to utility on Solana.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
          {socialLinks.map((link, index) => (
            <div key={index} className="text-center p-6 rounded-xl border transition-all duration-300 hover:scale-105 group cursor-pointer"
                 style={{
                   backgroundColor: 'rgba(255, 255, 255, 0.05)',
                   borderColor: link.color,
                   backdropFilter: 'blur(10px)'
                 }}>
              <div className="text-4xl mb-4">{link.icon}</div>
              <h3 className="text-xl font-bold mb-2 text-white group-hover:text-transparent group-hover:bg-clip-text group-hover:bg-gradient-to-r"
                  style={{backgroundImage: `linear-gradient(45deg, ${link.color}, #ffffff)`}}>
                {link.name}
              </h3>
              <div className="text-lg font-semibold mb-2" style={{color: link.color}}>
                {link.handle}
              </div>
              <p className="text-sm" style={{color: '#C9CED6'}}>
                {link.description}
              </p>
            </div>
          ))}
        </div>

        {/* Final CTA */}
        <div className="text-center p-12 rounded-xl border" 
             style={{
               backgroundColor: 'rgba(255, 255, 255, 0.05)',
               borderColor: '#7B61FF',
               backdropFilter: 'blur(10px)'
             }}>
          <h3 className="text-3xl md:text-4xl font-bold mb-4 text-white">
            E9TH — where utility comes full circle.
          </h3>
          <p className="text-xl mb-8 max-w-2xl mx-auto" style={{color: '#C9CED6'}}>
            Get started today. Buy, hold, and unlock more with every token.
          </p>
          <button className="px-12 py-4 rounded-lg font-semibold text-xl transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-purple-500/25" 
                  style={{backgroundColor: '#7B61FF', color: 'white'}}>
            Launch App
          </button>
        </div>
      </div>
    </section>
  );
};

export default CommunitySection;