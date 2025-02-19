// Interactive features for OPSource Documentation

// Copy code button
document.addEventListener('DOMContentLoaded', () => {
    // Add copy buttons to code blocks
    document.querySelectorAll('pre').forEach(block => {
        const button = document.createElement('button');
        button.className = 'copy-button';
        button.textContent = 'Copy';
        
        button.addEventListener('click', () => {
            const code = block.querySelector('code').textContent;
            navigator.clipboard.writeText(code);
            
            button.textContent = 'Copied!';
            setTimeout(() => {
                button.textContent = 'Copy';
            }, 2000);
        });
        
        block.style.position = 'relative';
        block.appendChild(button);
    });

    // Collapsible sections
    document.querySelectorAll('.collapsible').forEach(collapsible => {
        collapsible.addEventListener('click', () => {
            collapsible.classList.toggle('active');
        });
    });

    // Reading progress indicator
    const progressIndicator = document.createElement('div');
    progressIndicator.className = 'progress-indicator';
    document.body.appendChild(progressIndicator);

    window.addEventListener('scroll', () => {
        const windowHeight = document.documentElement.scrollHeight - document.documentElement.clientHeight;
        const progress = (window.scrollY / windowHeight) * 100;
        progressIndicator.style.width = `${progress}%`;
    });

    // Table of contents highlighting
    const headings = document.querySelectorAll('h2, h3');
    const tocLinks = document.querySelectorAll('.table-of-contents a');

    const observerOptions = {
        rootMargin: '0px',
        threshold: 1.0
    };

    const headingObserver = new IntersectionObserver(entries => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                tocLinks.forEach(link => {
                    if (link.getAttribute('href') === `#${entry.target.id}`) {
                        link.classList.add('active');
                    } else {
                        link.classList.remove('active');
                    }
                });
            }
        });
    }, observerOptions);

    headings.forEach(heading => headingObserver.observe(heading));

    // Dark mode toggle
    const darkModeToggle = document.createElement('button');
    darkModeToggle.className = 'dark-mode-toggle';
    darkModeToggle.innerHTML = '🌙';
    document.body.appendChild(darkModeToggle);

    darkModeToggle.addEventListener('click', () => {
        document.documentElement.setAttribute(
            'data-theme',
            document.documentElement.getAttribute('data-theme') === 'dark' ? 'light' : 'dark'
        );
        darkModeToggle.innerHTML = document.documentElement.getAttribute('data-theme') === 'dark' ? '☀️' : '🌙';
    });

    // Search highlighting
    const searchInput = document.querySelector('.search-input');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            const searchTerm = e.target.value.toLowerCase();
            const content = document.querySelector('.content');
            
            if (!searchTerm) {
                removeHighlights(content);
                return;
            }

            highlightText(content, searchTerm);
        });
    }
});

// Helper functions
function removeHighlights(element) {
    const highlights = element.querySelectorAll('mark');
    highlights.forEach(highlight => {
        const parent = highlight.parentNode;
        parent.replaceChild(document.createTextNode(highlight.textContent), highlight);
        parent.normalize();
    });
}

function highlightText(element, searchTerm) {
    removeHighlights(element);
    const walker = document.createTreeWalker(
        element,
        NodeFilter.SHOW_TEXT,
        null,
        false
    );

    const nodes = [];
    while (walker.nextNode()) nodes.push(walker.currentNode);

    nodes.forEach(node => {
        const text = node.textContent.toLowerCase();
        const index = text.indexOf(searchTerm);
        
        if (index >= 0) {
            const span = document.createElement('span');
            const before = document.createTextNode(node.textContent.substring(0, index));
            const highlight = document.createElement('mark');
            highlight.textContent = node.textContent.substring(index, index + searchTerm.length);
            const after = document.createTextNode(node.textContent.substring(index + searchTerm.length));
            
            span.appendChild(before);
            span.appendChild(highlight);
            span.appendChild(after);
            node.parentNode.replaceChild(span, node);
        }
    });
}

// Code syntax highlighting
document.addEventListener('DOMContentLoaded', () => {
    if (window.Prism) {
        Prism.highlightAll();
    }
});
