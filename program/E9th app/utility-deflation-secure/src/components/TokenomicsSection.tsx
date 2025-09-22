import React from 'react';

const TokenomicsSection: React.FC = () => {
  const allocation = [
    { label: "Public Sale", percentage: 40, color: "#7B61FF" },
    { label: "Team & Founders", percentage: 20, color: "#3EC3FF" },
    { label: "Marketing & Rewards", percentage: 15, color: "#7B61FF" },
    { label: "Liquidity", percentage: 15, color: "#3EC3FF" },
    { label: "Treasury", percentage: 10, color: "#C9CED6" }
  ];

  const tokenDetails = [
    { label: "Ticker", value: "E9TH" },
    { label: "Chain", value: "Solana (SPL / Token-2022)" },
    { label: "Supply", value: "10,000,000,000 (deflationary)" },
    { label: "Initial Price", value: "$0.00005" }
  ];

  return (
    <section className="py-20 px-4" style={{backgroundColor: '#0B0B10'}}>
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold mb-6 text-white">
            A Sustainable Model for Long-Term Growth
          </h2>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          {/* Token Details */}
          <div>
            <div className="space-y-6">
              {tokenDetails.map((detail, index) => (
                <div key={index} className="flex justify-between items-center p-4 rounded-lg border" 
                     style={{backgroundColor: 'rgba(255, 255, 255, 0.05)', borderColor: '#7B61FF'}}>
                  <span className="font-semibold text-white">{detail.label}:</span>
                  <span style={{color: '#C9CED6'}}>{detail.value}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Allocation Chart */}
          <div>
            <h3 className="text-2xl font-bold mb-6 text-white text-center">Token Allocation</h3>
            <div className="space-y-4">
              {allocation.map((item, index) => (
                <div key={index} className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <div className="w-4 h-4 rounded-full" style={{backgroundColor: item.color}}></div>
                    <span className="text-white">{item.label}</span>
                  </div>
                  <span className="font-bold" style={{color: item.color}}>{item.percentage}%</span>
                </div>
              ))}
            </div>
            
            {/* Visual Bar Chart */}
            <div className="mt-8">
              <div className="h-8 rounded-lg overflow-hidden flex" style={{backgroundColor: 'rgba(255, 255, 255, 0.1)'}}>
                {allocation.map((item, index) => (
                  <div key={index} 
                       className="h-full transition-all duration-500 hover:opacity-80"
                       style={{
                         backgroundColor: item.color,
                         width: `${item.percentage}%`
                       }}
                       title={`${item.label}: ${item.percentage}%`}>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        <div className="text-center mt-12">
          <button className="px-8 py-4 rounded-lg font-semibold text-lg transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-purple-500/25" 
                  style={{backgroundColor: '#7B61FF', color: 'white'}}>
            View Full Tokenomics
          </button>
        </div>
      </div>
    </section>
  );
};

export default TokenomicsSection;