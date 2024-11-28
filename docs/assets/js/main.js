// Mobile Menu Toggle
document.addEventListener('DOMContentLoaded', () => {
    const mobileMenuButton = document.querySelector('.mobile-menu-button');
    const mobileMenu = document.querySelector('.mobile-menu');

    if (mobileMenuButton && mobileMenu) {
        mobileMenuButton.addEventListener('click', () => {
            mobileMenu.classList.toggle('hidden');
        });
    }

    // Initialize theme
    initializeTheme();
});

// Theme Management
function initializeTheme() {
    const themeToggle = document.querySelector('.theme-toggle');
    if (!themeToggle) return;

    // Check for saved theme preference or system preference
    const savedTheme = localStorage.getItem('theme');
    const systemDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    if (savedTheme === 'dark' || (!savedTheme && systemDarkMode)) {
        document.documentElement.classList.add('dark');
        updateThemeIcon(true);
    }

    // Theme toggle click handler
    themeToggle.addEventListener('click', () => {
        const isDark = document.documentElement.classList.toggle('dark');
        localStorage.setItem('theme', isDark ? 'dark' : 'light');
        updateThemeIcon(isDark);
    });

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
        if (!localStorage.getItem('theme')) {
            document.documentElement.classList.toggle('dark', e.matches);
            updateThemeIcon(e.matches);
        }
    });
}

function updateThemeIcon(isDark) {
    const themeToggle = document.querySelector('.theme-toggle');
    if (!themeToggle) return;

    themeToggle.innerHTML = isDark
        ? '<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>'
        : '<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>';
}

// Smooth Scrolling
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
            // Update URL hash without scrolling
            history.pushState(null, null, this.getAttribute('href'));
        }
    });
});

// Code Copy Functionality
document.querySelectorAll('pre').forEach(block => {
    const button = document.createElement('button');
    button.className = 'copy-button';
    button.textContent = 'Copy';
    
    block.style.position = 'relative';
    block.appendChild(button);
    
    button.addEventListener('click', async () => {
        const code = block.querySelector('code');
        if (code) {
            try {
                await navigator.clipboard.writeText(code.textContent);
                button.textContent = 'Copied!';
                button.classList.add('copied');
                setTimeout(() => {
                    button.textContent = 'Copy';
                    button.classList.remove('copied');
                }, 2000);
            } catch (err) {
                console.error('Failed to copy text: ', err);
                button.textContent = 'Error!';
                setTimeout(() => {
                    button.textContent = 'Copy';
                }, 2000);
            }
        }
    });
});

// Active Section Highlight
const observerOptions = {
    root: null,
    rootMargin: '0px',
    threshold: 0.5
};

const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const id = entry.target.getAttribute('id');
            document.querySelectorAll('.nav-link').forEach(link => {
                link.classList.remove('active');
                if (link.getAttribute('href') === `#${id}`) {
                    link.classList.add('active');
                }
            });
        }
    });
}, observerOptions);

document.querySelectorAll('section[id]').forEach(section => {
    observer.observe(section);
});

// Search Functionality
class DocSearch {
    constructor() {
        this.searchIndex = {};
        this.searchInput = document.querySelector('.search-input');
        this.searchResults = document.querySelector('.search-results');
        this.searchData = [];
        
        if (this.searchInput && this.searchResults) {
            this.initialize();
        }
    }

    async initialize() {
        try {
            // Load search data
            const response = await fetch('/docs/search-index.json');
            this.searchData = await response.json();
            this.buildSearchIndex();
            this.setupEventListeners();
        } catch (error) {
            console.error('Failed to initialize search:', error);
        }
    }

    buildSearchIndex() {
        this.searchData.forEach(item => {
            const words = this.tokenize(`${item.title} ${item.content}`);
            words.forEach(word => {
                if (!this.searchIndex[word]) {
                    this.searchIndex[word] = [];
                }
                if (!this.searchIndex[word].includes(item)) {
                    this.searchIndex[word].push(item);
                }
            });
        });
    }

    tokenize(text) {
        return text.toLowerCase()
            .replace(/[^\w\s]/g, '')
            .split(/\s+/)
            .filter(word => word.length > 2);
    }

    search(query) {
        const words = this.tokenize(query);
        const results = new Map();

        words.forEach(word => {
            const matches = this.searchIndex[word] || [];
            matches.forEach(match => {
                const score = results.get(match) || 0;
                results.set(match, score + 1);
            });
        });

        return Array.from(results.entries())
            .sort((a, b) => b[1] - a[1])
            .map(([item]) => item)
            .slice(0, 5);
    }

    setupEventListeners() {
        let debounceTimeout;

        this.searchInput.addEventListener('input', (e) => {
            clearTimeout(debounceTimeout);
            const query = e.target.value;

            debounceTimeout = setTimeout(() => {
                if (query.length > 2) {
                    const results = this.search(query);
                    this.displayResults(results);
                } else {
                    this.searchResults.classList.add('hidden');
                }
            }, 300);
        });

        // Close search results when clicking outside
        document.addEventListener('click', (e) => {
            if (!this.searchInput.contains(e.target) && !this.searchResults.contains(e.target)) {
                this.searchResults.classList.add('hidden');
            }
        });
    }

    displayResults(results) {
        if (results.length === 0) {
            this.searchResults.innerHTML = '<div class="p-4 text-gray-600">No results found</div>';
        } else {
            this.searchResults.innerHTML = results.map(result => `
                <a href="${result.url}" class="block p-4 hover:bg-gray-50 dark:hover:bg-gray-800">
                    <h4 class="font-semibold">${result.title}</h4>
                    <p class="text-sm text-gray-600 dark:text-gray-400">${result.excerpt}</p>
                </a>
            `).join('');
        }
        this.searchResults.classList.remove('hidden');
    }
}

// Initialize search when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new DocSearch();
});