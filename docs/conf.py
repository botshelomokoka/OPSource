import os
import sys
sys.path.insert(0, os.path.abspath('..'))

from datetime import datetime

# Project information
project = 'OPSource'
copyright = '2024, OPSource Team'
author = 'OPSource Team'

# Add any Sphinx extension module names here
extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon',
    'sphinx.ext.viewcode',
    'sphinx.ext.githubpages',
    'sphinx.ext.intersphinx',
    'sphinx_rtd_theme',
    'sphinx.ext.todo',
    'myst_parser',
    'sphinxcontrib.mermaid',
    'sphinx_copybutton',
    'sphinx_design',
    'sphinx_tabs.tabs',
]

# Add any paths that contain templates here
templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# Configure intersphinx mapping for cross-referencing between documentation sets
intersphinx_mapping = {
    'anya': ('anya/docs/_build/html', None),
    'dash33': ('dash33/docs/_build/html', None),
    'enterprise': ('enterprise/docs/_build/html', None),
    'python': ('https://docs.python.org/3', None),
    'sphinx': ('https://www.sphinx-doc.org/en/master/', None),
}

# The theme to use for HTML and HTML Help pages
html_theme = 'sphinx_rtd_theme'

# Theme options are theme-specific and customize the look and feel
html_theme_options = {
    'logo_only': False,
    'display_version': True,
    'prev_next_buttons_location': 'bottom',
    'style_nav_header_background': '#2c3e50',
    'style_external_links': True,
    'style_nav': True,
    'titles_only': False,
    'sticky_navigation': True,
    'navigation_depth': 4,
    'includehidden': True,
    'collapse_navigation': True,
    'body_max_width': None,
}

# These paths are either relative to html_static_path or fully qualified paths
html_logo = '_static/logo.png'
html_favicon = '_static/favicon.ico'

html_static_path = ['_static']
html_css_files = [
    'css/custom.css',
]
html_js_files = [
    'js/custom.js',
]

# Custom sidebar templates
html_sidebars = {
    '**': [
        'globaltoc.html',
        'relations.html',
        'sourcelink.html',
        'searchbox.html'
    ]
}

# Additional options
todo_include_todos = True
myst_enable_extensions = [
    'dollarmath',
    'amsmath',
    'deflist',
    'fieldlist',
    'html_admonition',
    'html_image',
    'colon_fence',
    'smartquotes',
    'replacements',
    'linkify',
    'substitution',
    'tasklist',
]

# Support both .md and .rst files
source_suffix = {
    '.rst': 'restructuredtext',
    '.md': 'markdown',
}

# Output options
html_show_sourcelink = True
html_copy_source = True
html_show_sphinx = True

# Cross-references between documentation sets
nitpicky = True
nitpick_ignore = []

# Copy button configuration
copybutton_prompt_text = r'>>> |\.\.\. |\$ |In \[\d*\]: | {2,5}\.\.\.: | {5,8}: '
copybutton_prompt_is_regexp = True

# GitHub repository
html_context = {
    'display_github': True,
    'github_user': 'botshelomokoka',
    'github_repo': 'opsource',
    'github_version': 'main',
    'conf_py_path': '/docs/',
}

# Version info
version = open('../VERSION').read().strip()
release = version
