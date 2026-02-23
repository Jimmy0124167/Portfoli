# Kitso Nkalanga | Portfolio Website

A modern developer portfolio built with **HTML, CSS, JavaScript, and Rust (Axum)**.  
Showcases my programming journey, projects, and ongoing growth in **Linux**, **Cybersecurity**, and **Systems Development**.

🔗 **Live Site:** [kitsojimmynkalanga.up.railway.app](https://kitsojimmynkalanga.up.railway.app)

---

## 🚀 Features

- **Dark/Light Mode Toggle** - Persistent theme preference
- **Responsive Design** - Mobile-first, works on all devices
- **Working Contact Form** - Connected to Rust backend API
- **Smooth Animations** - Scroll-triggered section reveals
- **SEO Optimized** - Meta tags for better discoverability
- **Fast & Lightweight** - Minimal dependencies, optimized performance

---

## 🧱 Tech Stack

**Frontend:**
- HTML5, CSS3, Vanilla JavaScript
- Space Mono & Syne fonts
- Responsive grid layouts
- Intersection Observer API for animations

**Backend (Rust):**
- Axum 0.8 - Async HTTP framework
- Tower HTTP - Static file serving
- Tokio - Async runtime
- Serde - JSON serialization

**Deployment:**
- Docker containerization
- Railway.app hosting
- Automatic deployments via GitHub

---

## 📁 Project Structure
```
portfoli/
├── backend/
│   ├── src/
│   │   └── main.rs          # Axum server + API routes
│   ├── Cargo.toml           # Rust dependencies
│   └── Dockerfile           # Build configuration
├── frontend/
│   ├── index.html           # Main HTML structure
│   ├── style.css            # Styles with theme system
│   └── script.js            # Interactivity & form handling
├── railway.toml             # Railway deployment config (if needed)
└── README.md
```

---

## 🛠️ Local Development

### Prerequisites
- Rust 1.78+ ([rustup.rs](https://rustup.rs))
- Docker (optional, for containerized development)

### Running Locally

**Option 1: Direct Rust**
```bash
cd backend
cargo run --release
# Visit http://localhost:8080
```

**Option 2: Docker**
```bash
docker build -t portfolio .
docker run -p 8080:8080 portfolio
```

---

## 🚢 Deployment (Railway)

This project auto-deploys to Railway when you push to GitHub.

### Dockerfile Method (Current)
Railway automatically detects the `Dockerfile` and builds from it:
- Builds the Rust backend
- Copies frontend files into the container
- Exposes the app on Railway's assigned PORT

### Manual Deploy
```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and link project
railway login
railway link

# Deploy
railway up
```

---

## 🔧 Configuration

### Environment Variables (Railway)
- `PORT` - Automatically set by Railway (default: 8080)

### Updating Projects
Edit `frontend/index.html` - Projects section around line 140.

### Changing Theme Colors
Edit CSS variables in `frontend/style.css`:
```css
:root {
  --accent: #66fcf1;        /* Primary accent color */
  --font-display: 'Syne';   /* Heading font */
  --font-mono: 'Space Mono'; /* Body font */
}
```

---

## 📝 Best Practices Implemented

✅ **Performance**
- Preconnect to Google Fonts CDN
- CSS variables for consistent theming
- Minimal JavaScript, no heavy frameworks
- Optimized Dockerfile with multi-stage build

✅ **Accessibility**
- Semantic HTML5 elements
- ARIA labels on interactive elements
- Keyboard navigation support
- Proper color contrast ratios

✅ **SEO**
- Meta description and keywords
- Open Graph tags for social sharing
- Descriptive page title
- Proper heading hierarchy

✅ **Security**
- `rel="noopener"` on external links
- Input validation on contact form
- Rust's memory safety prevents common vulnerabilities

✅ **Code Quality**
- Clear file organization
- Commented sections in code
- Consistent naming conventions
- Error handling in form submission

---

## 🎯 Future Improvements

- [ ] Add blog section with Markdown support
- [ ] Implement email notifications for contact form
- [ ] Add project filtering by technology
- [ ] Create a dark/light mode for individual project pages
- [ ] Add analytics (privacy-focused)
- [ ] Implement rate limiting on contact form
- [ ] Add automated tests for backend API

---

## 📫 Contact

- **Email:** kitsojimmynkalanga@gmail.com
- **GitHub:** [@jimmy0124167](https://github.com/jimmy0124167)
- **Portfolio:** [kitsojimmynkalanga.up.railway.app](https://kitsojimmynkalanga.up.railway.app)

---

## 📄 License

This project is open source and available under the MIT License.

---

**Built with purpose, curiosity, and code.** ⚙️
