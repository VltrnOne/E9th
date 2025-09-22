# E9TH Static Site

This is a drop-in static site package for E9TH - a utility deflation secure token on Solana.

## Files

- `index.html` - Main homepage with tokenomics, features, and roadmap
- `app.html` - Interactive dashboard for wallet connection and token management
- `litepaper.md` - Comprehensive project documentation
- `assets/` - Logo, favicon, and hero images

## Features

### Homepage (`index.html`)
- Clean, modern design with E9TH branding
- Tokenomics overview with allocation breakdown
- Security and governance information
- Roadmap and utility features
- Mobile-responsive layout

### Dashboard (`app.html`)
- Solana wallet integration (Phantom)
- Token balance and staking interface
- Deflation metrics tracking
- Interactive staking/unstaking functionality
- Real-time transaction feedback

### Assets
- SVG-based logo and favicon
- Hero image with E9TH branding
- Optimized for web performance

## Deployment

This site is ready for deployment on any static hosting service:

- **DeployPad**: Upload the entire folder
- **Vercel**: Connect to GitHub repository
- **Netlify**: Drag and drop the folder
- **GitHub Pages**: Push to repository

## Customization

### Colors
The site uses CSS custom properties for easy theming:
```css
:root {
  --bg: #0B0B10;        /* Background */
  --card: #151621;       /* Card backgrounds */
  --text: #E9ECF1;       /* Main text */
  --muted: #9AA1AC;      /* Muted text */
  --violet: #7B61FF;     /* Primary purple */
  --blue: #3EC3FF;       /* Primary blue */
  --silver: #C9CED6;     /* Secondary text */
  --ok: #34d399;         /* Success green */
}
```

### Content
- Update tokenomics in `index.html`
- Modify roadmap and features
- Add real Solana program integration in `app.html`

## Integration with Solana Program

The dashboard (`app.html`) includes placeholder functions for:
- Wallet connection
- Token balance fetching
- Staking/unstaking transactions
- Reward claiming
- Deflation metrics

To integrate with the actual E9TH Solana program:
1. Update the program ID in the JavaScript
2. Implement real transaction calls
3. Add proper error handling
4. Connect to the actual token accounts

## Browser Support

- Modern browsers with ES6+ support
- Solana wallet extension required for dashboard
- Mobile-responsive design

## License

This static site package is provided as-is for E9TH project use.
