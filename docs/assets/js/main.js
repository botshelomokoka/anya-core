// Mobile menu functionality
document.addEventListener('DOMContentLoaded', () => {
    const mobileMenuButton = document.querySelector('.mobile-menu-button');
    const mobileMenu = document.querySelector('nav ul');

    if (mobileMenuButton && mobileMenu) {
        mobileMenuButton.addEventListener('click', () => {
            mobileMenu.classList.toggle('show');
        });
    }

    // Add smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });

    // Add active state to navigation links
    const navLinks = document.querySelectorAll('nav a');
    const currentPath = window.location.pathname;

    navLinks.forEach(link => {
        if (link.getAttribute('href') === currentPath) {
            link.classList.add('active');
        }
    });

    // Initialize search functionality
    const searchInput = document.querySelector('.search-input');
    const searchResults = document.querySelector('.search-results');

    if (searchInput && searchResults) {
        let searchTimeout;

        searchInput.addEventListener('input', (e) => {
            clearTimeout(searchTimeout);
            const query = e.target.value.trim();

            if (query.length < 2) {
                searchResults.classList.add('hidden');
                return;
            }

            searchTimeout = setTimeout(() => {
                performSearch(query);
            }, 300);
        });

        // Close search results when clicking outside
        document.addEventListener('click', (e) => {
            if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
                searchResults.classList.add('hidden');
            }
        });
    }
});

// Search functionality
async function performSearch(query) {
    try {
        const response = await fetch('/search-index.json');
        const searchIndex = await response.json();
        const results = searchIndex.filter(item => 
            item.title.toLowerCase().includes(query.toLowerCase()) ||
            item.content.toLowerCase().includes(query.toLowerCase())
        ).slice(0, 5);

        displaySearchResults(results);
    } catch (error) {
        console.error('Error performing search:', error);
    }
}

function displaySearchResults(results) {
    const searchResults = document.querySelector('.search-results');
    if (!searchResults) return;

    if (results.length === 0) {
        searchResults.innerHTML = '<div class="p-4 text-gray-500">No results found</div>';
    } else {
        searchResults.innerHTML = results.map(result => `
            <a href="${result.url}" class="block p-4 hover:bg-gray-100">
                <div class="font-medium text-gray-900">${result.title}</div>
                <div class="text-sm text-gray-500">${result.excerpt}</div>
            </a>
        `).join('');
    }

    searchResults.classList.remove('hidden');
}

// Feature card animations
const featureCards = document.querySelectorAll('.feature-card');
if (featureCards.length > 0) {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('animate-in');
                observer.unobserve(entry.target);
            }
        });
    }, observerOptions);

    featureCards.forEach(card => {
        observer.observe(card);
    });
}

// Add copy button to code blocks
document.querySelectorAll('pre code').forEach((codeBlock) => {
    const container = codeBlock.parentNode;
    const copyButton = document.createElement('button');
    copyButton.className = 'copy-button';
    copyButton.textContent = 'Copy';
    
    copyButton.addEventListener('click', () => {
        navigator.clipboard.writeText(codeBlock.textContent).then(() => {
            copyButton.textContent = 'Copied!';
            setTimeout(() => {
                copyButton.textContent = 'Copy';
            }, 2000);
        }).catch(err => {
            console.error('Failed to copy code:', err);
        });
    });

    container.appendChild(copyButton);
});