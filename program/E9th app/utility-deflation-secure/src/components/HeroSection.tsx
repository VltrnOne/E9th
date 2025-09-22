import React from 'react';

const HeroSection: React.FC = () => {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden" style={{backgroundColor: '#0B0B10'}}>
      {/* Background Image */}
      <div className="absolute inset-0 z-0">
        <img 
          src="https://d64gsuwffb70l.cloudfront.net/68d0ef5ed4cf87616e216bdd_1758523271093_80515c4e.webp"
          alt="E9TH Hero Background"
          className="w-full h-full object-cover opacity-60"
        />
        <div className="absolute inset-0 bg-gradient-to-br from-purple-900/20 via-transparent to-blue-900/20"></div>
      </div>

      {/* Content */}
      <div className="relative z-10 text-center px-4 max-w-6xl mx-auto">
        <div className="mb-8">
          <h1 className="text-5xl md:text-7xl font-bold mb-6 text-white leading-tight">
            E9TH — where utility comes{' '}
            <span className="text-transparent bg-clip-text bg-gradient-to-r" style={{backgroundImage: 'linear-gradient(45deg, #7B61FF, #3EC3FF)'}}>
              full circle
            </span>
          </h1>
          <p className="text-xl md:text-2xl mb-8 max-w-3xl mx-auto" style={{color: '#C9CED6'}}>
            Unlock discounts, early access, and exclusive products on Solana.
          </p>
        </div>

        <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
          <button className="px-8 py-4 rounded-lg font-semibold text-lg transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-purple-500/25" 
                  style={{backgroundColor: '#7B61FF', color: 'white'}}>
            Launch App
          </button>
          <button className="px-8 py-4 rounded-lg font-semibold text-lg border-2 transition-all duration-300 hover:scale-105" 
                  style={{borderColor: '#3EC3FF', color: '#3EC3FF'}} 
                  onMouseEnter={(e) => {
                    e.currentTarget.style.backgroundColor = '#3EC3FF';
                    e.currentTarget.style.color = '#0B0B10';
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.backgroundColor = 'transparent';
                    e.currentTarget.style.color = '#3EC3FF';
                  }}>
            Read Litepaper
          </button>
        </div>

        {/* Stats Strip */}
        <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl mx-auto">
          <div className="text-center">
            <div className="text-3xl font-bold" style={{color: '#7B61FF'}}>10B</div>
            <div className="text-sm" style={{color: '#C9CED6'}}>Total Supply</div>
          </div>
          <div className="text-center">
            <div className="text-3xl font-bold" style={{color: '#3EC3FF'}}>$0.00005</div>
            <div className="text-sm" style={{color: '#C9CED6'}}>Initial Price</div>
          </div>
          <div className="text-center">
            <div className="text-3xl font-bold" style={{color: '#7B61FF'}}>Deflationary</div>
            <div className="text-sm" style={{color: '#C9CED6'}}>Token Model</div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default HeroSection;