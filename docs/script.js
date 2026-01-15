// Theme Management
function initTheme() {
    const savedTheme = localStorage.getItem('theme') || 'dark';
    document.documentElement.setAttribute('data-theme', savedTheme);
    updateThemeIcon(savedTheme);
}

function toggleTheme() {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    updateThemeIcon(newTheme);
}

function updateThemeIcon(theme) {
    const themeIcon = document.getElementById('themeIcon');
    if (themeIcon) {
        themeIcon.textContent = theme === 'dark' ? '☀️' : '🌙';
    }
}

// Initialize theme before content loads
initTheme();

// Syntax highlighting
document.addEventListener('DOMContentLoaded', (event) => {
    // Load dynamic content
    loadLocales();
    loadFeatures();
    loadFakers();
    loadExamples();
    
    // Then highlight code
    hljs.highlightAll();
    
    // Setup theme toggle button
    const themeToggle = document.getElementById('themeToggle');
    themeToggle?.addEventListener('click', toggleTheme);
});

// Mobile menu toggle
const hamburger = document.querySelector('.hamburger');
const navMenu = document.querySelector('.nav-menu');

// Load locales from data file
function loadLocales() {
    const container = document.getElementById('locales-container');
    if (!container || typeof localesData === 'undefined') return;
    
    localesData.forEach(locale => {
        const item = document.createElement('div');
        item.className = 'locale-item';
        item.textContent = `${locale.name} (${locale.code})`;
        container.appendChild(item);
    });
}

// Load feature flags from data file
function loadFeatures() {
    const container = document.getElementById('features-container');
    if (!container || typeof featuresData === 'undefined') return;
    
    featuresData.forEach(feature => {
        const item = document.createElement('li');
        item.innerHTML = `<code>${escapeHtml(feature.code)}</code> - ${escapeHtml(feature.description)}`;
        container.appendChild(item);
    });
}

// Load fakers from data file
function loadFakers() {
    const container = document.getElementById('fakers-container');
    if (!container || typeof fakersData === 'undefined') return;
    
    fakersData.forEach(category => {
        const categoryDiv = document.createElement('div');
        categoryDiv.className = 'faker-category';
        
        const heading = document.createElement('h3');
        heading.id = category.id;
        heading.innerHTML = `${category.icon} ${category.title}`;
        categoryDiv.appendChild(heading);
        
        const grid = document.createElement('div');
        grid.className = 'faker-grid';
        
        category.fakers.forEach(faker => {
            const item = document.createElement('div');
            item.className = 'faker-item';
            item.innerHTML = `
                <code>${escapeHtml(faker.code)}</code>
                <span class="faker-desc">${escapeHtml(faker.desc)}</span>
            `;
            grid.appendChild(item);
        });
        
        categoryDiv.appendChild(grid);
        
        if (category.note) {
            const note = document.createElement('p');
            note.className = 'faker-note';
            note.innerHTML = category.note;
            categoryDiv.appendChild(note);
        }
        
        container.appendChild(categoryDiv);
    });
}

// Load examples from data file
function loadExamples() {
    const container = document.getElementById('examples-container');
    if (!container || typeof examplesData === 'undefined') return;
    
    examplesData.forEach(example => {
        const card = document.createElement('div');
        card.className = 'card';
        
        const heading = document.createElement('h3');
        heading.textContent = example.title;
        card.appendChild(heading);
        
        const pre = document.createElement('pre');
        const code = document.createElement('code');
        code.className = `language-${example.language}`;
        code.textContent = example.code;
        pre.appendChild(code);
        card.appendChild(pre);
        
        if (example.note) {
            const note = document.createElement('p');
            note.className = 'code-note';
            note.innerHTML = example.note;
            card.appendChild(note);
        }
        
        container.appendChild(card);
    });
}

// Escape HTML to prevent XSS
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

hamburger?.addEventListener('click', () => {
    hamburger.classList.toggle('active');
    navMenu.classList.toggle('active');
});

// Close mobile menu when clicking on a link
document.querySelectorAll('.nav-menu a').forEach(link => {
    link.addEventListener('click', () => {
        hamburger?.classList.remove('active');
        navMenu?.classList.remove('active');
    });
});

// Smooth scrolling for anchor links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            const offsetTop = target.offsetTop - 80; // Account for fixed navbar
            window.scrollTo({
                top: offsetTop,
                behavior: 'smooth'
            });
        }
    });
});

// Scroll to top button
const scrollToTopBtn = document.getElementById('scrollToTop');

window.addEventListener('scroll', () => {
    if (window.pageYOffset > 300) {
        scrollToTopBtn?.classList.add('visible');
    } else {
        scrollToTopBtn?.classList.remove('visible');
    }
});

scrollToTopBtn?.addEventListener('click', () => {
    window.scrollTo({
        top: 0,
        behavior: 'smooth'
    });
});

// Add animation on scroll
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Observe all cards and feature cards
document.querySelectorAll('.card, .feature-card, .faker-category').forEach(el => {
    el.style.opacity = '0';
    el.style.transform = 'translateY(20px)';
    el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
    observer.observe(el);
});

// Search functionality for fakers (optional enhancement)
function createSearchBox() {
    const fakersSection = document.getElementById('fakers');
    if (!fakersSection) return;

    const searchContainer = document.createElement('div');
    searchContainer.className = 'search-container';
    searchContainer.innerHTML = `
        <input 
            type="text" 
            id="fakerSearch" 
            placeholder="Search fakers... (e.g., name, email, date)"
            style="
                width: 100%;
                padding: 1rem;
                border-radius: 8px;
                border: 2px solid var(--border-color);
                background: var(--card-bg);
                color: var(--text-primary);
                font-size: 1rem;
                margin-bottom: 2rem;
                transition: border-color 0.3s ease;
            "
        >
    `;

    const h2 = fakersSection.querySelector('h2');
    const description = fakersSection.querySelector('.section-description');
    
    if (description) {
        description.after(searchContainer);
    } else {
        h2.after(searchContainer);
    }

    const searchInput = document.getElementById('fakerSearch');
    searchInput?.addEventListener('input', (e) => {
        const searchTerm = e.target.value.toLowerCase();
        const categories = document.querySelectorAll('.faker-category');

        categories.forEach(category => {
            const categoryTitle = category.querySelector('h3').textContent.toLowerCase();
            const items = category.querySelectorAll('.faker-item');
            let hasVisibleItems = false;

            items.forEach(item => {
                const code = item.querySelector('code').textContent.toLowerCase();
                const desc = item.querySelector('.faker-desc').textContent.toLowerCase();
                
                if (code.includes(searchTerm) || desc.includes(searchTerm) || categoryTitle.includes(searchTerm)) {
                    item.style.display = '';
                    hasVisibleItems = true;
                } else {
                    item.style.display = 'none';
                }
            });

            // Hide category if no items match
            category.style.display = hasVisibleItems ? '' : 'none';
        });
    });

    // Focus style for search input
    searchInput?.addEventListener('focus', (e) => {
        e.target.style.borderColor = 'var(--primary-color)';
        e.target.style.outline = 'none';
    });

    searchInput?.addEventListener('blur', (e) => {
        e.target.style.borderColor = 'var(--border-color)';
    });
}

// Initialize search box
createSearchBox();

// Copy code button functionality
function addCopyButtons() {
    const codeBlocks = document.querySelectorAll('pre code');
    
    codeBlocks.forEach((block) => {
        const pre = block.parentElement;
        const button = document.createElement('button');
        button.className = 'copy-button';
        button.textContent = 'Copy';
        button.style.cssText = `
            position: absolute;
            top: 0.5rem;
            right: 0.5rem;
            padding: 0.5rem 1rem;
            background: var(--primary-color);
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.85rem;
            opacity: 0;
            transition: opacity 0.3s ease, background 0.3s ease;
        `;

        pre.style.position = 'relative';
        pre.appendChild(button);

        pre.addEventListener('mouseenter', () => {
            button.style.opacity = '1';
        });

        pre.addEventListener('mouseleave', () => {
            button.style.opacity = '0';
        });

        button.addEventListener('click', async () => {
            const code = block.textContent;
            try {
                await navigator.clipboard.writeText(code);
                button.textContent = 'Copied!';
                button.style.background = 'var(--success-color)';
                setTimeout(() => {
                    button.textContent = 'Copy';
                    button.style.background = 'var(--primary-color)';
                }, 2000);
            } catch (err) {
                button.textContent = 'Failed';
                setTimeout(() => {
                    button.textContent = 'Copy';
                }, 2000);
            }
        });
    });
}

// Add copy buttons after highlight.js processes the code
setTimeout(addCopyButtons, 100);

// Add active state to nav links based on scroll position
function updateActiveNavLink() {
    const sections = document.querySelectorAll('section[id]');
    const navLinks = document.querySelectorAll('.nav-menu a[href^="#"]');
    
    let current = '';
    
    sections.forEach(section => {
        const sectionTop = section.offsetTop;
        const sectionHeight = section.clientHeight;
        if (window.pageYOffset >= sectionTop - 100) {
            current = section.getAttribute('id');
        }
    });
    
    navLinks.forEach(link => {
        link.classList.remove('active');
        if (link.getAttribute('href') === `#${current}`) {
            link.classList.add('active');
        }
    });
}

window.addEventListener('scroll', updateActiveNavLink);

// Add keyboard shortcuts
document.addEventListener('keydown', (e) => {
    // Press '/' to focus search
    if (e.key === '/' && !e.ctrlKey && !e.metaKey) {
        const searchInput = document.getElementById('fakerSearch');
        if (searchInput && document.activeElement !== searchInput) {
            e.preventDefault();
            searchInput.focus();
        }
    }
    
    // Press 'Escape' to clear search
    if (e.key === 'Escape') {
        const searchInput = document.getElementById('fakerSearch');
        if (searchInput && document.activeElement === searchInput) {
            searchInput.value = '';
            searchInput.dispatchEvent(new Event('input'));
            searchInput.blur();
        }
    }
});

// Add tooltip for keyboard shortcuts
function addSearchTooltip() {
    const searchInput = document.getElementById('fakerSearch');
    if (!searchInput) return;

    const tooltip = document.createElement('div');
    tooltip.style.cssText = `
        position: absolute;
        bottom: -25px;
        right: 0;
        font-size: 0.75rem;
        color: var(--text-secondary);
        opacity: 0.7;
    `;
    tooltip.textContent = 'Press / to search, ESC to clear';
    
    searchInput.parentElement.style.position = 'relative';
    searchInput.parentElement.appendChild(tooltip);
}

setTimeout(addSearchTooltip, 200);

// Easter egg: Konami code
let konamiCode = [];
const konamiSequence = ['ArrowUp', 'ArrowUp', 'ArrowDown', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'ArrowLeft', 'ArrowRight', 'b', 'a'];

document.addEventListener('keydown', (e) => {
    konamiCode.push(e.key);
    konamiCode = konamiCode.slice(-10);
    
    if (konamiCode.join(',') === konamiSequence.join(',')) {
        document.body.style.animation = 'rainbow 2s linear infinite';
        setTimeout(() => {
            document.body.style.animation = '';
        }, 5000);
    }
});

// Add rainbow animation
const style = document.createElement('style');
style.textContent = `
    @keyframes rainbow {
        0% { filter: hue-rotate(0deg); }
        100% { filter: hue-rotate(360deg); }
    }
`;
document.head.appendChild(style);

console.log('%c🎲 Fake-rs Documentation', 'font-size: 20px; font-weight: bold; color: #ff6b35;');
console.log('%cGenerate fake data with style!', 'font-size: 14px; color: #1a659e;');
console.log('%cTip: Press / to search fakers', 'font-size: 12px; color: #a1a1aa;');
