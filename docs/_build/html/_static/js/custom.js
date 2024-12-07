// Custom JavaScript for OPSource documentation

document.addEventListener('DOMContentLoaded', function() {
    // Progress bar
    const progressBar = document.createElement('div');
    progressBar.className = 'progress-bar';
    document.body.appendChild(progressBar);

    window.addEventListener('scroll', function() {
        const winScroll = document.body.scrollTop || document.documentElement.scrollTop;
        const height = document.documentElement.scrollHeight - document.documentElement.clientHeight;
        const scrolled = (winScroll / height) * 100;
        progressBar.style.width = scrolled + '%';
    });

    // Table of contents highlighting
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            const id = entry.target.getAttribute('id');
            if (entry.intersectionRatio > 0) {
                document.querySelector(`nav a[href="#${id}"]`)?.classList.add('active');
            } else {
                document.querySelector(`nav a[href="#${id}"]`)?.classList.remove('active');
            }
        });
    });

    // Track all section headings
    document.querySelectorAll('h2[id], h3[id], h4[id]').forEach((section) => {
        observer.observe(section);
    });

    // Add copy buttons to code blocks
    document.querySelectorAll('pre').forEach((block) => {
        const button = document.createElement('button');
        button.className = 'copybtn';
        button.innerHTML = 'Copy';
        
        button.addEventListener('click', () => {
            const code = block.querySelector('code').textContent;
            navigator.clipboard.writeText(code).then(() => {
                button.innerHTML = 'Copied!';
                setTimeout(() => {
                    button.innerHTML = 'Copy';
                }, 2000);
            });
        });
        
        block.appendChild(button);
    });

    // Version selector
    const versionSelector = document.querySelector('.version-selector select');
    if (versionSelector) {
        versionSelector.addEventListener('change', (e) => {
            window.location.href = e.target.value;
        });
    }

    // Mobile navigation
    const menuButton = document.querySelector('.wy-nav-top button');
    if (menuButton) {
        menuButton.addEventListener('click', () => {
            document.querySelector('.wy-nav-side').classList.toggle('shift');
        });
    }

    // Search highlighting
    const searchInput = document.querySelector('.wy-side-nav-search input[type="text"]');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            const searchTerm = e.target.value.toLowerCase();
            if (!searchTerm) {
                removeHighlights();
                return;
            }
            highlightText(searchTerm);
        });
    }
});

// Helper functions
function removeHighlights() {
    document.querySelectorAll('mark').forEach(mark => {
        const parent = mark.parentNode;
        parent.replaceChild(document.createTextNode(mark.textContent), mark);
        parent.normalize();
    });
}

function highlightText(searchTerm) {
    removeHighlights();
    const walker = document.createTreeWalker(
        document.body,
        NodeFilter.SHOW_TEXT,
        {
            acceptNode: function(node) {
                return node.parentElement.tagName !== 'SCRIPT' &&
                       node.parentElement.tagName !== 'STYLE' &&
                       !node.parentElement.classList.contains('copybtn')
                    ? NodeFilter.FILTER_ACCEPT
                    : NodeFilter.FILTER_REJECT;
            }
        }
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

// Dark mode toggle
const darkModeToggle = document.createElement('button');
darkModeToggle.className = 'dark-mode-toggle';
darkModeToggle.innerHTML = '🌙';
document.body.appendChild(darkModeToggle);

darkModeToggle.addEventListener('click', () => {
    document.documentElement.classList.toggle('dark-mode');
    darkModeToggle.innerHTML = document.documentElement.classList.contains('dark-mode') ? '☀️' : '🌙';
    localStorage.setItem('dark-mode', document.documentElement.classList.contains('dark-mode'));
});

// Check for saved dark mode preference
if (localStorage.getItem('dark-mode') === 'true') {
    document.documentElement.classList.add('dark-mode');
    darkModeToggle.innerHTML = '☀️';
}
