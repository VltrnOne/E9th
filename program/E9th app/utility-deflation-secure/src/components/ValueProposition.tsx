import React from 'react';

const ValueProposition: React.FC = () => {
  const features = [
    {
      icon: "https://d64gsuwffb70l.cloudfront.net/68d0ef5ed4cf87616e216bdd_1758523272270_4dc05b93.webp",
      title: "Instant Utility",
      description: "Use E9TH to redeem discounts, access tools, and unlock services from day one."
    },
    {
      icon: "https://d64gsuwffb70l.cloudfront.net/68d0ef5ed4cf87616e216bdd_1758523273960_b16c4fc6.webp",
      title: "Deflationary Design",
      description: "Supply reduces with every transfer, aligning growth with scarcity."
    },
    {
      icon: "https://d64gsuwffb70l.cloudfront.net/68d0ef5ed4cf87616e216bdd_1758523275646_1c0742c8.webp",
      title: "Transparent Security",
      description: "Multisig governance, pausing, blacklisting, and on-chain logs to protect holders."
    }
  ];

  return (
    <section className="py-20 px-4" style={{backgroundColor: '#0B0B10'}}>
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold mb-6 text-white">
            Why E9TH?
          </h2>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <div key={index} className="text-center group">
              <div className="mb-6 flex justify-center">
                <div className="w-20 h-20 rounded-full overflow-hidden border-2 transition-all duration-300 group-hover:scale-110 group-hover:shadow-lg" 
                     style={{borderColor: '#7B61FF', boxShadow: '0 0 20px rgba(123, 97, 255, 0.3)'}}>
                  <img 
                    src={feature.icon} 
                    alt={feature.title}
                    className="w-full h-full object-cover"
                  />
                </div>
              </div>
              <h3 className="text-2xl font-bold mb-4 text-white">
                {feature.title}
              </h3>
              <p className="text-lg leading-relaxed" style={{color: '#C9CED6'}}>
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default ValueProposition;