import React from 'react';

const SecuritySection: React.FC = () => {
  const securityFeatures = [
    {
      icon: "🔑",
      title: "Role-based access",
      description: "Owner, Operator, Treasury"
    },
    {
      icon: "⛓️",
      title: "Multisig controls",
      description: "For sensitive actions"
    },
    {
      icon: "🛡️",
      title: "Emergency pause",
      description: "& blacklist protection"
    },
    {
      icon: "⏳",
      title: "Optional timelocks",
      description: "For critical changes"
    },
    {
      icon: "📖",
      title: "Open-source code",
      description: "& audits"
    }
  ];

  return (
    <section className="py-20 px-4" style={{backgroundColor: '#0B0B10'}}>
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold mb-6 text-white">
            Security First. Confidence Always.
          </h2>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6 mb-12">
          {securityFeatures.map((feature, index) => (
            <div key={index} className="text-center p-6 rounded-xl border transition-all duration-300 hover:scale-105 group"
                 style={{
                   backgroundColor: 'rgba(255, 255, 255, 0.05)',
                   borderColor: '#3EC3FF',
                   backdropFilter: 'blur(10px)'
                 }}>
              <div className="text-3xl mb-4">{feature.icon}</div>
              <h3 className="text-lg font-bold mb-2 text-white group-hover:text-transparent group-hover:bg-clip-text group-hover:bg-gradient-to-r"
                  style={{backgroundImage: 'linear-gradient(45deg, #3EC3FF, #ffffff)'}}>
                {feature.title}
              </h3>
              <p className="text-sm" style={{color: '#C9CED6'}}>
                {feature.description}
              </p>
            </div>
          ))}
        </div>

        <div className="text-center">
          <button className="px-8 py-4 rounded-lg font-semibold text-lg transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-blue-500/25" 
                  style={{backgroundColor: '#3EC3FF', color: '#0B0B10'}}>
            Read Security Overview
          </button>
        </div>
      </div>
    </section>
  );
};

export default SecuritySection;