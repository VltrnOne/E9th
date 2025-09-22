import React from 'react';

const Footer: React.FC = () => {
  const footerLinks = {
    Product: ['Utility', 'Tokenomics', 'Staking', 'Airdrops'],
    Resources: ['Litepaper', 'Security', 'Brand Kit', 'Updates'],
    Community: ['Twitter/X', 'Discord', 'Partners', 'Contact'],
    Legal: ['Terms of Service', 'Privacy Policy', 'Risk Disclosure']
  };

  return (
    <footer className="py-16 px-4 border-t" style={{backgroundColor: '#0B0B10', borderColor: 'rgba(123, 97, 255, 0.3)'}}>
      <div className="max-w-6xl mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-5 gap-8">
          {/* Brand */}
          <div className="md:col-span-1">
            <div className="text-2xl font-bold text-white mb-4">E9TH</div>
            <p className="text-sm mb-4" style={{color: '#C9CED6'}}>
              Where utility comes full circle.
            </p>
            <div className="text-xs" style={{color: '#C9CED6'}}>
              Utility token on Solana
            </div>
          </div>

          {/* Links */}
          {Object.entries(footerLinks).map(([category, links]) => (
            <div key={category}>
              <h4 className="text-white font-semibold mb-4">{category}</h4>
              <ul className="space-y-2">
                {links.map((link) => (
                  <li key={link}>
                    <a href="#" className="text-sm transition-colors duration-300 hover:text-white" 
                       style={{color: '#C9CED6'}}>
                      {link}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        {/* Bottom */}
        <div className="mt-12 pt-8 border-t flex flex-col md:flex-row justify-between items-center" 
             style={{borderColor: 'rgba(123, 97, 255, 0.3)'}}>
          <div className="text-sm mb-4 md:mb-0" style={{color: '#C9CED6'}}>
            © 2024 E9TH. All rights reserved.
          </div>
          <div className="text-xs text-center md:text-right max-w-md" style={{color: '#C9CED6'}}>
            <strong>Risk Notice:</strong> Cryptocurrency investments carry significant risk. 
            Past performance does not guarantee future results. Please invest responsibly.
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;