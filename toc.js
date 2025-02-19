// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="quick-start.html"><strong aria-hidden="true">1.</strong> Quick Start Guide</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item expanded "><a href="configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture</li><li class="chapter-item expanded "><a href="architecture/overview.html"><strong aria-hidden="true">4.</strong> System Overview</a></li><li class="chapter-item expanded "><a href="architecture/components.html"><strong aria-hidden="true">5.</strong> Components</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="architecture/components/anya.html"><strong aria-hidden="true">5.1.</strong> Anya</a></li><li class="chapter-item "><a href="architecture/components/dash33.html"><strong aria-hidden="true">5.2.</strong> Dash33</a></li><li class="chapter-item "><a href="architecture/components/enterprise.html"><strong aria-hidden="true">5.3.</strong> Enterprise</a></li><li class="chapter-item "><a href="architecture/components/mobile.html"><strong aria-hidden="true">5.4.</strong> Mobile</a></li></ol></li><li class="chapter-item expanded "><a href="architecture/data-flow.html"><strong aria-hidden="true">6.</strong> Data Flow</a></li><li class="chapter-item expanded "><a href="architecture/security.html"><strong aria-hidden="true">7.</strong> Security</a></li><li class="chapter-item expanded affix "><li class="part-title">Development</li><li class="chapter-item expanded "><a href="development/standards.html"><strong aria-hidden="true">8.</strong> Standards</a></li><li class="chapter-item expanded "><a href="development/best-practices.html"><strong aria-hidden="true">9.</strong> Best Practices</a></li><li class="chapter-item expanded "><a href="development/code-style.html"><strong aria-hidden="true">10.</strong> Code Style</a></li><li class="chapter-item expanded "><a href="development/testing.html"><strong aria-hidden="true">11.</strong> Testing</a></li><li class="chapter-item expanded "><a href="development/documentation.html"><strong aria-hidden="true">12.</strong> Documentation</a></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/overview.html"><strong aria-hidden="true">13.</strong> Overview</a></li><li class="chapter-item expanded "><a href="api/authentication.html"><strong aria-hidden="true">14.</strong> Authentication</a></li><li class="chapter-item expanded "><a href="api/endpoints.html"><strong aria-hidden="true">15.</strong> Endpoints</a></li><li class="chapter-item expanded "><a href="api/errors.html"><strong aria-hidden="true">16.</strong> Error Handling</a></li><li class="chapter-item expanded "><a href="api/rate-limiting.html"><strong aria-hidden="true">17.</strong> Rate Limiting</a></li><li class="chapter-item expanded affix "><li class="part-title">Deployment</li><li class="chapter-item expanded "><a href="deployment/prerequisites.html"><strong aria-hidden="true">18.</strong> Prerequisites</a></li><li class="chapter-item expanded "><a href="deployment/environment.html"><strong aria-hidden="true">19.</strong> Environment Setup</a></li><li class="chapter-item expanded "><a href="deployment/configuration.html"><strong aria-hidden="true">20.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="deployment/monitoring.html"><strong aria-hidden="true">21.</strong> Monitoring</a></li><li class="chapter-item expanded "><a href="deployment/maintenance.html"><strong aria-hidden="true">22.</strong> Maintenance</a></li><li class="chapter-item expanded affix "><li class="part-title">Troubleshooting</li><li class="chapter-item expanded "><a href="troubleshooting/common-issues.html"><strong aria-hidden="true">23.</strong> Common Issues</a></li><li class="chapter-item expanded "><a href="troubleshooting/faq.html"><strong aria-hidden="true">24.</strong> FAQ</a></li><li class="chapter-item expanded "><a href="troubleshooting/support.html"><strong aria-hidden="true">25.</strong> Support</a></li><li class="chapter-item expanded affix "><li class="part-title">Contributing</li><li class="chapter-item expanded "><a href="contributing/guidelines.html"><strong aria-hidden="true">26.</strong> Guidelines</a></li><li class="chapter-item expanded "><a href="contributing/code-of-conduct.html"><strong aria-hidden="true">27.</strong> Code of Conduct</a></li><li class="chapter-item expanded "><a href="contributing/development-process.html"><strong aria-hidden="true">28.</strong> Development Process</a></li><li class="chapter-item expanded "><a href="contributing/pull-requests.html"><strong aria-hidden="true">29.</strong> Pull Requests</a></li><li class="chapter-item expanded affix "><li class="part-title">Release Notes</li><li class="chapter-item expanded "><a href="release-notes/changelog.html"><strong aria-hidden="true">30.</strong> Changelog</a></li><li class="chapter-item expanded "><a href="release-notes/roadmap.html"><strong aria-hidden="true">31.</strong> Roadmap</a></li><li class="chapter-item expanded "><a href="release-notes/migration.html"><strong aria-hidden="true">32.</strong> Migration Guide</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
