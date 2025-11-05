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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item affix "><li class="part-title">User Guide</li><li class="chapter-item "><a href="installation.html"><strong aria-hidden="true">1.</strong> Installation</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="installation.html"><strong aria-hidden="true">1.1.</strong> Prerequisites</a></li><li class="chapter-item "><a href="installation.html"><strong aria-hidden="true">1.2.</strong> Installing Porters</a></li><li class="chapter-item "><a href="installation.html"><strong aria-hidden="true">1.3.</strong> Building from Source</a></li></ol></li><li class="chapter-item "><a href="getting-started.html"><strong aria-hidden="true">2.</strong> Getting Started</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="getting-started.html"><strong aria-hidden="true">2.1.</strong> Creating a New Project</a></li><li class="chapter-item "><a href="getting-started.html"><strong aria-hidden="true">2.2.</strong> Initializing Existing Project</a></li><li class="chapter-item "><a href="getting-started.html"><strong aria-hidden="true">2.3.</strong> Project Structure</a></li></ol></li><li class="chapter-item "><a href="dependencies.html"><strong aria-hidden="true">3.</strong> Dependency Management</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="dependencies.html"><strong aria-hidden="true">3.1.</strong> Adding Dependencies</a></li><li class="chapter-item "><a href="dependencies.html"><strong aria-hidden="true">3.2.</strong> Global vs Local Dependencies</a></li><li class="chapter-item "><a href="dependencies.html"><strong aria-hidden="true">3.3.</strong> Git Dependencies</a></li><li class="chapter-item "><a href="dependencies.html"><strong aria-hidden="true">3.4.</strong> Syncing Dependencies</a></li><li class="chapter-item "><a href="dependencies.html"><strong aria-hidden="true">3.5.</strong> Lock File</a></li></ol></li><li class="chapter-item "><a href="building.html"><strong aria-hidden="true">4.</strong> Building Projects</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="building.html"><strong aria-hidden="true">4.1.</strong> Build Systems</a></li><li class="chapter-item "><a href="building.html"><strong aria-hidden="true">4.2.</strong> Build Configuration</a></li><li class="chapter-item "><a href="building.html"><strong aria-hidden="true">4.3.</strong> Environment Variables</a></li></ol></li><li class="chapter-item "><a href="execute.html"><strong aria-hidden="true">5.</strong> Execute - Single File Execution</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="execute.html"><strong aria-hidden="true">5.1.</strong> Overview</a></li><li class="chapter-item "><a href="execute.html"><strong aria-hidden="true">5.2.</strong> Basic Usage</a></li><li class="chapter-item "><a href="execute.html"><strong aria-hidden="true">5.3.</strong> Supported File Extensions</a></li><li class="chapter-item "><a href="execute.html"><strong aria-hidden="true">5.4.</strong> Examples</a></li><li class="chapter-item "><a href="execute.html"><strong aria-hidden="true">5.5.</strong> Optional Configuration</a></li></ol></li><li class="chapter-item "><a href="publishing.html"><strong aria-hidden="true">6.</strong> Publishing</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="publishing.html"><strong aria-hidden="true">6.1.</strong> GitHub Releases</a></li><li class="chapter-item "><a href="publishing.html"><strong aria-hidden="true">6.2.</strong> Version Management</a></li></ol></li><li class="chapter-item "><a href="extensions.html"><strong aria-hidden="true">7.</strong> Extensions</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="extensions.html"><strong aria-hidden="true">7.1.</strong> What are Extensions</a></li><li class="chapter-item "><a href="extensions.html"><strong aria-hidden="true">7.2.</strong> Installing Extensions</a></li><li class="chapter-item "><a href="extensions.html"><strong aria-hidden="true">7.3.</strong> Creating Extensions</a></li><li class="chapter-item "><a href="extensions.html"><strong aria-hidden="true">7.4.</strong> Publishing Extensions</a></li></ol></li><li class="chapter-item "><li class="part-title">Reference</li><li class="chapter-item "><a href="configuration.html"><strong aria-hidden="true">8.</strong> Configuration</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="configuration.html"><strong aria-hidden="true">8.1.</strong> porters.toml</a></li><li class="chapter-item "><a href="configuration.html"><strong aria-hidden="true">8.2.</strong> Global Configuration</a></li><li class="chapter-item "><a href="configuration.html"><strong aria-hidden="true">8.3.</strong> Build Configuration</a></li></ol></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.</strong> Command Reference</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.1.</strong> porters init</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.2.</strong> porters create</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.3.</strong> porters add</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.4.</strong> porters install</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.5.</strong> porters sync</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.6.</strong> porters lock</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.7.</strong> porters build</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.8.</strong> porters run</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.9.</strong> porters execute</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.10.</strong> porters publish</a></li><li class="chapter-item "><a href="commands.html"><strong aria-hidden="true">9.11.</strong> porters self-update</a></li></ol></li><li class="chapter-item "><a href="troubleshooting.html"><strong aria-hidden="true">10.</strong> Troubleshooting</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="troubleshooting.html"><strong aria-hidden="true">10.1.</strong> Common Issues</a></li><li class="chapter-item "><a href="troubleshooting.html"><strong aria-hidden="true">10.2.</strong> Build Errors</a></li><li class="chapter-item "><a href="troubleshooting.html"><strong aria-hidden="true">10.3.</strong> Dependency Resolution</a></li><li class="chapter-item "><a href="troubleshooting.html"><strong aria-hidden="true">10.4.</strong> Platform-Specific Issues</a></li></ol></li><li class="chapter-item "><li class="part-title">Development</li><li class="chapter-item "><a href="contributing.html"><strong aria-hidden="true">11.</strong> Contributing</a></li><li class="chapter-item "><a href="architecture.html"><strong aria-hidden="true">12.</strong> Architecture</a></li><li class="chapter-item "><a href="development.html"><strong aria-hidden="true">13.</strong> Development Guide</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
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
