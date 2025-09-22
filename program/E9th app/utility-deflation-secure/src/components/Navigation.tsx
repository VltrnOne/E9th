import React, { useState } from 'react';

const Navigation: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);

  const navItems = [
    { name: 'Utility', href: '#utility' },
    { name: 'Tokenomics', href: '#tokenomics' },
    { name: 'Staking', href: '#staking' },
    { name: 'Security', href: '#security' },
    { name: 'Litepaper', href: '#litepaper' }
  ];

  return (
    <nav className="fixed top-0 w-full z-50 backdrop-blur-md border-b" 
         style={{backgroundColor: 'rgba(11, 11, 16, 0.9)', borderColor: 'rgba(123, 97, 255, 0.3)'}}>
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Logo */}
          <div className="flex items-center">
            <div className="text-2xl font-bold text-white">
              E9TH
            </div>
          </div>

          {/* Desktop Navigation */}
          <div className="hidden md:block">
            <div className="ml-10 flex items-baseline space-x-8">
              {navItems.map((item) => (
                <a
                  key={item.name}
                  href={item.href}
                  className="text-white hover:text-transparent hover:bg-clip-text hover:bg-gradient-to-r px-3 py-2 text-sm font-medium transition-all duration-300"
                  style={{backgroundImage: 'linear-gradient(45deg, #7B61FF, #3EC3FF)'}}
                >
                  {item.name}
                </a>
              ))}
            </div>
          </div>

          {/* CTA Button */}
          <div className="hidden md:block">
            <button className="px-6 py-2 rounded-lg font-semibold text-sm transition-all duration-300 hover:scale-105" 
                    style={{backgroundColor: '#7B61FF', color: 'white'}}>
              Launch App
            </button>
          </div>

          {/* Mobile menu button */}
          <div className="md:hidden">
            <button
              onClick={() => setIsOpen(!isOpen)}
              className="text-white hover:text-gray-300 focus:outline-none focus:text-gray-300"
            >
              <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                {isOpen ? (
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                ) : (
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
                )}
              </svg>
            </button>
          </div>
        </div>

        {/* Mobile Navigation */}
        {isOpen && (
          <div className="md:hidden">
            <div className="px-2 pt-2 pb-3 space-y-1 sm:px-3">
              {navItems.map((item) => (
                <a
                  key={item.name}
                  href={item.href}
                  className="text-white hover:text-gray-300 block px-3 py-2 text-base font-medium"
                  onClick={() => setIsOpen(false)}
                >
                  {item.name}
                </a>
              ))}
              <button className="w-full mt-4 px-6 py-2 rounded-lg font-semibold text-sm" 
                      style={{backgroundColor: '#7B61FF', color: 'white'}}>
                Launch App
              </button>
            </div>
          </div>
        )}
      </div>
    </nav>
  );
};

export default Navigation;