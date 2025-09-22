#!/bin/bash

# E9TH Static Site Deployment Script
echo "🚀 Deploying E9TH Static Site..."

# Check if we're in the right directory
if [ ! -f "index.html" ]; then
    echo "❌ Error: index.html not found. Please run this script from the static-site directory."
    exit 1
fi

# Create a deployment package
echo "📦 Creating deployment package..."
tar -czf e9th-static-site.tar.gz *.html *.md assets/ README.md

echo "✅ Deployment package created: e9th-static-site.tar.gz"
echo ""
echo "📋 Next steps:"
echo "1. Upload e9th-static-site.tar.gz to your hosting service"
echo "2. Extract the files to your web root directory"
echo "3. Update any hardcoded URLs in the HTML files"
echo "4. Test the site functionality"
echo ""
echo "🌐 Ready for deployment!"

# Optional: Open the site locally for testing
if command -v python3 &> /dev/null; then
    echo "🔧 Starting local server for testing..."
    echo "Visit: http://localhost:8000"
    echo "Press Ctrl+C to stop the server"
    python3 -m http.server 8000
fi
