# Fake-rs Documentation Site

This directory contains the GitHub Pages documentation site for fake-rs.

## 🌐 Live Site

Once deployed, the site will be available at: `https://cksac.github.io/fake-rs/`

## 📁 Structure

```
docs/
├── index.html      # Main documentation page
├── styles.css      # Styling
├── script.js       # Interactive features
└── README.md       # This file
```

## 🚀 Deployment

### GitHub Pages Setup

1. Go to your repository settings on GitHub
2. Navigate to "Pages" section
3. Under "Source", select:
   - **Source**: Deploy from a branch
   - **Branch**: main (or master)
   - **Folder**: /docs
4. Click "Save"

Your site will be published at `https://cksac.github.io/fake-rs/`

### Alternative: Using GitHub Actions

You can also use GitHub Actions for deployment. Create `.github/workflows/pages.yml`:

```yaml
name: Deploy GitHub Pages

on:
  push:
    branches: [ main ]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v3
      
      - name: Setup Pages
        uses: actions/configure-pages@v3
      
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v2
        with:
          path: './docs'
      
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v2
```

## ✨ Features

- **Responsive Design**: Mobile-friendly layout
- **Dark Theme**: Modern dark color scheme
- **Interactive Search**: Real-time faker search functionality
- **Code Highlighting**: Syntax highlighting for Rust, Bash, and TOML
- **Copy Buttons**: One-click code copying
- **Smooth Scrolling**: Enhanced navigation experience
- **Keyboard Shortcuts**: 
  - `/` - Focus search
  - `ESC` - Clear search
- **Comprehensive Coverage**: All 100+ fakers documented
- **Multi-language Support**: Documentation of all 12 supported locales
- **Feature Documentation**: Complete feature flag reference

## 🎨 Customization

The documentation uses a **data-driven architecture** for easy maintenance:

- **`data/locales.js`** - All supported locales ⭐
- **`data/features.js`** - All feature flags ⭐
- **`data/fakers.js`** - All faker categories and items ⭐
- **`data/examples.js`** - All code examples ⭐
- **`styles.css`** - Visual styling
- **`script.js`** - Interactive features
- **`index.html`** - Page structure

### Quick Updates

**Adding a new locale?** Just edit `data/locales.js`:
```javascript
{ name: "Finnish", code: "FI_FI" }
```

**Adding a new feature?** Just edit `data/features.js`:
```javascript
{ code: "new_feature", description: "What it enables" }
```

**Adding a new faker?** Just edit `data/fakers.js`:
```javascript
{ code: "NewFaker()", desc: "What it does" }
```

**Adding an example?** Just edit `data/examples.js`:
```javascript
{ title: "Title", language: "rust", code: `...` }
```

**Changing colors?** Edit `styles.css` variables:
```css
:root {
    --primary-color: #ff6b35;
    --secondary-color: #004e89;
    --accent-color: #1a659e;
}
```

📝 **See [MAINTENANCE.md](MAINTENANCE.md) for complete maintenance guide**

## 🧪 Local Development

Simply open `index.html` in a web browser, or use a local server:

```bash
# Using Python
python -m http.server 8000

# Using Node.js http-server
npx http-server

# Using PHP
php -S localhost:8000
```

Then visit `http://localhost:8000`

## 📦 Dependencies

The site uses CDN-hosted libraries:

- **Highlight.js** (v11.9.0): Code syntax highlighting
  - Rust, Bash, and TOML language support

No build process or npm packages required!

## 🔄 Updating

To update the documentation:

1. **Add/Update Locales**: Edit `data/locales.js` ⭐
2. **Add/Update Features**: Edit `data/features.js` ⭐
3. **Add/Update Fakers**: Edit `data/fakers.js` ⭐
4. **Add/Update Examples**: Edit `data/examples.js` ⭐
5. **Change Styling**: Edit `styles.css`
6. **Add Functionality**: Edit `script.js`
7. **Update Structure**: Edit `index.html`
8. Test locally
9. Commit and push to GitHub
10. GitHub Pages will automatically rebuild (may take a few minutes)

See [MAINTENANCE.md](MAINTENANCE.md) for detailed instructions.

## 🐛 Troubleshooting

### Site not showing up

- Check GitHub Pages settings in repository settings
- Ensure the `docs/` folder is on the correct branch
- Wait a few minutes after pushing changes

### Styling issues

- Clear browser cache
- Check browser console for errors
- Verify all CSS/JS files are in the `docs/` directory

### Search not working

- Ensure JavaScript is enabled
- Check browser console for errors
- Verify `script.js` loaded correctly

## 📝 Maintenance

The documentation uses a **data-driven architecture** for maintainability:

✅ **Easy Updates**: Add new content by editing data files  
✅ **No HTML Changes**: Just update the relevant data/*.js files  
✅ **Consistent Formatting**: Automatic styling and structure  
✅ **Search Included**: New content automatically searchable  

**Keep in sync with fake-rs:**
- New locales → Update `data/locales.js`
- New features → Update `data/features.js`
- New fakers → Update `data/fakers.js`
- New examples → Update `data/examples.js`

📚 **Full guide**: [MAINTENANCE.md](MAINTENANCE.md)

## 🤝 Contributing

To contribute to the documentation:

1. Fork the repository
2. Make changes in the `docs/` directory
3. Test locally
4. Submit a pull request

## 📄 License

The documentation site follows the same license as fake-rs: MIT OR Apache-2.0
